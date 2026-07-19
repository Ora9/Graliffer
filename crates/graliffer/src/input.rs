use std::{default, fmt::Display};

use action::State;
use crossterm::event::{KeyEvent, MouseEvent};
use log::debug;
use ratatui::{
    layout::Position,
    style::Stylize,
    text::{Span, ToSpan},
};

use crate::{Context, app::AppState};

mod key_context;
pub use key_context::*;

mod key_context_predicate;
pub use key_context_predicate::*;

mod keystroke;
pub use keystroke::*;

mod key;
pub use key::*;

mod modifiers;
pub use modifiers::*;

mod keymap;
pub use keymap::*;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum InputMode {
    #[default]
    Insert,
    Command,
}

impl Display for InputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Insert => f.write_str("insert"),
            Self::Command => f.write_str("command"),
        }
    }
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
    pub fn handle_key_events(&mut self, key_event: KeyEvent, app_context: Context) {
        if let Result::Ok(keystroke) = Keystroke::try_from(key_event) {
            if let Some(action) = self.keymap.find(app_context, keystroke) {
                debug!("{:?}", action);
                let _ = self.act(&action.try_into().unwrap());
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
