use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode, MouseEvent};
use ratatui::layout::Position;

use crate::app::App;

#[derive(Debug)]
struct Keybind {
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
}

#[derive(Debug, Default)]
struct Keymap(Vec<(Keybind, String)>);

impl Keymap {
    pub fn new() -> Self {
        let mut map = Self::default();

        map.push(Keybind::from_key(KeyCode::Char('q')), "prout".to_string());

        map
    }

    pub fn push(&mut self, keybind: Keybind, action: String) {
        self.0.push((keybind, action));
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputMode {
    Insert,
    Command,
}

impl App {
    pub fn handle_key_events(&mut self, key_event: KeyEvent) {
        // match self.input_mode {
        //     InputMode::Insert => {}
        // }

        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.quit(),
            KeyCode::Char('c') | KeyCode::Char('C')
                if key_event.modifiers == KeyModifiers::CONTROL =>
            {
                self.quit()
            }
            // KeyCode::Right | KeyCode::Char('j') => app.increment_counter(),
            // KeyCode::Left | KeyCode::Char('k') => app.decrement_counter(),
            _ => {}
        };

        self.grid_state.handle_key_event(key_event);
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
