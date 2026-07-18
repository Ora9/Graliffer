use std::{cell::RefCell, rc::Rc, str::FromStr};

use eyre::eyre;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::{
    ConsoleAction, ConsoleState, Context, FocusId, GridAction, GridState, PaneId, PopupId,
    input::{InputMode, Keymap},
    ui::PickerState,
};

mod action;
pub use action::*;

#[derive(Debug)]
pub struct AppState {
    pub should_run: bool,

    pub console_state: ConsoleState,
    pub grid_state: GridState,
    pub command_picker_state: PickerState,

    pub keymap: Keymap,

    pub last_focused_pane: Option<PaneId>,
    pub context: Context,
}

#[derive(Debug)]
pub struct App;

impl App {
    pub fn new() -> Self {
        Self
    }
}

impl AppState {
    pub fn new() -> Self {
        let mut grid = grai::Grid::new();

        grid.set(
            grai::Position::from_string("AA").unwrap(),
            grai::Cell::new_trim("100"),
        );
        grid.set(
            grai::Position::from_string("BA").unwrap(),
            grai::Cell::new_trim("&BB"),
        );
        grid.set(
            grai::Position::from_string("CA").unwrap(),
            grai::Cell::new_trim("div"),
        );
        grid.set(
            grai::Position::from_string("BB").unwrap(),
            grai::Cell::new_trim("@CB"),
        );
        grid.set(
            grai::Position::from_string("CB").unwrap(),
            grai::Cell::new_trim("3"),
        );

        grid.set(
            grai::Position::from_string("EA").unwrap(),
            grai::Cell::new_trim("20"),
        );
        grid.set(
            grai::Position::from_string("FA").unwrap(),
            grai::Cell::new_trim("sub"),
        );
        grid.set(
            grai::Position::from_string("HA").unwrap(),
            grai::Cell::new_trim("@AB"),
        );
        grid.set(
            grai::Position::from_string("IA").unwrap(),
            grai::Cell::new_trim("set"),
        );
        grid.set(
            grai::Position::from_string("aa").unwrap(),
            grai::Cell::new_trim("jmp"),
        );
        let frame = Rc::new(RefCell::new(grai::Frame {
            grid,
            head: grai::Head::default(),
            stack: grai::Stack::default(),
        }));

        let context = Context::new(PaneId::Grid, InputMode::Insert);

        let mut app = Self {
            context: context.clone(),

            should_run: true,

            keymap: Keymap::new(),

            console_state: ConsoleState::new(1000, context.clone()),
            grid_state: GridState::new(frame, context),
            command_picker_state: PickerState::new(),

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

    pub fn is_focused(&self, focus_id: impl Into<FocusId>) -> bool {
        self.focused() == focus_id.into()
    }

    pub fn focused(&self) -> FocusId {
        self.context.get_focus()
    }

    pub fn set_focus(&mut self, focus_id: impl Into<FocusId>) {
        self.context.set_focus(focus_id.into());
    }

    pub fn popup_opened(&self) -> bool {
        matches!(self.focused(), FocusId::Popup(_))
    }

    pub fn close_popup(&mut self) {
        if let Some(last_focus) = self.last_focused_pane {
            self.set_focus(last_focus);
        }

        self.context.remove_flag("focusing_popup");
    }

    pub fn open_popup(&mut self, popup_id: PopupId) {
        if let FocusId::Pane(pane_id) = self.focused() {
            self.last_focused_pane = Some(pane_id);
        }

        self.context.insert_flag("focusing_popup".to_string());

        self.set_focus(popup_id);
    }

    pub fn toggle_popup(&mut self, popup_id: PopupId) {
        if self.is_focused(popup_id) {
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
