use std::ops::AddAssign;

use crate::ui::{Console, ConsoleState};

#[derive(Debug)]
pub struct App {
    pub should_run: bool,
    pub console_state: ConsoleState,
    pub focused: Focused,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_run: true,
            focused: Focused::Grid,
            console_state: ConsoleState::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.console_state.scroll_offset = self.console_state.scroll_offset.wrapping_add(1);
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_run = false;
    }
}

#[derive(Debug)]
pub enum Focused {
    Grid,
    Stack,
    Console,
}
