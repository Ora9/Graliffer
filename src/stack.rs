//! Stack is a part of Graliffer's memory system
//! The grid holds the code, and data
//! The stack hold execution data

use crate::{artifact::{Action, Artifact}, Frame, Operand};

#[derive(Default, Debug)]
pub struct Stack {
    data: Vec<Operand>,
}

impl Stack {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, operand: Operand) {
        self.data.push(operand);
    }

    fn pop(&mut self) -> Option<Operand> {
        self.data.pop()
    }

    // fn pop_err(&mut self) -> Result<Operand, anyhow::Error> {
    //     self.data.pop().ok_or(anyhow::anyhow!("Could not pop an element from the stack"))
    // }

    pub fn get_last(&self) -> Option<&Operand> {
        self.data.last()
    }

    // pub fn get_last_err(&self) -> Result<&Operand, anyhow::Error> {
    //     self.data.last().ok_or(anyhow::anyhow!("Could not pop an element from the stack"))
    // }
}

#[derive(Debug, Clone)]
pub enum StackAction {
    Pop,
    Push(Operand)
}

impl Action for StackAction {
    fn act(&self, frame: &mut Frame) -> Artifact {
        match self {
            Self::Push(operand) => {
                frame.stack.push(operand.to_owned());

                Artifact::from_redo_undo(
                    Box::new(self.to_owned()),
                    Box::new(Self::Pop)
                )
            }
            Self::Pop => {
                if let Some(popped) = frame.stack.pop() {
                    Artifact::from_redo_undo(
                        Box::new(self.to_owned()),
                        Box::new(Self::Push(popped))
                    )
                } else {
                    Artifact::from_redo(
                        Box::new(self.to_owned())
                    )
                }
            }
        }
    }
}
