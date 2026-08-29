use act::{Action, Revert, State};
use serde::{Deserialize, Serialize};

use crate::Operand;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stack(Vec<Operand>);

impl Stack {
    /// Obtain a new empty `Stack`
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an operand at the top of the stack
    ///
    /// # Panic
    /// Panic if we can't push to the stack,
    /// see [`Vec::push()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
    fn push(&mut self, operand: Operand) {
        self.0.push(operand);
    }

    /// Remove the operand on top of the stack
    fn pop(&mut self) -> Option<Operand> {
        self.0.pop()
    }

    /// Get the operand on top of the stack
    pub fn last(&self) -> Option<&Operand> {
        self.0.last()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Operand> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Operand> {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone)]
pub enum StackAction {
    Push(Operand),
    Pop,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum StackError {
    #[error("could not pop the stack, as it is empty")]
    EmptyStack,
    // StackOverflow is unlikely
    // #[error("could not push to the stack, as it would overflow")]
    // StackOverflow,
}

impl Action for StackAction {}

impl State for Stack {
    type Action = StackAction;
    type Error = StackError;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert, Self::Error> {
        match action.into() {
            StackAction::Push(operand) => {
                self.push(operand.clone());
                Ok(Revert::new(StackAction::Pop))
            }
            StackAction::Pop => {
                if let Some(popped) = self.pop() {
                    Ok(Revert::new(StackAction::Push(popped)))
                } else {
                    Err(StackError::EmptyStack)
                }
            }
        }
    }
}
