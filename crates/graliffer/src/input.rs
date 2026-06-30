use action::{Action, AnyAction, State};
use crossterm::event::{KeyEvent, MouseEvent};
use log::debug;
use ratatui::{
    layout::Position,
    style::Stylize,
    text::{Span, ToSpan},
};

use crate::{
    ConsoleAction, GridAction, PaneId,
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
    context_predicate: KeyContextPredicate,
    action: AnyAction,
}

impl KeymapEntry {
    pub fn new(
        keystroke: Keystroke,
        context_predicate: KeyContextPredicate,
        action: AnyAction,
    ) -> Self {
        Self {
            keystroke,
            context_predicate,
            action,
        }
    }
}

#[derive(Debug, Default)]
pub struct Keymap(Vec<KeymapEntry>);

impl Keymap {
    pub fn new() -> Self {
        let mut map = Self::default();

        map.push(
            Keystroke::try_from("up").unwrap(),
            KeyContextPredicate {
                focus: Some(PaneId::Grid.into()),
                input_mode: Some(InputMode::Command),
            },
            GridAction::CursorUp,
        );

        map.push(
            Keystroke::try_from("down").unwrap(),
            KeyContextPredicate {
                focus: Some(PaneId::Grid.into()),
                input_mode: Some(InputMode::Command),
            },
            GridAction::CursorDown,
        );

        map.push(
            Keystroke::try_from("q").unwrap(),
            KeyContextPredicate::default(),
            AppAction::Quit,
        );

        map.push(
            Keystroke::try_from("i").unwrap(),
            KeyContextPredicate {
                input_mode: Some(InputMode::Command),
                ..Default::default()
            },
            AppAction::InsertMode,
        );

        map.push(
            Keystroke::try_from("escape").unwrap(),
            KeyContextPredicate {
                input_mode: Some(InputMode::Insert),
                ..Default::default()
            },
            AppAction::CommandMode,
        );

        map.push(
            Keystroke::try_from("ctrl-c").unwrap(),
            KeyContextPredicate::default(),
            AppAction::Quit,
        );

        map.push(
            Keystroke::try_from("ctrl-a").unwrap(),
            KeyContextPredicate::default(),
            AppAction::ToggleAbout,
        );

        map.push(
            Keystroke::try_from("ctrl-p").unwrap(),
            KeyContextPredicate::default(),
            AppAction::ToggleCommandPicker,
        );

        map.push(
            Keystroke::try_from("shift-c").unwrap(),
            KeyContextPredicate::default(),
            ConsoleAction::Clear,
        );

        map.push(
            Keystroke::try_from("ctrl-f").unwrap(),
            KeyContextPredicate::default(),
            AppAction::FocusStack,
        );

        map
    }

    pub fn push(
        &mut self,
        keystroke: Keystroke,
        context_predicate: KeyContextPredicate,
        action: impl Action,
    ) {
        self.0.push(KeymapEntry {
            keystroke,
            context_predicate,
            action: AnyAction::new(action),
        });
    }

    pub fn find(&self, keystroke: Keystroke, key_context: KeyContext) -> Option<AnyAction> {
        self.0
            .iter()
            .find(|item| item.keystroke == keystroke && item.context_predicate.matches(key_context))
            .and_then(|item| Some(item.action.clone()))
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
    pub fn handle_key_events(&mut self, key_event: KeyEvent, key_context: KeyContext) {
        // debug!("{:?}", key_event);

        if let Result::Ok(keystroke) = Keystroke::try_from(key_event) {
            debug!("{:?}", keystroke);
            if let Some(action) = self.keymap.find(keystroke, key_context) {
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
