use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::{Deref, DerefMut},
};

use action::{Action, AnyAction, State};
use crossterm::event::{KeyEvent, MouseEvent};
use log::debug;
use ratatui::{
    layout::Position,
    style::Stylize,
    text::{Span, ToSpan},
};

use crate::{
    ConsoleAction, Context, GridAction, PaneId,
    app::{
        AppAction::{self, FocusStack},
        AppState, FocusId,
    },
};

mod context;
pub use context::*;

mod keystroke;
pub use keystroke::*;

#[derive(Debug)]
pub struct KeymapEntry {
    keystroke: Keystroke,
    // context_predicate: KeyContextPredicate,
    action: AnyAction,
}

impl KeymapEntry {
    pub fn new(
        keystroke: Keystroke,
        // context_predicate: KeyContextPredicate,
        action: impl Action,
    ) -> Self {
        Self {
            keystroke,
            // context_predicate,
            action: AnyAction::new(action),
        }
    }
}

#[derive(Debug, Default)]
struct KeymapEntries(Vec<KeymapEntry>);

impl KeymapEntries {
    fn find_keystroke(&self, keystroke: Keystroke) -> Option<AnyAction> {
        self.iter()
            .find(|entry| entry.keystroke == keystroke)
            .and_then(|entry| Some(entry.action.clone()))
    }
}

impl From<Vec<KeymapEntry>> for KeymapEntries {
    fn from(value: Vec<KeymapEntry>) -> Self {
        Self(value)
    }
}

impl Deref for KeymapEntries {
    type Target = Vec<KeymapEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KeymapEntries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Default)]
pub struct Keymap(HashMap<KeyContext, KeymapEntries>);

impl Keymap {
    pub fn new() -> Self {
        let mut map = Self::default();

        let mut grid_insert = KeyContext::from(vec!["Grid", "insert"]);
        let mut grid_command = KeyContext::from(vec!["Grid", "command"]);

        map.insert(
            &grid_insert,
            KeymapEntry::new(Keystroke::try_from("up").unwrap(), GridAction::CursorUp),
        );

        map.insert(
            &grid_insert,
            KeymapEntry::new(Keystroke::try_from("down").unwrap(), GridAction::CursorDown),
        );

        map.insert(
            &KeyContext::empty(),
            KeymapEntry::new(Keystroke::try_from("q").unwrap(), AppAction::Quit),
        );

        map.insert(
            &grid_insert,
            KeymapEntry::new(
                Keystroke::try_from("escape").unwrap(),
                AppAction::CommandMode,
            ),
        );

        map.insert(
            &grid_command,
            KeymapEntry::new(Keystroke::try_from("i").unwrap(), AppAction::InsertMode),
        );
        // map.push(
        //     Keystroke::try_from("i").unwrap(),
        //     KeyContextPredicate {
        //         input_mode: Some(InputMode::Command),
        //         ..Default::default()
        //     },
        //     AppAction::InsertMode,
        // );

        // map.push(
        //     Keystroke::try_from("escape").unwrap(),
        //     KeyContextPredicate {
        //         input_mode: Some(InputMode::Insert),
        //         ..Default::default()
        //     },
        //     AppAction::CommandMode,
        // );

        // map.push(
        //     Keystroke::try_from("ctrl-c").unwrap(),
        //     KeyContextPredicate::default(),
        //     AppAction::Quit,
        // );

        // map.push(
        //     Keystroke::try_from("ctrl-a").unwrap(),
        //     KeyContextPredicate::default(),
        //     AppAction::ToggleAbout,
        // );

        // map.push(
        //     Keystroke::try_from("ctrl-p").unwrap(),
        //     KeyContextPredicate::default(),
        //     AppAction::ToggleCommandPicker,
        // );

        // map.push(
        //     Keystroke::try_from("shift-c").unwrap(),
        //     KeyContextPredicate::default(),
        //     ConsoleAction::Clear,
        // );

        // map.push(
        //     Keystroke::try_from("ctrl-f").unwrap(),
        //     KeyContextPredicate::default(),
        //     AppAction::FocusStack,
        // );

        map
    }

    pub fn insert(&mut self, context: &KeyContext, keymap_entry: KeymapEntry) {
        if let Some(for_context) = self.0.get_mut(context) {
            for_context.push(keymap_entry);
        } else {
            self.0.insert(context.clone(), vec![keymap_entry].into());
        }
    }

    // pub fn push(
    //     &mut self,
    //     keystroke: Keystroke,
    //     context_predicate: KeyContextPredicate,
    //     action: impl Action,
    // ) {
    //     self.0.push(KeymapEntry {
    //         keystroke,
    //         context_predicate,
    //         action: AnyAction::new(action),
    //     });
    // }

    pub fn find(&self, app_context: Context, keystroke: Keystroke) -> Option<AnyAction> {
        // todo: make specific context predicate have a higher priorities
        for (_, entries) in self
            .0
            .iter()
            .filter(|(key_context, _)| app_context.matches_key_context(*key_context))
        {
            return entries.find_keystroke(keystroke);
        }

        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputMode {
    Insert,
    Command,
}

impl InputMode {
    pub fn formated<'a>(&self) -> Span<'a> {
        use InputMode::*;
        match self {
            Command => "COMMAND".red(),
            Insert => "INSERT".to_span(),
        }
    }
}

impl AppState {
    pub fn handle_key_events(&mut self, key_event: KeyEvent, context: Context) {
        if let Result::Ok(keystroke) = Keystroke::try_from(key_event) {
            if let Some(action) = self.keymap.find(context, keystroke) {
                debug!("{:?}", action);
                self.act(&action.try_into().unwrap());
            }
        }
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        if let Some(console_layouts) = self.console_state.layouts() {
            let contained = console_layouts
                .viewport_area()
                .union(console_layouts.vertical_scrollbar_area())
                .contains(Position {
                    x: mouse_event.column,
                    y: mouse_event.row,
                });

            if contained {
                self.console_state.handle_mouse_event(mouse_event);
            }
        }

        if let Some(grid_layout) = self.grid_state.layout() {
            let contained = grid_layout.contains(Position {
                x: mouse_event.column,
                y: mouse_event.row,
            });

            if contained {
                self.grid_state.handle_mouse_event(mouse_event);
            }
        }
    }
}
