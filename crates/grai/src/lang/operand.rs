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

/// An `Address` contains a [`Position`] and can be used by operations in two ways :
/// - To designate a certain [`Cell`] in the grid, on wich the operation can act
///   (e.g the operation `set` takes an address that designate the cell to change)
/// - To reference another [`Cell`]'s contained [`Literal`] (more frequent use case)
///   (e.g the operation `add` needs two literal, any one of the two operands can be an address
///   pointing to a literal, allowing for a level of indirection).
///
/// Note that in the second case, any designated cell will *not* be interpreted further, that mean
/// that if we reference a cell containing another address (or any other special operand) the
/// operation will use that cell as a literal without any interpretation.
/// [`Pointer`]s exists just for this purpose, to allow for more that one indirection.
///
/// # Format
/// An address must be in format : `@XY`, with :
/// - `@` being a prefix (wich denote an address, e.g. a `&` would denote a [`Pointer`])
/// - `X` and `Y` being respectively the horizontal and vertical axis of a [`Position`] in textual
/// form, see [position representation](Position#representation) for more informations
///
/// Example : `@AB`, `@Q+` or `@8a`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Address(Position);

impl Address {
    const PREFIX: char = '@';

    /// Get `Self` from a [`Position`]
    pub fn from_position(position: Position) -> Self {
        Self(position)
    }

    /// Return the designated [`Position`]
    pub fn position(&self) -> &Position {
        &self.0
    }

    /// Get `Self` from a [`Cell`] using the `@XY` format (see [address format](Address#Format) for
    /// more infos)
    ///
    /// # Error
    /// Returns an error if the `Cell` could not be parsed as an address, either because it does not
    /// start with the right prefix (`@`) or because the following position is invalid,
    /// see [position representation](Position#representation) for more info
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

    /// Return a [`Cell`], using the `@XY` format, see [address format](Address#format) for more
    /// information
    pub fn to_cell(&self) -> Cell {
        let (x, y) = self.position().as_textual();
        Cell::new_trim(&format!("{}{}{}", Self::PREFIX, x, y))
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_cell().as_str())
    }
}

/// A `Pointer` contains a [`Position`] and can be used in operations to reference another
/// [`Cell`]'s operand, that will then be interpreted.
/// Mutltiples pointers can be chained to allow for more level of indirectio, and complex data
/// references.
///
/// # Format
/// A pointer must be in format : `&XY`, with :
/// - `&` being a prefix (wich denote a pointer, e.g. a `@` would denote an [`Address`]).
/// - `X` and `Y` being respectively the horizontal and vertical axis of a [`Position`] in textual
/// form, see [position representation](Position#representation) for more informations
///
/// Example : `&AB`, `&Q+` or `&8a`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pointer(Position);

impl Pointer {
    const PREFIX: char = '&';

    /// Get `Self` from a [`Position`]
    pub fn from_position(position: Position) -> Self {
        Self(position)
    }

    /// Return the designated [`Position`]
    pub fn position(&self) -> &Position {
        &self.0
    }

    /// Get `Self` from a [`Cell`] using the `&XY` format (see [pointer format](Pointer#Format) for
    /// more infos)
    ///
    /// # Error
    /// Returns an error if the `Cell` could not be parsed as a pointer, either because it does not
    /// start with the right prefix (`&`) or because the following position is invalid,
    /// see [position representation](Position#representation) for more info
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

    /// Return a [`Cell`], using the `&XY` format, see [pointer format](Pointer#format) for more
    /// information
    pub fn to_cell(&self) -> Cell {
        let (x, y) = self.0.as_textual();
        Cell::new_trim(&format!("{}{}{}", Self::PREFIX, x, y))
    }
}

impl Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_cell().as_str())
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

    #[test]
    fn parse_address() -> Result<(), OperandError> {
        assert_eq!(
            Address::from_ref_cell(&Cell::new_trim("@yA"))?.position(),
            &Position::from_string("yA").unwrap()
        );

        assert_eq!(
            Address::from_ref_cell(&Cell::new_trim("@/+"))?.position(),
            &Position::from_string("/+").unwrap()
        );

        assert_eq!(
            Address::from_ref_cell(&Cell::new_trim(" @A")),
            Err(OperandError::InvalidAddressFormat(" @A".to_string()))
        );

        assert_eq!(
            Address::from_ref_cell(&Cell::new_trim("&AA")),
            Err(OperandError::InvalidAddressFormat("&AA".to_string()))
        );

        assert_eq!(
            Address::from_ref_cell(&Cell::new_trim("AA")),
            Err(OperandError::InvalidAddressFormat("AA".to_string()))
        );

        assert_eq!(
            Address::from_ref_cell(&Cell::new_trim("@p")),
            Err(OperandError::InvalidAddress(PositionError::WrongFormat(
                "p".to_string()
            )))
        );

        Ok(())
    }

    #[test]
    fn address_to_cell() {
        assert_eq!(
            Address::from_position(Position::from_string("a5").unwrap()).to_string(),
            String::from("@a5")
        );

        let address = Address::from_position(Position::from_string("oO").unwrap());
        assert_eq!(address.to_cell().to_string(), address.to_string());
    }

    #[test]
    fn parse_pointer() -> Result<(), OperandError> {
        assert_eq!(
            Pointer::from_ref_cell(&Cell::new_trim("&yA"))?.position(),
            &Position::from_string("yA").unwrap()
        );

        assert_eq!(
            Pointer::from_ref_cell(&Cell::new_trim("&/+"))?.position(),
            &Position::from_string("/+").unwrap()
        );

        assert_eq!(
            Pointer::from_ref_cell(&Cell::new_trim(" &A")),
            Err(OperandError::InvalidPointerFormat(" &A".to_string()))
        );

        assert_eq!(
            Pointer::from_ref_cell(&Cell::new_trim("@AA")),
            Err(OperandError::InvalidPointerFormat("@AA".to_string()))
        );

        assert_eq!(
            Pointer::from_ref_cell(&Cell::new_trim("AA")),
            Err(OperandError::InvalidPointerFormat("AA".to_string()))
        );

        assert_eq!(
            Pointer::from_ref_cell(&Cell::new_trim("&p")),
            Err(OperandError::InvalidPointer(PositionError::WrongFormat(
                "p".to_string()
            )))
        );

        Ok(())
    }

    #[test]
    fn pointer_to_cell() {
        assert_eq!(
            Pointer::from_position(Position::from_string("a5").unwrap()).to_string(),
            String::from("&a5")
        );

        let pointer = Pointer::from_position(Position::from_string("oO").unwrap());
        assert_eq!(pointer.to_cell().to_string(), pointer.to_string());
    }
}
