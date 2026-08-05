use act::State;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Position;

use crate::{AppState, Context, GridView, Key, Keystroke, PickerView, View};

impl AppState {
    pub fn handle_key_events(&mut self, key_event: KeyEvent, app_context: Context) {
        if let Result::Ok(keystroke) = Keystroke::try_from(key_event) {
            if let Some(action) = self.keymap.find(app_context, keystroke) {
                // debug!("{:?}", action);
                let _ = self.act(action);
            } else if let Key::Char(char) = keystroke.key {
                let action = match self.focused().to_string().as_str() {
                    "Grid" => GridView::input_sink_action(char.to_string()),
                    "Picker" => PickerView::input_sink_action(char.to_string()),
                    _ => None,
                };

                let _ = self.act(action.unwrap());
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
