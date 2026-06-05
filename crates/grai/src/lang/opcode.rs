use std::str::FromStr;

use crate::Cell;

#[derive(Debug, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Opcode {
    Gou,
    Gor,
    God,
    Gol,
}

#[derive(Debug, thiserror::Error)]
pub enum OpcodeError {
    #[error("not an opcode, found {0}")]
    NotAnOpcode(String),
}

impl Opcode {
    pub fn from_cell(cell: Cell) -> Result<Opcode, OpcodeError> {
        Opcode::from_str(&cell.content()).map_err(|_| OpcodeError::NotAnOpcode(cell.content()))
    }
}
