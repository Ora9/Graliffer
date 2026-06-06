use std::any::Any;

use crate::{Action, Grid, GridAction, Head, HeadAction, Stack, StackAction, State};

#[derive(Debug)]
pub struct Frame {
    pub head: Head,
    pub grid: Grid,
    pub stack: Stack,
}

impl Frame {
    pub fn act(&mut self, action: impl Action + 'static) {
        let action = &action as &dyn Any;

        if let Some(head_action) = action.downcast_ref::<HeadAction>() {
            self.head.act(head_action);
        } else if let Some(stack_action) = action.downcast_ref::<StackAction>() {
            self.stack.act(stack_action);
        } else if let Some(grid_action) = action.downcast_ref::<GridAction>() {
            self.grid.act(grid_action);
        } else {
            eprintln!("unknown action");
        }
    }
}
