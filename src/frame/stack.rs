//! Stack is a part of Graliffer's memory system
//! The grid holds the code, and data
//! The stack hold execution data

use std::{fmt::Debug, slice::Iter};

use serde::{Deserialize, Serialize};

use crate::{
    Frame, Operand,
    history::Artifact,
};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Stack {
    data: Vec<Operand>,
}

impl Stack {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, operand: Operand) {
        self.data.push(operand);
    }

    pub fn pop(&mut self) -> Option<Operand> {
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

    pub fn iter(&self) -> Iter<'_, Operand> {
        self.data.iter()
    }
}
