use std::{any::type_name_of_val, cell::RefCell, rc::Rc};

use action::{Action, AnyAction, Revert, State};
use serde::{Deserialize, Serialize};

pub mod examples;

mod grid;
pub use grid::*;

mod stack;
pub use stack::*;

mod head;
pub use head::*;

use crate::Word;

#[derive(Debug, thiserror::Error)]
pub enum FrameError {
    #[error("head error")]
    HeadError,
    #[error("grid error")]
    GridError,
    #[error("stack error")]
    StackError,

    #[error("unknown action, found {0}")]
    UnknownAction(String),
}

#[derive(Debug, Clone)]
pub struct FrameGuard(Rc<RefCell<Frame>>);

impl FrameGuard {
    pub fn new(frame: Frame) -> Self {
        Self(Rc::new(RefCell::new(frame)))
    }

    pub fn read<T>(&self, reader: impl FnOnce(&Frame) -> T) -> T {
        reader(&self.0.borrow())
    }

    pub fn write<T>(&mut self, writer: impl FnOnce(&mut Frame) -> T) -> T {
        writer(&mut *self.0.borrow_mut())
    }
}

impl State for FrameGuard {
    type Action = AnyAction;
    type Error = FrameError;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert, Self::Error> {
        self.write(|frame| frame.act(action))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
    pub head: Head,
    pub grid: Grid,
    pub stack: Stack,
}

impl Frame {
    pub fn step(&mut self) {
        let cell = self.grid.get(self.head.position);

        if cell.is_empty() {
            self.act(AnyAction::new(HeadAction::Step));
        } else {
            let word = Word::from_cell(cell);

            match word {
                Word::Opcode(opcode) => {
                    println!("opcode: {:?}", opcode);
                    opcode.evaluate(self);
                }
                Word::Operand(operand) => {}
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum FrameAction {
    Step,
    Run,

    SetBreakpoint(Position),
    ToggleBreakpoint(Position),
}

impl Action for FrameAction {}

impl State for Frame {
    type Error = FrameError;
    type Action = AnyAction;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert, Self::Error> {
        Ok(Revert::None)

        // let action = action.into();

        // if let Ok(frame_action) = action.downcast::<FrameAction>() {
        //     dbg!(frame_action);
        //     Ok(Revert::None)
        // } else if let Ok(head_action) = action.downcast::<HeadAction>() {
        //     self.head
        //         .act(*head_action)
        //         .map_err(|_| FrameError::HeadError)
        // } else if let Ok(stack_action) = action.downcast::<StackAction>() {
        //     self.stack
        //         .act(*stack_action)
        //         .map_err(|_| FrameError::StackError)
        // } else if let Ok(grid_action) = action.downcast::<GridAction>() {
        //     self.grid
        //         .act(*grid_action)
        //         .map_err(|_| FrameError::HeadError)
        // } else {
        //     Err(FrameError::UnknownAction(
        //         type_name_of_val(&action)
        //             .split("::")
        //             .last()
        //             .unwrap_or("unknown action")
        //             .to_string(),
        //     ))
        // }
    }
}
