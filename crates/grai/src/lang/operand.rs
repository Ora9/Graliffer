use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Cell, CellError, Grid, Position, PositionError};

mod address;
pub use address::Address;

mod literal;
pub use literal::Literal;

mod pointer;
pub use pointer::Pointer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperandKind {
    Literal,
    Address,
    Pointer,
}

impl Display for OperandKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal => f.write_str("literal"),
            Self::Address => f.write_str("address"),
            Self::Pointer => f.write_str("pointer"),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum OperandError {
    #[error("invalid literal: `{0}`")]
    InvalidLiteralFormat(#[source] CellError),

    #[error("literal could not be parsed as bool, expected either `0` or `1` found `{0}`")]
    CouldNotParseLiteralAsBool(String),

    #[error("literal could not be parsed as number, expected valid number, found `{0}`")]
    CouldNotParseLiteralAsNumber(String),

    #[error("invalid address: expected to find format `@XY`, found `{0}`")]
    InvalidAddressFormat(String),

    #[error("invalid address: `{0}`")]
    InvalidAddress(#[source] PositionError),

    #[error("invalid pointer: expected to find format `&XY`, found `{0}`")]
    InvalidPointerFormat(String),

    #[error("invalid pointer: `{0}`")]
    InvalidPointer(#[source] PositionError),

    #[error("could not resolve pointer chain, loop at `{looping_position}`")]
    PointerChainLoop {
        last_pointer: Pointer,
        looping_position: Position,
    },

    #[error("could not resolve to address, got {operand_kind} : `{got}`")]
    CouldNotResolveAsAddress {
        operand_kind: OperandKind,
        got: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Operand {
    Literal(Literal),
    Address(Address),
    Pointer(Pointer),
}

impl Operand {
    pub fn from_cell(cell: Cell) -> Self {
        if let Ok(address) = Address::from_ref_cell(&cell) {
            Self::Address(address)
        } else if let Ok(pointer) = Pointer::from_ref_cell(&cell) {
            Self::Pointer(pointer)
        } else {
            Self::Literal(Literal::from_cell(cell))
        }
    }

    pub fn to_cell(&self) -> Cell {
        match self {
            Self::Literal(literal) => literal.as_cell().clone(),
            Self::Address(address) => address.to_cell(),
            Self::Pointer(pointer) => pointer.to_cell(),
        }
    }

    /// Get a [`Literal`] without any conversion
    pub fn as_literal(&self) -> Option<&Literal> {
        match self {
            Self::Literal(literal) => Some(literal),
            _ => None,
        }
    }

    /// Get an [`Address`] without any conversion
    pub fn as_address(&self) -> Option<&Address> {
        match self {
            Self::Address(address) => Some(address),
            _ => None,
        }
    }

    /// Get [`Pointer`] without any conversion
    pub fn as_pointer(&self) -> Option<&Pointer> {
        match self {
            Self::Pointer(pointer) => Some(pointer),
            _ => None,
        }
    }

    pub fn resolve_to_literal(&self, grid: &Grid) -> Result<Literal, OperandError> {
        match self {
            Self::Literal(literal) => Ok(literal.clone()),
            Self::Address(address) => Ok(address.fetch_literal(grid)),
            Self::Pointer(pointer) => pointer.resolve_to_literal(grid),
        }
    }

    pub fn resolve_to_address(&self, grid: &Grid) -> Result<Address, OperandError> {
        match self {
            Self::Literal(literal) => Err(OperandError::CouldNotResolveAsAddress {
                operand_kind: OperandKind::Literal,
                got: literal.to_string(),
            }),
            Self::Address(address) => Ok(*address),
            Self::Pointer(pointer) => pointer.resolve_to_address(grid),
        }
    }
}

impl From<Literal> for Operand {
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

impl From<Address> for Operand {
    fn from(value: Address) -> Self {
        Self::Address(value)
    }
}

impl From<Pointer> for Operand {
    fn from(value: Pointer) -> Self {
        Self::Pointer(value)
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::Address(address) => write!(f, "{address}"),
            Self::Pointer(pointer) => write!(f, "{pointer}"),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     mod helper {
//         use super::*;

//     }
// }
