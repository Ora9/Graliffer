use serde::{Deserialize, Serialize};

use crate::{Cell, Position, PositionError};

#[derive(Debug, thiserror::Error)]
pub enum OperandError {
    #[error("invalid address, expected to find format `@XY`, found `{0}`")]
    InvalidAddressFormat(String),

    #[error("invalid address, `{0}`")]
    InvalidAddress(#[source] PositionError),

    #[error("invalid pointer, expected to find format `&XY`, found `{0}`")]
    InvalidPointerFormat(String),

    #[error("invalid pointer, `{0}`")]
    InvalidPointer(#[source] PositionError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Literal(Cell);

impl Literal {
    pub fn new(cell: Cell) -> Self {
        Self(cell)
    }

    pub fn from_cell(cell: Cell) -> Self {
        Self::new(cell)
    }

    pub fn from_string_trim(string: &str) -> Self {
        Self::new(Cell::new_trim(string))
    }

    pub fn as_cell(&self) -> Cell {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address(Position);

impl Address {
    const PREFIX: char = '@';

    pub fn from_position(position: Position) -> Self {
        Self(position)
    }

    pub fn from_cell(cell: Cell) -> Result<Self, OperandError> {
        let cell_content = cell.content();
        let pos = cell_content
            .strip_prefix(Self::PREFIX)
            .ok_or(OperandError::InvalidAddressFormat(cell.content()))?;

        let pos = Position::from_string(pos).map_err(OperandError::InvalidAddress)?;

        Ok(Self::from_position(pos))
    }

    pub fn as_cell(&self) -> Cell {
        let (x, y) = self.0.as_textual();
        Cell::new_trim(&format!("{}{}{}", Self::PREFIX, x, y))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pointer(Position);

impl Pointer {
    const PREFIX: char = '&';

    pub fn from_position(position: Position) -> Self {
        Self(position)
    }

    pub fn from_cell(cell: Cell) -> Result<Self, OperandError> {
        let cell_content = cell.content();
        let pos = cell_content
            .strip_prefix(Self::PREFIX)
            .ok_or(OperandError::InvalidPointerFormat(cell.content()))?;

        let pos = Position::from_string(pos).map_err(OperandError::InvalidPointer)?;

        Ok(Self::from_position(pos))
    }

    pub fn as_cell(&self) -> Cell {
        let (x, y) = self.0.as_textual();
        Cell::new_trim(&format!("{}{}{}", Self::PREFIX, x, y))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operand {
    Literal(Literal),
    Address(Address),
    Pointer(Pointer),
}

impl Operand {
    pub fn from_cell(cell: Cell) -> Self {
        let cell_content = cell.content();

        match cell_content.chars().next() {
            Some('@') => {
                if let Ok(address) = Address::from_cell(cell.clone()) {
                    Self::Address(address)
                } else {
                    Self::Literal(Literal::from_cell(cell))
                }
            }
            Some('&') => {
                if let Ok(pointer) = Pointer::from_cell(cell.clone()) {
                    Self::Pointer(pointer)
                } else {
                    Self::Literal(Literal::from_cell(cell))
                }
            }
            _ => Self::Literal(Literal::from_cell(cell)),
        }
    }

    pub fn as_cell(&self) -> Cell {
        match self {
            Self::Literal(literal) => literal.as_cell(),
            Self::Address(address) => address.as_cell(),
            Self::Pointer(pointer) => pointer.as_cell(),
        }
    }
}
