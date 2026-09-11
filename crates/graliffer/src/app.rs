use grai::FrameGuard;
use rand::seq::SliceRandom;

use crate::{
    ConsoleView, GridView, PickerView, StackView, View, ViewId,
    config::Config,
    input::{InputMode, Keymap},
};

mod render;

mod events;

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
    pub last_focused_pane: Option<ViewId>,
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

    pub fn tick(&mut self) {}

    pub fn focusing(&self, view_id: impl Into<ViewId>) -> bool {
        self.focused() == view_id.into()
    }

    pub fn focused(&self) -> ViewId {
        self.context.focus()
    }

    pub fn set_focus(&mut self, focus_id: impl Into<ViewId>) {
        self.context.set_focus(focus_id.into());
    }

    pub fn popup_opened(&self) -> bool {
        self.context.has_flag("popup_opened")
    }

    pub fn close_popup(&mut self) {
        self.context.remove_flag("popup_opened");

        if let Some(last_focus) = self.last_focused_pane {
            self.set_focus(last_focus);
        }
    }

    pub fn open_popup(&mut self, view_id: impl Into<ViewId>) {
        if !self.context.has_flag("popup_opened") {
            self.last_focused_pane = Some(self.focused());
        }

        self.context.insert_flag("popup_opened");
        self.set_focus(view_id.into());
    }

    pub fn toggle_popup(&mut self, view_id: impl Into<ViewId>) {
        let view_id = view_id.into();

        if self.focusing(view_id) {
            self.close_popup();
        } else {
            self.open_popup(view_id);
        }
    }

    pub fn input_mode(&self) -> InputMode {
        self.context.input_mode()
    }

    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.context.set_input_mode(input_mode);
    }

    pub fn quit(&mut self) {
        self.should_run = false;
    }
}
