use crate::ui::Console;

#[derive(Debug)]
pub struct App {
    pub should_run: bool,

    pub console: Console,

    pub focused: Focused,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_run: true,
            focused: Focused::Grid,

            console: Console::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

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
