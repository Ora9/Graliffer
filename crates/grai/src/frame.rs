use std::any::type_name_of_val;

use serde::{Deserialize, Serialize};

use crate::{
    Action, AnyAction, Apply, Grid, GridAction, Head, HeadAction, Revert, Stack, StackAction, State,
};

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
