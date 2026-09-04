use act::Timeline;
use crossterm::event::{KeyEvent, MouseEvent};
use log::debug;
use ratatui::layout::Position;

use crate::{AppState, Context, GridView, Key, Keystroke, Picker, PickerView, View};

pub fn handle_key_events(
    app_state_timeline: &mut Timeline<AppState>,
    key_event: KeyEvent,
    app_context: Context,
) {
    if let Ok(keystroke) = Keystroke::try_from(key_event) {
        let state = app_state_timeline.state();

        if let Some(action) = state.keymap.find(app_context, keystroke) {
            app_state_timeline.act(action);
        } else if let Key::Char(char) = keystroke.key {
            let input = char.to_string();
            let action = match state.focused().to_string().as_str() {
                "Grid" => GridView::input_sink_action(input),
                "Picker" => PickerView::input_sink_action(input),
                _ => None,
            };

            if let Some(action) = action {
                app_state_timeline.act(action);
            }
        }
    }
}

impl AppState {
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
