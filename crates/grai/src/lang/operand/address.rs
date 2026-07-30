use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Cell, Grid, Literal, Operand, Position, PositionError};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum AddressParseError {
    #[error("invalid address format: expected to find format `@XY`, found `{got}`")]
    InvalidFormat { got: String },

    #[error(transparent)]
    Position(#[from] PositionError),
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

    /// Get `Self` from a `&str` using the `@XY` format (see [address format](Address#Format) for
    /// more infos)
    ///
    /// # Error
    /// Returns an error if the string could not be parsed as an address, either because it does not
    /// start with the right prefix (`@`) or because the following position is invalid,
    /// see [position representation](Position#representation) for more info
    pub fn from_str(string: &str) -> Result<Self, AddressParseError> {
        let pos = string
            .strip_prefix(Self::PREFIX)
            .ok_or(AddressParseError::InvalidFormat {
                got: string.to_string(),
            })?;

        let pos = Position::from_string(pos)?;

        Ok(Self::from_position(pos))
    }

    /// Get `Self` from a [`Cell`] using the `@XY` format (see [address format](Address#Format) for
    /// more infos)
    ///
    /// # Error
    /// Returns an error if the `Cell` could not be parsed as an address, either because its
    /// content does not start with the right prefix (`@`) or because the following position is
    /// invalid, see [position representation](Position#representation) for more info
    pub fn from_ref_cell(cell: &Cell) -> Result<Self, AddressParseError> {
        Self::from_str(cell.as_str())
    }

    /// Return a [`Cell`] from `Self`, using the `@XY` format,
    /// see [address format](Address#format) for more information
    pub fn to_cell(&self) -> Cell {
        let (x, y) = self.position().as_textual();
        Cell::new_trim(&format!("{}{}{}", Self::PREFIX, x, y))
    }

    /// Return a [`Literal`] from `Self`, using the `@XY` format, see [address format](Address#format) for more
    /// information
    pub fn as_literal(&self) -> Literal {
        Literal::from_cell(self.to_cell())
    }

    /// Fetch the designated [`Operand`] in a [`Grid`]
    pub fn fetch_operand(&self, grid: &Grid) -> Operand {
        Operand::from_cell(grid.get(*self.position()))
    }

    /// Fetch the designated [`Literal`] in a [`Grid`]
    pub fn fetch_literal(&self, grid: &Grid) -> Literal {
        Literal::from_cell(grid.get(*self.position()))
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_cell().as_str())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::PositionError;

    use super::*;

    pub fn create_grid_with_operand(json: serde_json::Value) -> (Grid, Operand) {
        let grid: Grid = serde_json::from_value(json).expect("must be a valid grid");
        let operand = Operand::from_cell(grid.get("AA".parse().unwrap()));

        (grid, operand)
    }

    pub fn create_grid_with_address(json: serde_json::Value) -> (Grid, Address) {
        let (grid, operand) = create_grid_with_operand(json);

        (grid, *operand.as_address().expect("AA must be an address"))
    }

    #[test]
    fn new() -> Result<(), AddressParseError> {
        assert_eq!(
            Address::from_str("@PO")?.position(),
            &Position::from_string("PO").unwrap()
        );

        assert_eq!(
            Address::from_ref_cell(&Cell::new_trim("@a9"))?,
            Address::from_str("@a9")?,
        );

        assert_eq!(
            Address::from_position(Position::from_string("/j").unwrap()),
            Address::from_str("@/j")?,
        );

        Ok(())
    }

    #[test]
    fn parse() -> Result<(), AddressParseError> {
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
            Err(AddressParseError::InvalidFormat {
                got: " @A".to_string()
            })
        );

        assert_eq!(
            Address::from_ref_cell(&Cell::new_trim("&AA")),
            Err(AddressParseError::InvalidFormat {
                got: "&AA".to_string()
            })
        );

        assert_eq!(
            Address::from_ref_cell(&Cell::new_trim("AA")),
            Err(AddressParseError::InvalidFormat {
                got: "AA".to_string()
            })
        );

        assert_eq!(
            Address::from_ref_cell(&Cell::new_trim("@p")),
            Err(AddressParseError::Position(PositionError::WrongFormat(
                "p".to_string()
            )))
        );

        Ok(())
    }

    #[test]
    fn to_cell() {
        assert_eq!(
            Address::from_str("@a5").unwrap().to_string(),
            String::from("@a5")
        );

        let address = Address::from_str("@oO").unwrap();
        assert_eq!(address.to_cell().to_string(), address.to_string());
    }

    #[test]
    fn fetch_literal() {
        let (grid, addr) = create_grid_with_address(json!({
            "AA": "@AB",
            "AB": "abc"
        }));
        assert_eq!(addr.fetch_literal(&grid), Literal::from_str_trim("abc"));

        let (grid, addr) = create_grid_with_address(json!({
            "AA": "@aa",
            "aa": "@AA"
        }));
        assert_eq!(addr.fetch_literal(&grid), Literal::from_str_trim("@AA"));

        let (grid, addr) = create_grid_with_address(json!({
            "AA": "@AB",
        }));
        assert_eq!(addr.fetch_literal(&grid), Literal::from_str_trim(""));
    }

    #[test]
    fn fetch_operand() {
        let (grid, addr) = create_grid_with_address(json!({
            "AA": "@Bl",
            "Bl": "abc"
        }));
        assert_eq!(
            *addr.fetch_operand(&grid).as_literal().unwrap(),
            addr.fetch_literal(&grid)
        );

        let (grid, addr) = create_grid_with_address(json!({
            "AA": "@Kr",
            "Kr": "@pm"
        }));
        assert_eq!(
            addr.fetch_operand(&grid),
            Address::from_str("@pm").unwrap().into()
        );
    }
}
