use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Cell, Position, PositionError};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum OperandError {
    #[error("literal could not be parsed as bool, expected either `0` or `1` found `{0}`")]
    LiteralCouldNotBeParsedAsBool(String),

    #[error("invalid address, expected to find format `@XY`, found `{0}`")]
    InvalidAddressFormat(String),

    #[error("invalid address, `{0}`")]
    InvalidAddress(#[source] PositionError),

    #[error("invalid pointer, expected to find format `&XY`, found `{0}`")]
    InvalidPointerFormat(String),

    #[error("invalid pointer, `{0}`")]
    InvalidPointer(#[source] PositionError),
}

/// A `Literal` is a string of character that represents data
///
/// It is the default operand (as in : when parsing a cell, a literal is the unfallible fallback)
///
/// Differents operations can interpret a literal in differents ways :
/// - As bool (see [`from_bool()`], [`as_bool_with_defaults()`] or [`try_as_bool()`])
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Literal(Cell);

impl Literal {
    pub fn new(cell: Cell) -> Self {
        Self(cell)
    }

    pub fn from_cell(cell: Cell) -> Self {
        Self::new(cell)
    }

    pub fn from_str_trim(string: &str) -> Self {
        Self::new(Cell::new_trim(string))
    }

    pub fn as_cell(&self) -> &Cell {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        self.as_cell().as_str()
    }

    /// Return a boolean evaluation of `Self`, defaulting to `true`
    ///
    /// - `0` returns `false`
    /// - Anything else returns `true`
    pub fn as_bool_with_defaults(&self) -> bool {
        self.as_str() != "0"
    }

    /// Return a boolean evaluation of `Self`
    ///
    /// - `0` returns `Ok(false)`
    /// - `1` returns `Ok(true)`
    /// - Anything else returns an `Err`
    pub fn try_as_bool(&self) -> Result<bool, OperandError> {
        match self.as_str() {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(OperandError::LiteralCouldNotBeParsedAsBool(
                self.to_string(),
            )),
        }
    }

    /// Get `Self` from a `bool`
    ///
    /// - `false` returns `0`
    /// - `true` returns `1`
    pub fn from_bool(value: bool) -> Self {
        Self::from_cell(Cell::new_trim(match value {
            true => "1",
            false => "0",
        }))
    }
}

impl From<Cell> for Literal {
    fn from(value: Cell) -> Self {
        Literal::new(value)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address(Position);

impl Address {
    const PREFIX: char = '@';

    pub fn from_position(position: Position) -> Self {
        Self(position)
    }

    pub fn from_ref_cell(cell: &Cell) -> Result<Self, OperandError> {
        let pos =
            cell.as_str()
                .strip_prefix(Self::PREFIX)
                .ok_or(OperandError::InvalidAddressFormat(String::from(
                    cell.as_str(),
                )))?;

        let pos = Position::from_string(pos).map_err(OperandError::InvalidAddress)?;

        Ok(Self::from_position(pos))
    }

    pub fn to_cell(&self) -> Cell {
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

    pub fn from_ref_cell(cell: &Cell) -> Result<Self, OperandError> {
        let pos =
            cell.as_str()
                .strip_prefix(Self::PREFIX)
                .ok_or(OperandError::InvalidPointerFormat(String::from(
                    cell.as_str(),
                )))?;

        let pos = Position::from_string(pos).map_err(OperandError::InvalidPointer)?;

        Ok(Self::from_position(pos))
    }

    pub fn to_cell(&self) -> Cell {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_literal() {
        assert_eq!(
            Literal::new(Cell::new_trim("hlt")).as_cell(),
            &Cell::new_trim("hlt")
        );

        assert_eq!(
            Literal::new(Cell::new_trim("gri")),
            Literal::from_cell(Cell::new_trim("gri"))
        );

        assert_eq!(
            Literal::new(Cell::new_trim("@AA")),
            Cell::new_trim("@AA").into(),
        );

        assert_eq!(
            Literal::new(Cell::new_trim("&PB")),
            Literal::from_str_trim("&PB"),
        );

        assert_eq!(
            Literal::from_cell(Cell::new_trim("excess")),
            Literal::from_str_trim("excess"),
        );
    }

    #[test]
    fn string_conversion() {
        assert_eq!(Literal::from_str_trim("a").as_str(), "a");
        assert_eq!(
            String::from(Literal::from_str_trim("a").as_str()),
            Literal::from_str_trim("a").to_string()
        );
    }

    #[test]
    fn literal_as_bool() {
        assert_eq!(Literal::from_str_trim("0").as_bool_with_defaults(), false);
        assert_eq!(Literal::from_str_trim("1").as_bool_with_defaults(), true);
        assert_eq!(Literal::from_str_trim("nop").as_bool_with_defaults(), true);
        assert_eq!(Literal::from_str_trim("@AB").as_bool_with_defaults(), true);

        assert_eq!(Literal::from_str_trim("0").try_as_bool(), Ok(false));
        assert_eq!(Literal::from_str_trim("1").try_as_bool(), Ok(true));
        assert_eq!(
            Literal::from_str_trim("gle").try_as_bool(),
            Err(OperandError::LiteralCouldNotBeParsedAsBool("gle".into()))
        );
        assert_eq!(
            Literal::from_str_trim("05").try_as_bool(),
            Err(OperandError::LiteralCouldNotBeParsedAsBool("05".into()))
        );
        assert_eq!(
            Literal::from_str_trim("0 ").try_as_bool(),
            Err(OperandError::LiteralCouldNotBeParsedAsBool("0 ".into()))
        );
    }

    #[test]
    fn literal_from_bool() {
        assert_eq!(Literal::from_bool(false).as_str(), "0");
        assert_eq!(Literal::from_bool(true).as_str(), "1");
    }
}
