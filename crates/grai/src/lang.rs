mod opcode;
pub use opcode::*;

mod operand;
pub use operand::*;

use crate::Cell;

pub enum Word {
    Operand(Operand),
    Opcode(Opcode),
}

impl Word {
    pub fn from_cell(cell: Cell) -> Self {
        // avoid clone ?
        if let Ok(opcode) = Opcode::from_cell(cell.clone()) {
            Self::Opcode(opcode)
        } else {
            Self::Operand(Operand::from_cell(cell))
        }
    }
}
