use crate::{
    artifact::{Action, Artifact}, grid::{Cell, Grid, Head, Position}, stack::{Stack, StackAction}, Address, Opcode, Operand, Word
};

#[derive(Default)]
pub struct RunDescriptor {
    pub head: Head,
    pub grid: Grid,
    pub stack: Stack,
}

/// A [`Frame`] represents a run
// #[derive(Debug)]
pub struct Frame {
    pub head: Head,
    pub grid: Grid,
    pub stack: Stack,
}

impl Frame {
    pub fn new(descriptor: RunDescriptor) -> Self {
        Self {
            head: descriptor.head,
            grid: descriptor.grid,
            stack: descriptor.stack,
        }
    }

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
    pub fn step(&mut self) {
        let current_cell = self.grid.get(self.head.position);

        if current_cell.is_empty() {
            let _ = self.head.take_step();
        } else {
            let word = Word::from_cell(current_cell);

            let artifact = match word {
                Word::Opcode(opcode) => {
                    println!("Opcode! : {:?}", opcode);
                    opcode.evaluate(self)
                }
                Word::Operand(operand) => {
                    let _ = self.head.take_step();
                    self.act(Box::new(StackAction::Push(operand)))
                }
            };
            println!(" - {:?}", self.stack);

            dbg!(artifact);
        }
    }

    #[must_use]
    pub fn act(&mut self, action: Box<dyn Action>) -> Artifact {
        action.act(self)
    }

    pub fn act_by_ref(&mut self, action: &dyn Action) -> Artifact {
        action.act(self)
    }
}
