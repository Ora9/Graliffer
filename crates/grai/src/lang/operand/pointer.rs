use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Address, Cell, Grid, Literal, Operand, OperandError, Position};

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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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

    /// Get `Self` from a `&str` using the `&XY` format (see [pointer format](Pointer#Format) for
    /// more infos)
    ///
    /// # Error
    /// Returns an error if string could not be parsed as a pointer, either because it does not
    /// start with the right prefix (`&`) or because the following position is invalid,
    /// see [position representation](Position#representation) for more info
    pub fn from_str(string: &str) -> Result<Self, OperandError> {
        let pos = string
            .strip_prefix(Self::PREFIX)
            .ok_or(OperandError::InvalidPointerFormat(String::from(string)))?;

        let pos = Position::from_string(pos).map_err(OperandError::InvalidPointer)?;

        Ok(Self::from_position(pos))
    }

    /// Get `Self` from a [`Cell`] using the `&XY` format (see [pointer format](Pointer#Format) for
    /// more infos)
    ///
    /// # Error
    /// Returns an error if the `Cell` could not be parsed as a pointer, either because it does not
    /// start with the right prefix (`&`) or because the following position is invalid,
    /// see [position representation](Position#representation) for more info
    pub fn from_ref_cell(cell: &Cell) -> Result<Self, OperandError> {
        Self::from_str(cell.as_str())
    }

    /// Return a [`Cell`], using the `&XY` format, see [pointer format](Pointer#format) for more
    /// information
    pub fn to_cell(&self) -> Cell {
        let (x, y) = self.0.as_textual();
        Cell::new_trim(&format!("{}{}{}", Self::PREFIX, x, y))
    }

    /// Return a [`Literal`] from `Self`, using the `&XY` format,
    /// see [address format](Address#format) for more information
    pub fn to_literal(&self) -> Literal {
        Literal::from_cell(self.to_cell())
    }

    pub fn resolve_to_operand(&self, grid: &Grid) -> Result<Operand, OperandError> {
        fn get(
            current_pointer: Pointer,
            grid: &Grid,
            visited_cells: &mut Vec<Position>,
        ) -> Result<Operand, OperandError> {
            let next_position = current_pointer.position();
            visited_cells.push(*next_position);

            let next_cell = grid.get(*next_position);
            if let Ok(next_pointer) = Pointer::from_ref_cell(&next_cell) {
                if visited_cells.contains(next_pointer.position()) {
                    // pointer chain loop
                    Err(OperandError::PointerChainLoop {
                        last_pointer: next_pointer,
                        looping_position: *next_position,
                    })
                } else {
                    get(next_pointer, grid, visited_cells)
                }
            } else {
                Ok(Operand::from_cell(next_cell))
            }
        }

        get(*self, grid, &mut Vec::new())
    }

    pub fn resolve_to_literal(&self, grid: &Grid) -> Result<Literal, OperandError> {
        // TODO: Might induce unchecked recusrsion
        self.resolve_to_operand(grid)?.resolve_to_literal(grid)
    }

    pub fn resolve_to_address(&self, grid: &Grid) -> Result<Address, OperandError> {
        self.resolve_to_operand(grid)?.resolve_to_address(grid)
    }
}

impl Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_cell().as_str())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde_json::json;

    use crate::{OperandKind, PositionError};

    use super::*;

    pub fn create_grid_with_operand(json: serde_json::Value) -> (Grid, Operand) {
        let grid: Grid = serde_json::from_value(json).expect("must be a valid grid");
        let operand = Operand::from_cell(grid.get("AA".parse().unwrap()));

        (grid, operand)
    }

    pub fn create_grid_with_pointer(json: serde_json::Value) -> (Grid, Pointer) {
        let (grid, operand) = create_grid_with_operand(json);

        (grid, *operand.as_pointer().expect("AA must be an pointer"))
    }

    #[test]
    fn new() -> Result<(), OperandError> {
        assert_eq!(
            Pointer::from_str("&b8")?.position(),
            &Position::from_string("b8").unwrap()
        );

        assert_eq!(
            Pointer::from_ref_cell(&Cell::new_trim("&x+"))?,
            Pointer::from_str("&x+")?
        );

        assert_eq!(
            Pointer::from_position(Position::from_string("uu").unwrap()),
            Pointer::from_str("&uu")?
        );

        Ok(())
    }

    #[test]
    fn parse() -> Result<(), OperandError> {
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
    fn to_cell() -> Result<(), OperandError> {
        assert_eq!(Pointer::from_str("&a5")?.to_string(), String::from("&a5"));

        let pointer = Pointer::from_str("&oO")?;
        assert_eq!(pointer.to_cell().to_string(), pointer.to_string());

        Ok(())
    }

    #[test]
    fn resolve_to_literal() -> Result<(), OperandError> {
        let (grid, pointer) = create_grid_with_pointer(json!({
            "AA": "&AB",
            "AB": "pwt",
        }));
        assert_eq!(
            pointer.resolve_to_literal(&grid)?,
            Literal::from_str_trim("pwt").into(),
        );

        let (grid, pointer) = create_grid_with_pointer(json!({
            "AA": "&AB",
            "AB": "@AC",
            "AC": "0"
        }));
        assert_eq!(
            pointer.resolve_to_literal(&grid)?,
            Literal::from_str("0")?.into(),
        );

        let (grid, pointer) = create_grid_with_pointer(json!({
            "AA": "&AB",
            "AB": "@AC",
            "AC": "@di"
        }));
        assert_eq!(
            pointer.resolve_to_literal(&grid)?,
            Literal::from_str("@di")?.into(),
        );

        Ok(())
    }

    #[test]
    fn resolve_to_operand() -> Result<(), OperandError> {
        let (grid, pointer) = create_grid_with_pointer(json!({
            "AA": "&AB",
            "AB": "pwt",
        }));
        assert_eq!(
            pointer.resolve_to_operand(&grid)?,
            Literal::from_str_trim("pwt").into(),
        );

        let (grid, pointer) = create_grid_with_pointer(json!({
            "AA": "&AB",
            "AB": "@AC",
        }));
        assert_eq!(
            pointer.resolve_to_operand(&grid)?,
            Address::from_str("@AC")?.into(),
        );

        let (grid, pointer) = create_grid_with_pointer(json!({
            "AA": "&AB",
            "AB": "&AC",
            "AC": "&AD",
            "AD": "@pr",
        }));
        assert_eq!(
            pointer.resolve_to_operand(&grid)?,
            Address::from_str("@pr")?.into(),
        );

        Ok(())
    }

    #[test]
    fn resolve_to_address() -> Result<(), OperandError> {
        let (grid, pointer) = create_grid_with_pointer(json!({
            "AA": "&AB",
            "AB": "@d5",
        }));
        assert_eq!(
            pointer.resolve_to_address(&grid)?,
            Address::from_str("@d5")?
        );

        let (grid, pointer) = create_grid_with_pointer(json!({
            "AA": "&AB",
            "AB": "&AC",
            "AC": "@pa",
        }));
        assert_eq!(
            pointer.resolve_to_address(&grid)?,
            Address::from_str("@pa")?
        );

        let (grid, pointer) = create_grid_with_pointer(json!({
            "AA": "&AB",
            "AB": "&AC",
            "AC": "prt",
        }));
        assert_eq!(
            pointer.resolve_to_address(&grid),
            Err(OperandError::CouldNotResolveAsAddress {
                operand_kind: OperandKind::Literal,
                got: String::from("prt")
            })
        );

        Ok(())
    }

    #[test]
    fn resolve_loop() -> Result<(), OperandError> {
        let (grid, pointer) = create_grid_with_pointer(json!({
            "AA": "&AB",
            "AB": "&AC",
            "AC": "&AD",
            "AD": "&AE",
            "AE": "&AA",
        }));
        assert_eq!(
            pointer.resolve_to_operand(&grid),
            Err(OperandError::PointerChainLoop {
                last_pointer: Pointer::from_str("&AB")?.into(),
                looping_position: Position::from_str("AA").unwrap()
            }),
        );

        Ok(())
    }
}
