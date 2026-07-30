use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Cell, CellError, Grid};

mod address;
pub use address::*;

mod literal;
pub use literal::*;

mod pointer;
pub use pointer::*;

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
#[error("expected an address, got a literal `{got}`")]
pub struct NotAnAddress {
    got: Literal,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ResolveToAddressError {
    #[error(transparent)]
    PointerLoop(#[from] PointerLoopError),

    #[error(transparent)]
    NotAnAddress(#[from] NotAnAddress),
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("invalid operand: {0}")]
pub struct OperandFormatError(#[from] CellError);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "String")]
#[serde(into = "String")]
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

    pub fn from_str(string: &str) -> Result<Self, OperandFormatError> {
        Ok(Self::from_cell(Cell::new(string)?))
    }

    pub fn from_str_trim(string: &str) -> Self {
        Self::from_cell(Cell::new_trim(string))
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

    pub fn resolve_to_literal(&self, grid: &Grid) -> Result<Literal, PointerLoopError> {
        match self {
            Self::Literal(literal) => Ok(literal.clone()),
            Self::Address(address) => Ok(address.fetch_literal(grid)),
            Self::Pointer(pointer) => pointer.resolve_to_literal(grid),
        }
    }

    pub fn resolve_to_address(&self, grid: &Grid) -> Result<Address, ResolveToAddressError> {
        match self {
            Self::Literal(literal) => Err(NotAnAddress {
                got: literal.clone(),
            }
            .into()),
            Self::Address(address) => Ok(*address),
            Self::Pointer(pointer) => pointer.resolve_to_address(grid),
        }
    }
}

impl From<String> for Operand {
    fn from(value: String) -> Self {
        Self::from_cell(Cell::new_trim(&value))
    }
}

impl From<Operand> for String {
    fn from(value: Operand) -> Self {
        value.to_string()
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
