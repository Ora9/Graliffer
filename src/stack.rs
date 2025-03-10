//! Stack is a part of Graliffer's memory system
//! The grid holds the code, and data
//! The stack hold execution data

use crate::Operand;

#[derive(Default, Debug)]
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

    pub fn pop_err(&mut self) -> Result<Operand, anyhow::Error> {
        self.data.pop().ok_or(anyhow::anyhow!("Could not pop an element from the stack"))
    }


    // /// Return an option of the last operation in the stack if there is any
    // pub fn drain_last_operation(&mut self) -> Option<Operation> {
    //     if let Some((operation, index)) = self.get_last_operation_with_index() {
    //         self.truncate(index);
    //         Some(operation)
    //     } else {
    //         None
    //     }
    // }

    // pub fn peek_last_operation(&self) -> Option<Operation> {
    //     // return operation and drop index as we don't need it
    //     self.get_last_operation_with_index().map(|opt| opt.0)
    // }

    // // This is quite ugly, please refactor this, actually, the whole stack implementation is
    // fn get_last_operation_with_index(&self) -> Option<(Operation, usize)> {
    //     let mut operands: Vec<Operand> = Vec::new();
    //     let mut opcode_opt: Option<(Opcode, usize)> = None;

    //     for (index, word) in self.data.iter().enumerate().rev() {
    //         match word {
    //             Word::Operand(operand) => {
    //                 operands.push(operand.clone());
    //             },
    //             Word::Opcode(opcode) => {
    //                 opcode_opt = Some((opcode.clone(), index));
    //                 break;
    //             }
    //         }
    //     }

    //     if let Some((opcode, index)) = opcode_opt {
    //         let operands_needed = opcode.syntax().operand_amount();

    //         // Keeping only operand that are needed, based on opcode syntax
    //         operands.truncate(operands_needed as usize);

    //         if let Ok(operation) = Operation::new(opcode, operands) {
    //             Some((operation, index))
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }

    // pub fn truncate(&mut self, length: usize) {
    //     self.data.truncate(length);
    // }
}
