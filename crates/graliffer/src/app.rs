use std::{cell::RefCell, rc::Rc};

use rand::seq::SliceRandom;
use ratatui::layout::Position;

use crate::{
    Config, ConsoleView, Context, GridView, PaneId, PickerView, PopupId, View, ViewId,
    input::{InputMode, Keymap},
};

mod action;
pub use action::*;

#[derive(Debug)]
pub struct AppState {
    pub context: Context,

    pub keymap: Keymap,

    pub console_state: ConsoleView,
    pub grid_state: GridView,
    pub command_picker_state: PickerView,

    pub should_run: bool,
    pub last_focused_pane: Option<PaneId>,
}

#[derive(Debug)]
pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let frame = Rc::new(RefCell::new(
            grai::Frame::from_example("getting_started").expect("should be a valid example"),
        ));

        let default_focus = GridView::view_id();

        let context = Context::new(config, default_focus);

        let mut app = Self {
            keymap: Keymap::new(),

            context: context.clone(),

            should_run: true,

            console_state: ConsoleView::new(context.clone()),
            grid_state: GridView::new(frame, context.clone()),

            command_picker_state: PickerView::new(context.clone()),

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
            app.console_state.append_line(shuffler());
        }

        app
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {}

    pub fn is_focused(&self, focus_id: impl Into<ViewId>) -> bool {
        self.focused() == focus_id.into()
    }

    pub fn focused(&self) -> ViewId {
        self.context.get_focus()
    }

    pub fn set_focus(&mut self, focus_id: impl Into<ViewId>) {
        let focus_id = focus_id.into();

        let prev = self.context.get_focus();
        let next = focus_id.clone();

        if prev != next {
            match prev.to_string().as_str() {
                "Grid" => GridView::loose_focus(&mut self.context),
                "Picker" => PickerView::loose_focus(&mut self.context),
                _ => {}
            }

            match next.to_string().as_str() {
                "Grid" => GridView::gain_focus(&mut self.context),
                "Picker" => PickerView::gain_focus(&mut self.context),
                _ => {}
            }
        }

        self.context.set_focus(focus_id);
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
        self.context.get_input_mode()
    }

    pub fn set_input_mode(&mut self, input_mode: InputMode) {
        self.context.set_input_mode(input_mode);
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_run = false;
    }
}
