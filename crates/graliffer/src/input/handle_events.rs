use action::State;
use crossterm::event::{KeyEvent, MouseEvent};
use log::debug;
use ratatui::layout::Position;

use crate::{AppState, Context, GridView, Key, Keystroke, View};

impl AppState {
    pub fn handle_key_events(&mut self, key_event: KeyEvent, app_context: Context) {
        if let Result::Ok(keystroke) = Keystroke::try_from(key_event) {
            if let Some(action) = self.keymap.find(app_context, keystroke) {
                // debug!("{:?}", action);
                let _ = self.act(&action.try_into().unwrap());
            }

            if self.is_focused(GridView::view_id())
                && let Key::Char(char) = keystroke.key
            {
                let action = GridView::input_sink_action(char.to_string());
                let _ = self.act(&action.unwrap().try_into().unwrap());
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
