use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{Cell, Grid, Position, PositionError};

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

    #[error("could not resolve pointer chain, loop at `{looping_position}`")]
    PointerChainLoop {
        last_operand: Operand,
        looping_position: Position,
    },

    #[error("could not resolve to address, got {operand} : `{got}`")]
    CouldNotResolveAsAddress { operand: OperandKind, got: String },
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
    pub fn from_str(string: &str) -> Result<Self, OperandError> {
        let pos = string
            .strip_prefix(Self::PREFIX)
            .ok_or(OperandError::InvalidAddressFormat(String::from(string)))?;

        let pos = Position::from_string(pos).map_err(OperandError::InvalidAddress)?;

        Ok(Self::from_position(pos))
    }

    /// Get `Self` from a [`Cell`] using the `@XY` format (see [address format](Address#Format) for
    /// more infos)
    ///
    /// # Error
    /// Returns an error if the `Cell` could not be parsed as an address, either because its
    /// content does not start with the right prefix (`@`) or because the following position is
    /// invalid, see [position representation](Position#representation) for more info
    pub fn from_ref_cell(cell: &Cell) -> Result<Self, OperandError> {
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
            current_pos: Position,
            grid: &Grid,
            visited_cells: &mut Vec<Position>,
        ) -> Result<Operand, OperandError> {
            visited_cells.push(current_pos);
            let current_cell = grid.get(current_pos);

            if let Ok(next_pointer) = Pointer::from_ref_cell(&current_cell) {
                if visited_cells.contains(next_pointer.position()) {
                    // pointer chain loop
                    Err(OperandError::PointerChainLoop {
                        last_operand: Operand::from_cell(current_cell),
                        looping_position: current_pos,
                    })
                } else {
                    get(*next_pointer.position(), grid, visited_cells)
                }
            } else {
                Ok(Operand::from_cell(current_cell))
            }
        }

        let mut visited_cells = Vec::default();
        get(*self.position(), grid, &mut visited_cells)
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
    pub fn as_literal(self) -> Option<Literal> {
        match self {
            Self::Literal(literal) => Some(literal),
            _ => None,
        }
    }

    /// Get an [`Address`] without any conversion
    pub fn as_address(self) -> Option<Address> {
        match self {
            Self::Address(address) => Some(address),
            _ => None,
        }
    }

    /// Get [`Pointer`] without any conversion
    pub fn as_pointer(self) -> Option<Pointer> {
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
                operand: OperandKind::Literal,
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

#[cfg(test)]
mod tests {
    mod helper {
        use super::*;
        pub fn create_grid_with_operand(json: serde_json::Value) -> (Grid, Operand) {
            let grid: Grid = serde_json::from_value(json).expect("must be a valid grid");
            let operand = Operand::from_cell(grid.get("AA".parse().unwrap()));

            (grid, operand)
        }

        pub fn create_grid_with_literal(json: serde_json::Value) -> (Grid, Literal) {
            let (grid, operand) = create_grid_with_operand(json);

            (grid, operand.as_literal().expect("AA must be a literal"))
        }

        pub fn create_grid_with_address(json: serde_json::Value) -> (Grid, Address) {
            let (grid, operand) = create_grid_with_operand(json);

            (grid, operand.as_address().expect("AA must be an address"))
        }

        pub fn create_grid_with_pointer(json: serde_json::Value) -> (Grid, Pointer) {
            let (grid, operand) = create_grid_with_operand(json);

            (grid, operand.as_pointer().expect("AA must be an pointer"))
        }
    }

    use serde_json::json;

    use super::*;
    use helper::*;

    #[test]
    fn new_literal() {
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
    fn literal_string() {
        assert_eq!(Literal::from_str_trim("a").as_str(), "a");
        assert_eq!(
            String::from(Literal::from_str_trim("b").as_str()),
            Literal::from_str_trim("b").to_string()
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
    fn new_address() -> Result<(), OperandError> {
        assert_eq!(
            Address::from_str("&PO")?.position(),
            &Position::from_string("PO").unwrap()
        );
        assert_eq!(
            Address::from_ref_cell(&Cell::new_trim("&a9"))?,
            Address::from_str("&a9")?,
        );

        assert_eq!(
            Address::from_position(Position::from_string("/j").unwrap()),
            Address::from_str("&/j")?,
        );

        Ok(())
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
    fn address_to_cell() -> Result<(), OperandError> {
        assert_eq!(Address::from_str("@a5")?.to_string(), String::from("@a5"));

        let address = Address::from_str("oO")?;
        assert_eq!(address.to_cell().to_string(), address.to_string());

        Ok(())
    }

    #[test]
    fn address_fetch_literal() {
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
    fn address_fetch_operand() {
        let (grid, addr) = create_grid_with_address(json!({
            "AA": "@Bl",
            "Bl": "abc"
        }));
        assert_eq!(
            addr.fetch_operand(&grid).as_literal().unwrap(),
            addr.fetch_literal(&grid)
        );

        let (grid, addr) = create_grid_with_address(json!({
            "AA": "@Kr",
            "Kr": "@pm"
        }));
        assert_eq!(
            addr.fetch_operand(&grid),
            Address::from_str("&pm").unwrap().into()
        );
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
