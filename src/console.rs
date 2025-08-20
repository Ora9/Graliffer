use std::fmt::Debug;

use crate::{
    Frame,
    action::{FrameAction, Artifact},
};

#[derive(Default, Debug)]
pub struct Console {
    pub buffer: String,
}

impl Console {
    pub const MAX_BUFFER_LENGTH: u32 = 1000;

    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    fn print(&mut self, string: &str) {
        self.buffer.push_str(string);
        dbg!(string);
    }
}

#[derive(Clone)]
pub enum ConsoleAction {
    Print(String),
}

impl FrameAction for ConsoleAction {
    fn act(&self, frame: &mut Frame) -> Artifact {
        match self {
            Self::Print(string) => {
                frame.console.print(string);

                Artifact::from_redo(Box::new(self.to_owned()))
            }
        }
    }
}

impl Debug for ConsoleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Print(string) => {
                write!(f, "ConsoleAction::Print ({:?})", string)
            }
        }
    }
}
