use serde::{Deserialize, Serialize};

use crate::{
    artifact::{Action, Artifact}, console::Console, grid::{Grid, Head, HeadAction}, stack::{Stack, StackAction}, Word
};

// #[derive(Default)]
// pub struct RunDescriptor {
//     pub head: Head,
//     pub grid: Grid,
//     pub stack: Stack,
// }

/// A [`Frame`] represents a run
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Frame {
    pub head: Head,
    pub grid: Grid,
    pub stack: Stack,

    #[serde(skip)]
    pub console: Console,
}

impl Frame {
    // pub fn new() -> Self {
    //     Self {
    //         head: descriptor.head,
    //         grid: descriptor.grid,
    //         stack: descriptor.stack,

    //         console: descriptor.console,
    //     }
    // }

    /// Make a step, the minimal unit of a Graliffer execution :
    /// - Move head 1 cell in its direction
    /// - Parse the Cell under the head
    /// - If cell content :
    ///     - is empty, continue
    ///     - can correspond to an Opcode, push to stack
    ///     - can be parsed as an Adress Operand, push to stack
    ///     - can be parsed as a Pointer Operand, push to stack
    ///     - is non of the above, push to stack as a Literal Operand
    /// - Does the stack contains a valid operation
    ///     - if yes, evaluate the operation
    ///     - if not, hop
    ///
    pub fn step(&mut self) -> Artifact {
        let current_cell = self.grid.get(self.head.position);

        if current_cell.is_empty() {
            self.act(Box::new(HeadAction::TakeStep()))
        } else {
            let word = Word::from_cell(current_cell);

            match word {
                Word::Opcode(opcode) => {
                    println!("Opcode! : {:?}", opcode);
                    opcode.evaluate(self)
                }
                Word::Operand(operand) => {
                    let mut artifact = self.act(Box::new(StackAction::Push(operand)));
                    artifact.push(self.act(Box::new(HeadAction::TakeStep())));

                    artifact
                }
            }
        }
    }

    #[must_use]
    pub fn act(&mut self, action: Box<dyn Action>) -> Artifact {
        action.act(self)
    }

    #[must_use]
    pub fn act_by_ref(&mut self, action: &dyn Action) -> Artifact {
        action.act(self)
    }
}
