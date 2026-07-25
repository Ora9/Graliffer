use std::{cell::RefCell, convert::Infallible, rc::Rc};

use act::{Action, Revert, State};
use serde::{Deserialize, Serialize};

pub mod examples;

mod grid;
pub use grid::*;

mod stack;
pub use stack::*;

mod head;
pub use head::*;

use crate::Word;

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
    type Error = Infallible;

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
                Word::Opcode(opcode) => {
                    let eval = opcode.evaluate(self)?;
                    let step = self.act(HeadAction::Step)?;

                    Ok(vec![eval, step].into())
                }
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

impl Action for FrameAction {}

impl State for Frame {
    type Error = Infallible;
    type Action = FrameAction;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert, Self::Error> {
        match action.into() {
            FrameAction::Grid(grid_action) => self.grid.act(grid_action),
            FrameAction::Head(head_action) => self.head.act(head_action),
            FrameAction::Stack(stack_action) => self.stack.act(stack_action),

            FrameAction::Step => self.step(),
        }
    }
}
