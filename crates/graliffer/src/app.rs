use std::{cell::RefCell, iter, ops::AddAssign, rc::Rc};

use action::{Action, AnyAction, Revert, State};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use log::debug;
use rand::seq::SliceRandom;
use ratatui::layout::Position;

use crate::{
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

    pub focused_pane: FocusedPane,
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

        let mut app = Self {
            should_run: true,

            input_mode: InputMode::Command,
            keymap: Keymap::new(),

            console_state: ConsoleState::new(1000),
            grid_state: GridState::new(frame),

            focused_pane: FocusedPane::Grid,
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

    pub fn input_mode(&mut self, input_mode: InputMode) {
        self.input_mode = input_mode;
        if input_mode == InputMode::Insert {
            self.focused_pane = FocusedPane::Grid
        }
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_run = false;
    }
}

#[derive(Debug, Clone)]
pub enum AppAction {
    Quit,
    About,
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
