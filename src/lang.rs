//! Graliffer words are either an Opcode or an Operand

use crate::{grid::Cell, Frame};

mod opcode;
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
