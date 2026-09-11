use act::State;
use crossterm::event::{KeyEvent, MouseEvent};
use grai::FrameGuard;
use rand::seq::SliceRandom;
use ratatui::layout::Position;

use crate::{
    Config, ConsoleView, GridView, Key, Keystroke, PaneId, PickerView, PopupId, StackView, View,
    ViewId,
    input::{InputMode, Keymap},
};

mod render;

mod actions;
pub use actions::*;

mod context;
pub use context::*;

#[derive(Debug)]
pub struct App {
    pub context: Context,

    pub keymap: Keymap,

    pub frame: FrameGuard,

    pub grid_view: GridView,
    pub stack_view: StackView,
    pub console_view: ConsoleView,

    pub command_picker_state: PickerView,

    pub should_run: bool,
    pub last_focused_pane: Option<PaneId>,
}

#[derive(Debug, Default)]
pub struct AppWidget;

impl AppWidget {
    pub fn new() -> Self {
        Self
    }
}

impl App {
    pub fn new(config: Config) -> Self {
        let frame = grai::FrameGuard::new(
            grai::Frame::from_example("getting_started").expect("should be a valid example"),
        );

        let default_focus = GridView::view_id();

        let context = Context::new(config, default_focus);

        let mut app = Self {
            frame: frame.clone(),

            keymap: Keymap::new(),

            context: context.clone(),

            console_view: ConsoleView::new(context.clone()),
            grid_view: GridView::new(frame.clone(), context.clone()),
            stack_view: StackView::new(frame),

            command_picker_state: PickerView::new(context.clone()),

            should_run: true,
            last_focused_pane: None,
        };

        let mut rng = rand::rng();
        let phrase = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string();

        let mut shuffler = || {
            let mut phrase = phrase.split(" ").collect::<Vec<&str>>();
            phrase.shuffle(&mut rng);
            phrase.join(" ").to_string()
        };

        for _ in 0..100 {
            app.console_view.append_line(shuffler());
        }

        app
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {}

    pub fn is_focused(&self, focus_id: impl Into<ViewId>) -> bool {
        self.focused() == focus_id.into()
    }

    pub fn focused(&self) -> ViewId {
        self.context.focus()
    }

    pub fn set_focus(&mut self, focus_id: impl Into<ViewId>) {
        self.context.set_focus(focus_id.into());
    }

    pub fn popup_opened(&self) -> bool {
        matches!(self.focused(), ViewId::Popup(_))
    }

    pub fn close_popup(&mut self) {
        if let Some(last_focus) = self.last_focused_pane.clone() {
            self.set_focus(last_focus);
        }

        self.context.remove_flag("popuped");
    }

    pub fn open_popup(&mut self, popup_id: PopupId) {
        if let ViewId::Pane(pane_id) = self.focused() {
            self.last_focused_pane = Some(pane_id);
        }

        self.context.insert_flag("popuped".to_string());

        self.set_focus(popup_id);
    }

    pub fn toggle_popup(&mut self, popup_id: PopupId) {
        if self.is_focused(popup_id.clone()) {
            self.close_popup();
        } else {
            self.open_popup(popup_id);
        }
    }

    pub fn input_mode(&self) -> InputMode {
        self.context.input_mode()
    }

    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.context.set_input_mode(input_mode);
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_run = false;
    }
}

impl App {
    pub fn handle_key_events(&mut self, key_event: KeyEvent, app_context: Context) {
        if let Ok(keystroke) = Keystroke::try_from(key_event) {
            if let Some(action) = self.keymap.find(app_context, keystroke) {
                let _ = self.act(action);
            } else if let Key::Char(char) = keystroke.key {
                let input = char.to_string();
                let action = match self.focused().to_string().as_str() {
                    "Grid" => GridView::input_sink_action(input),
                    "Picker" => PickerView::input_sink_action(input),
                    _ => None,
                };

                if let Some(action) = action {
                    let _ = self.act(action);
                }
            }
        }
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        let mouse_pos = Position {
            x: mouse_event.column,
            y: mouse_event.row,
        };

        if let Some(console_layouts) = self.console_view.layouts() {
            let contained = console_layouts
                .viewport_area()
                .union(console_layouts.vertical_scrollbar_area())
                .contains(mouse_pos);

            if contained {
                self.console_view.handle_mouse_event(mouse_event);
            }
        }

        if let Some(grid_layout) = self.grid_view.layouts()
            && grid_layout.union().contains(mouse_pos)
        {
            self.grid_view.handle_mouse_event(mouse_event);
        }
    }
}
