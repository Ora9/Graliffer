use std::{cell::RefCell, rc::Rc};

use act::{Action, Revert, State};
use serde::{Deserialize, Serialize};

pub mod examples;

mod grid;
pub use grid::*;

mod stack;
pub use stack::*;

mod head;
pub use head::*;
use unwrap_infallible::UnwrapInfallible;

use crate::{OpcodeError, Word};

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
    type Action = FrameAction;
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
    pub fn step(&mut self) -> Result<Revert, <Frame as State>::Error> {
        let cell = self.grid.get(self.head.position);

        if cell.is_empty() {
            self.act(HeadAction::Step)
        } else {
            match Word::from_cell(cell) {
                Word::Opcode(opcode) => Ok(opcode.evaluate(self)?),
                Word::Operand(operand) => {
                    let push = self.act(StackAction::Push(operand))?;
                    let step = self.act(HeadAction::Step)?;

                    Ok(vec![push, step].into())
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum FrameAction {
    Step,
    // Run,

    // SetBreakpoint(Position),
    // ToggleBreakpoint(Position),
    Grid(GridAction),
    Stack(StackAction),
    Head(HeadAction),
}

impl From<GridAction> for FrameAction {
    fn from(value: GridAction) -> Self {
        Self::Grid(value)
    }
}

impl From<StackAction> for FrameAction {
    fn from(value: StackAction) -> Self {
        Self::Stack(value)
    }
}

impl From<HeadAction> for FrameAction {
    fn from(value: HeadAction) -> Self {
        Self::Head(value)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FrameError {
    #[error("stack error : {0}")]
    Stack(#[from] StackError),

    #[error("opcode error : {0}")]
    Opcode(#[from] OpcodeError),
}

impl Action for FrameAction {}

impl State for Frame {
    type Action = FrameAction;
    type Error = FrameError;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert, Self::Error> {
        match action.into() {
            FrameAction::Grid(grid_action) => Ok(self.grid.act(grid_action).unwrap_infallible()),
            FrameAction::Head(head_action) => Ok(self.head.act(head_action).unwrap_infallible()),
            FrameAction::Stack(stack_action) => {
                self.stack.act(stack_action).map_err(|err| err.into())
            }

            FrameAction::Step => self.step(),
        }
    }
}
