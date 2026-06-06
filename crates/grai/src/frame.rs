use std::any::Any;

use crate::{
    Action, ActionBox, Grid, GridAction, Head, HeadAction, Revert, Stack, StackAction, State,
};

#[derive(Debug, thiserror::Error)]
pub enum FrameError {
    #[error("head error")]
    HeadError,
    #[error("grid error")]
    GridError,
    #[error("stack error")]
    StackError,
}

#[derive(Debug)]
pub struct Frame {
    pub head: Head,
    pub grid: Grid,
    pub stack: Stack,
}

impl Frame {
    pub fn act(&mut self, action: impl Action + 'static) -> Result<Revert, FrameError> {
        let action = &action as &dyn Any;

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
            eprintln!("unknown action");
            panic!()
        }
    }
}
