use action::{Action, AnyAction, State};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode, MouseEvent};
use ratatui::{
    layout::Position,
    style::Stylize,
    text::{Span, ToSpan},
};

use crate::{
    app::{
        App,
        AppAction::{self, FocusStack},
    },
    ui::{ConsoleAction, FocusedPane::Console},
};

#[derive(Debug)]
pub struct Keybind {
    modifiers: KeyModifiers,
    key: KeyCode,
}

impl Keybind {
    pub fn from_key(key: KeyCode) -> Self {
        Self {
            modifiers: KeyModifiers::NONE,
            key,
        }
    }

    pub fn matches(&self, key_event: KeyEvent) -> bool {
        self.key == key_event.code && self.modifiers == key_event.modifiers
    }
}

#[derive(Debug, Default)]
pub struct Keymap(Vec<(Keybind, AnyAction)>);

impl Keymap {
    pub fn new() -> Self {
        let mut map = Self::default();

        map.push(Keybind::from_key(KeyCode::Char('q')), AppAction::Quit);
        map.push(
            Keybind {
                key: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            },
            AppAction::Quit,
        );

        map.push(
            Keybind {
                key: KeyCode::Char('a'),
                modifiers: KeyModifiers::CONTROL,
            },
            AppAction::About,
        );
        map.push(
            Keybind {
                key: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            },
            ConsoleAction::Clear,
        );

        map.push(
            Keybind {
                key: KeyCode::Char('f'),
                modifiers: KeyModifiers::CONTROL,
            },
            AppAction::FocusStack,
        );

        map
    }

    pub fn push(&mut self, keybind: Keybind, action: impl Action) {
        self.0.push((keybind, AnyAction::new(action)));
    }

    pub fn find(&self, key_event: KeyEvent) -> Option<AnyAction> {
        self.0
            .iter()
            .find(|item| item.0.matches(key_event))
            .and_then(|item| Some(item.1.clone()))
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

impl App {
    pub fn handle_key_events(&mut self, key_event: KeyEvent) {
        if let Some(action) = self.keymap.find(key_event) {
            self.act(&action);
        }

        // self.grid_state.handle_key_event(key_event);
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        // TODO: this is a temporary solution to filter event targets based on position
        if let Some(console_layouts) = self.console_state.layouts() {
            // if mouse_event console_layouts.viewport_area()

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
