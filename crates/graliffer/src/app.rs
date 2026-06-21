use std::{
    cell::{Ref, RefCell},
    iter,
    ops::AddAssign,
    rc::Rc,
};

use action::{Action, AnyAction, Revert, State};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use log::debug;
use rand::seq::SliceRandom;
use ratatui::layout::Position;

use crate::{
    app,
    inputs::{InputMode, Keymap},
    ui::{Console, ConsoleAction, ConsoleState, FocusedPane, GridState},
};

#[derive(Debug)]
pub struct App {
    pub should_run: bool,

    pub console_state: ConsoleState,
    pub grid_state: GridState,

    pub input_mode: InputMode,
    pub keymap: Keymap,

    pub focused: RefCell<Focusable>,
}

impl App {
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
        let mut frame = Rc::new(RefCell::new(grai::Frame {
            grid,
            head: grai::Head::default(),
            stack: grai::Stack::default(),
        }));

        let app_focus = RefCell::new(Focusable::Grid);

        // let grid_focus_handle = FocusHandle::new(Focusable::Grid, app_focus);

        let mut app = Self {
            should_run: true,

            input_mode: InputMode::Command,
            keymap: Keymap::new(),

            console_state: ConsoleState::new(
                1000,
                FocusHandle::new(Focusable::Console, app_focus.clone()),
            ),
            grid_state: GridState::new(frame, FocusHandle::new(Focusable::Grid, app_focus.clone())),
            focused: app_focus,
        };

        let mut rng = rand::rng();
        let phrase = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string();

        let mut shuffler = || {
            let mut phrase = phrase.split(" ").collect::<Vec<&str>>();
            phrase.shuffle(&mut rng);
            phrase.join(" ").to_string()
        };

        for i in 0..100 {
            app.console_state.append_line(shuffler());
        }

        app
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        // self.console_state.scroll_down_by(1);

        // self.console_state.scroll_offset = self.console_state.scroll_offset.wrapping_add(1);
    }

    pub fn focused(&self, focusable: Focusable) -> bool {
        *self.focused.borrow() == focusable
    }

    pub fn focus(&mut self, focused: Focusable) {
        *self.focused.get_mut() = focused
    }

    // pub fn input_mode(&mut self, input_mode: InputMode) {
    //     self.input_mode = input_mode;
    //     if input_mode == InputMode::Insert {
    //         self.focused_pane = FocusedPane::Grid
    //     }
    // }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_run = false;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focusable {
    Grid,
    Console,
    Stack,
    // Popup(PopupId)
}

impl Focusable {
    pub fn grid(&self) -> bool {
        matches!(self, Self::Grid)
    }

    pub fn console(&self) -> bool {
        matches!(self, Self::Console)
    }

    pub fn stack(&self) -> bool {
        matches!(self, Self::Stack)
    }
}

#[derive(Debug)]
pub struct FocusHandle {
    current: Focusable,
    app_focus: RefCell<Focusable>,
}

impl FocusHandle {
    pub fn new(current: Focusable, app_focus: RefCell<Focusable>) -> Self {
        Self { current, app_focus }
    }

    pub fn focused(&self) -> bool {
        self.current == *self.app_focus.borrow()
    }

    // pub fn focus(&mut self, focused: bool) {
    //     self.focused = focused
    // }
}

#[derive(Debug, Clone)]
pub enum AppAction {
    Quit,
    About,
    FocusStack,
}

impl Action for AppAction {}

impl State for App {
    type Action = AnyAction;
    type Error = eyre::Error;

    fn act(&mut self, action: &Self::Action) -> Result<Revert, Self::Error> {
        if let Some(app_action) = action.downcast_ref::<AppAction>() {
            use AppAction::*;
            match app_action {
                Quit => {
                    self.quit();
                }
                About => {
                    debug!("about!");
                }
                FocusStack => {
                    self.focus(Focusable::Stack);
                }
            };
            Ok(Revert::None)

            // debug!("app action");
            // unimplemented!()
        } else if let Some(console_action) = action.downcast_ref::<ConsoleAction>() {
            self.console_state.act(console_action)
        } else {
            Err(eyre::anyhow!("unknown action"))
        }
    }
}
