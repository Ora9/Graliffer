use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Cell, CellError};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("invalid literal: {0}")]
pub struct LiteralFormatError(#[from] CellError);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("literal could not be parsed as bool, expected either `0` or `1` found `{got}`")]
pub struct ParseLiteralAsBoolError {
    got: String,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("literal could not be parsed as number, expected valid number found `{got}`")]
pub struct ParseLiteralAsNumberError {
    got: String,
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

    pub fn from_str(string: &str) -> Result<Self, LiteralFormatError> {
        Ok(Self::new(Cell::new(string)?))
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
    pub fn try_as_bool(&self) -> Result<bool, ParseLiteralAsBoolError> {
        match self.as_str() {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(ParseLiteralAsBoolError {
                got: self.to_string(),
            }),
        }
    }

    /// Get `Self` from a `bool`
    ///
    /// - `false` returns `0`
    /// - `true` returns `1`
    pub fn from_bool(value: bool) -> Self {
        Self::from_str_trim(match value {
            true => "1",
            false => "0",
        })
    }

    /// Return a number evaluation of `Self`
    ///
    /// # Error
    /// Returns an error if `Self` could not be parsed as a number
    pub fn try_as_number(&self) -> Result<u32, ParseLiteralAsNumberError> {
        self.as_str()
            .parse()
            .map_err(|_| ParseLiteralAsNumberError {
                got: self.to_string(),
            })
    }

    /// Get `Self` from an `u32`
    ///
    /// Trim the end of any excess of the string representation of `value`
    pub fn from_number_trim(value: u32) -> Self {
        Self::from_str_trim(&value.to_string())
    }

    /// Get `Self` from an `u32`, trimming any excess
    ///
    /// # Error
    /// Return an error if the string representation of the number could not fit in the literal,
    /// because of the [`Cell`] restrictions
    pub fn try_from_number(value: u32) -> Result<Self, LiteralFormatError> {
        Self::from_str(&value.to_string())
    }
}

impl TryFrom<u32> for Literal {
    type Error = LiteralFormatError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_number(value)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(
            Literal::from_str_trim("gri"),
            Literal::from_cell(Cell::new_trim("gri"))
        );

        assert_eq!(
            Literal::from_str_trim("excess"),
            Literal::from_cell(Cell::new_trim("excess")),
        );

        assert_eq!(
            Literal::from_str_trim("hlt").as_cell(),
            &Cell::new_trim("hlt")
        );

        assert_eq!(Literal::from_str_trim("@AA"), Cell::new_trim("@AA").into(),);
    }

    #[test]
    fn to_cell() {
        assert_eq!(Literal::from_str_trim("a").as_str(), "a");
        assert_eq!(
            String::from(Literal::from_str_trim("b").as_str()),
            Literal::from_str_trim("b").to_string()
        );
    }

    #[test]
    fn as_bool() {
        assert_eq!(Literal::from_str_trim("0").as_bool_with_defaults(), false);
        assert_eq!(Literal::from_str_trim("1").as_bool_with_defaults(), true);
        assert_eq!(Literal::from_str_trim("nop").as_bool_with_defaults(), true);
        assert_eq!(Literal::from_str_trim("@AB").as_bool_with_defaults(), true);

        assert_eq!(Literal::from_str_trim("0").try_as_bool(), Ok(false));
        assert_eq!(Literal::from_str_trim("1").try_as_bool(), Ok(true));
        assert_eq!(
            Literal::from_str_trim("gle").try_as_bool(),
            Err(ParseLiteralAsBoolError { got: "gle".into() })
        );
        assert_eq!(
            Literal::from_str_trim("05").try_as_bool(),
            Err(ParseLiteralAsBoolError { got: "05".into() })
        );
        assert_eq!(
            Literal::from_str_trim("0 ").try_as_bool(),
            Err(ParseLiteralAsBoolError { got: "0 ".into() })
        );
    }

    #[test]
    fn from_bool() {
        assert_eq!(Literal::from_bool(false).as_str(), "0");
        assert_eq!(Literal::from_bool(true).as_str(), "1");
    }

    #[test]
    fn from_number() -> Result<(), LiteralFormatError> {
        assert_eq!(Literal::try_from_number(5)?.as_str(), "5");
        assert_eq!(Literal::try_from_number(999)?.as_str(), "999");

        assert_eq!(
            Literal::try_from_number(413)?,
            Literal::from_number_trim(413)
        );

        // assert_eq!(Literal::try_from_number(1234), Err(LiteralFormatError));

        Ok(())
    }
}
