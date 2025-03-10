//! Graliffer words are either an Opcode or an Operand

use crate::{grid::Cell, Frame};

mod opcode;
use anyhow::{anyhow, bail};
pub use opcode::Opcode;

mod operand;
pub use operand::{Operand, Literal, Address, Pointer};

/// A `Word` is the broadest language element in Graliffer
///
/// It can either be :
/// - an [`Opcode`] : an "operation code",
/// - an [`Operand`] : the object of an operation, its parameters, an operand in itself can be one of 3 things :
///     - a [`Literal`] : a piece of text, with no inhinherent meaning
///     - a [`Address`] : a representation of [`Cell`]'s position in a [`Grid`](crate::grid::Grid)
///     - a [`Pointer`] : pointing to another cell's operand. Allowing pointers chain
#[derive(Debug, Clone)]
pub enum Word {
    Opcode(Opcode),
    Operand(Operand),
}

impl Word {
    /// Return a `Word` given a valid [`Cell`]
    /// cell is a valid [`Operand`] return `Self::Operand` variant, containing the parsed

    pub fn from_cell(cell: Cell) -> Self {
        if Opcode::is_cell_valid(&cell) {
            Self::Opcode(Opcode::from_cell(cell).unwrap())
        } else {
            Self::Operand(Operand::from_cell(cell))
        }
    }
}

/// An operation is a combination of an [`Opcode`] an multiple [`Operands`]
#[derive(Debug)]
pub struct Operation {
    opcode: Opcode,
    operands: Vec<Operand>,
}

// impl Operation {
//     /// Return a "valid" operation, valid here means an [`Opcode`] and a `Vec` of [`Operand`]s, the amount of operands in this list must match the opcode's syntax
//     ///
//     /// # Errors:
//     /// Return an error if the vec of operand length does not match the opcode's amount of needed operand
//     pub fn new(opcode: Opcode, operands: Vec<Operand>) -> Result<Self, anyhow::Error> {
//         let operands_needed = opcode.syntax().operand_amount();

//         if operands.len() as u32 != operands_needed {
//             Err(anyhow!("Trying to construct opcode '{:?}' needed {} operands, butwas given {}", opcode, operands_needed, operands.len()))
//         } else {
//             Ok(Self {
//                 opcode,
//                 operands
//             })
//         }
//     }

//     pub fn evaluate(&self, frame: &mut Frame) {
//         self.opcode().evaluate(self, frame);
//     }

//     pub fn opcode(&self) -> &Opcode {
//         &self.opcode
//     }

//     pub fn operands_iter(&self) -> std::slice::Iter<'_, Operand> {
//         self.operands.iter()
//     }

//     pub fn operand_nth(&self, index: usize) -> Option<&Operand> {
//         self.operands.iter().nth(index)
//     }
// }
