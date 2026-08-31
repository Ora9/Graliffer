use act::{Revert, State};
use crossterm::event::{KeyEvent, MouseEvent};
use log::debug;
use ratatui::layout::Position;

use crate::{AppState, Context, GridView, Key, Keystroke, Picker, PickerView, View};

impl AppState {
    pub fn handle_key_events(&mut self, key_event: KeyEvent, app_context: Context) {
        let revert = if let Ok(keystroke) = Keystroke::try_from(key_event) {
            if let Some(action) = self.keymap.find(app_context, keystroke) {
                match self.act(action) {
                    Ok(revert) => revert,
                }
            } else if let Key::Char(char) = keystroke.key {
                self.handle_input_sink(char.to_string())
            } else {
                Revert::None
            }
        } else {
            Revert::None
        };

        debug!("{:?}", revert);
    }

    pub fn handle_input_sink(&mut self, input: String) -> Revert {
        let action = match self.focused().to_string().as_str() {
            "Grid" => GridView::input_sink_action(input),
            "Picker" => PickerView::input_sink_action(input),
            _ => None,
        };

        if let Some(action) = action {
            match self.act(action) {
                Ok(revert) => revert,
            }
        } else {
            Revert::None
        }
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        let mouse_pos = Position {
            x: mouse_event.column,
            y: mouse_event.row,
        };

        if let Some(console_layouts) = self.console_state.layouts() {
            let contained = console_layouts
                .viewport_area()
                .union(console_layouts.vertical_scrollbar_area())
                .contains(mouse_pos);

            if contained {
                self.console_state.handle_mouse_event(mouse_event);
            }
        }

        if let Some(grid_layout) = self.grid_state.layouts() {
            if grid_layout.union().contains(mouse_pos) {
                self.grid_state.handle_mouse_event(mouse_event);
            }
        }
    }
}
