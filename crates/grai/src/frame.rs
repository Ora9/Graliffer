use std::{any::type_name_of_val, cell::RefCell, ops::Deref, rc::Rc};

use action::{AnyAction, Revert, State};
use serde::{Deserialize, Serialize};

use crate::{Grid, GridAction, Head, HeadAction, Stack, StackAction};

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

    fn act(&mut self, action: &Self::Action) -> Result<Revert, Self::Error> {
        self.write(|frame| frame.act(action))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
    pub head: Head,
    pub grid: Grid,
    pub stack: Stack,
}

impl State for Frame {
    type Error = FrameError;
    type Action = AnyAction;

    fn act(&mut self, action: &Self::Action) -> Result<Revert, Self::Error> {
        if let Some(head_action) = action.downcast_ref::<HeadAction>() {
            self.head
                .act(head_action)
                .map_err(|_| FrameError::HeadError)
        } else if let Some(stack_action) = action.downcast_ref::<StackAction>() {
            self.stack
                .act(stack_action)
                .map_err(|_| FrameError::StackError)
        } else if let Some(grid_action) = action.downcast_ref::<GridAction>() {
            self.grid
                .act(grid_action)
                .map_err(|_| FrameError::HeadError)
        } else {
            Err(FrameError::UnknownAction(
                type_name_of_val(action)
                    .split("::")
                    .last()
                    .unwrap_or("unknown action")
                    .to_string(),
            ))
        }
    }
}
