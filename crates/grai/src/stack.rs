use action::{Action, Revert, State};
use serde::{Deserialize, Serialize};

use crate::Operand;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Stack(Vec<Operand>);

impl Stack {
    /// Obtain a new empty `Stack`
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an operand at the top of the stack
    pub fn push(&mut self, operand: Operand) {
        self.0.push(operand);
    }

    /// Remove the operand on top of the stack
    pub fn pop(&mut self) -> Option<Operand> {
        self.0.pop()
    }

    /// Get the operand on top of the stack
    pub fn get_last(&self) -> Option<&Operand> {
        self.0.last()
    }

    // pub fn iter(&self) -> Iter<'_, Operand> {
    //     self.0.iter()
    // }
}

#[derive(Debug, Clone)]
pub enum StackAction {
    Push(Operand),
    Pop,
}

impl Action for StackAction {}

impl State for Stack {
    type Action = StackAction;
    type Error = ();

    fn act(&mut self, action: &StackAction) -> Result<Revert, Self::Error> {
        match action {
            StackAction::Push(operand) => {
                self.push(operand.clone());
                Ok(Revert::new(StackAction::Pop))
            }
            StackAction::Pop => {
                let popped = self.pop();

                if let Some(popped) = self.pop() {
                    Ok(Revert::new(StackAction::Push(popped)))
                } else {
                    Ok(Revert::None)
                }
            }
        }
    }
}
