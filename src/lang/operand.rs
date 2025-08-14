use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};

use crate::grid::{Grid, Cell, Position};

/// A `Literal` is a string of character that represents data
/// It can contain any unicode characters
/// Different operations can interpret this string in different ways :
/// - as string
/// - as number, parsed as
/// - as boolean, can either be `false` when the cell's content strictly equal `0`, anything other is considered `true`, a common
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    value: Cell,
}

impl Literal {
    /// Get a `Literal` from a [`Cell`]
    pub fn new(value: Cell) -> Self {
        Self {
            value
        }
    }

    /// Get a `Literal` from a [`Cell`]
    pub fn from_cell(cell: Cell) -> Self {
        Self::new(cell)
    }

    /// Get a `Literal` from a `bool`, where :
    /// - `false` equals `0`
    /// - `true` equals `1`
    pub fn from_bool(value: bool) -> Self {
        Self::from_cell(Cell::new_trim(
            match value {
                true => "1",
                false => "0"
            }
        ))
    }

    pub fn from_number(value: u32) -> Self {
        Self::from_cell(Cell::new_trim(
            value.to_string().as_str()
        ))
    }

    /// Get a [`Cell`] from a `Literal`
    pub fn as_cell(&self) -> Cell {
        self.value.clone()
    }

    /// Return a boolean evaluation of a `Literal` :
    ///
    /// - `0` return `Ok(false)`
    /// - `1` return `Ok(true)`
    /// - Anything else return an `Err`
    pub fn as_bool(&self) -> Result<bool, anyhow::Error> {
        match self.value.content().as_str() {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(anyhow!("Could not parse the operand as a bool")),
        }
    }

    /// Return a boolean evaluation of a `Literal`, with a default to true :
    ///
    /// - `0` return `false`
    /// - Anything else return `true`
    pub fn as_bool_with_default(&self) -> bool {
        self.value.content() != "0"
    }

    /// Return a numeric evaluation of a `Literal`
    ///
    /// Return an `Err` if contained value couldn't be parsed a numeric
    pub fn as_numeric(&self) -> Result<u32, anyhow::Error> {
        self.value.content()
            .parse()
            .context("Could not parse the operand as a number")
    }

    /// Return a numeric evaluation of a `Literal`, with a default to `0`
    ///
    /// Return `0` as a default if contained value couldn't be parsed as numeric
    pub fn as_numeric_with_default(&self) -> u32 {
        self.value.content().parse().unwrap_or(0)
    }

}

/// An `Address` contain a [`Position`] and can be used by operations in
/// two ways, either :
/// - To designate a certain [`Cell`] in the grid, on wich the operation can act on
/// (e.g. the operation `set` uses an address to change the content of
/// the designated cell).
/// - More frequently, to reference another [`Cell`]'s [`Literal`].
/// (e.g. the operation `add` needs two literal, any one of these two can
/// actually be an address pointing to a literal, allowing for
/// more complex programs).
///
/// Note that, in the second case, any designated cell will not
/// be interpreted further, that mean that if we reference a cell containing
/// another address (or anything other special operand), the operation will use
/// this cell as a literal without any interpretation.
/// [`Pointer`]s exists just for this purpose. to allow more than one delegation.
///
/// # Format
/// An address must be formated like `@XY`, with :
/// - `@` being a prefix (wich denote an address, a `&` would denote
/// a [`Pointer`])
/// - `X` and `Y` being respectively the horizontal and vertical axis of a
/// [`Position`] in textual form, see
/// [position representation](Position#representation) for more information
///
/// Example : `@AB` or `@Q+`
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Address {
    pub position: Position
}

impl Address {
    const PREFIX: char = '@';

    /// Get an `Address` from a [`Position`]
    pub fn from_position(position: &Position) -> Self {
        Self {
            position: *position
        }
    }

    /// Get an `Address` from a [`Cell`] using the `@XY` format, see
    /// [address format](Address#format) for more information
    ///
    /// # Error
    /// Return an error if the `Cell` could not be parsed, either because it
    /// does not start with the right prefix (`@`) or because the following
    /// position is invalid, see
    /// [position representation](Position#representation) for more information
    pub fn from_cell(cell: &Cell) -> Result<Self, anyhow::Error> {
        let cell_content = cell.content();
        let pos = cell_content.strip_prefix(Self::PREFIX)
            .ok_or(anyhow!(format!("{} is not a valid `Address` format", cell_content)))?;
        let pos = Position::try_from(pos)?;

        Ok(Self::from_position(&pos))
    }

    /// Return a [`Cell`] from an `Address`, using the `@XY` format,
    /// see [address format](Address#format) for more information
    pub fn as_cell(&self) -> Cell {
        let (x, y) = self.position.as_textual();
        Cell::new(
            format!("{}{}{}", Self::PREFIX, x, y).as_str()
        ).expect("A Position must always have a valid Cell representation")
    }

    /// Return a [`Literal`] from an `Address`, using the `@XY` format,
    /// see [address format](Address#format) for more information
    pub fn as_literal(&self) -> Literal {
        Literal::from_cell(self.as_cell())
    }

    pub fn fetch_operand(&self, grid: &Grid) -> Operand {
        Operand::from_cell(grid.get(self.position))
    }

    /// Fetch the literal referenced by the `Address`
    pub fn fetch_literal(&self, grid: &Grid) -> Literal {
        Literal::from_cell(grid.get(self.position))
    }
}

/// A `Pointer` contain a [`Position`] and can be used in operation to reference
/// another [`Cell`]'s operand that will then be interpreted.
/// Multiple pointers can be chained to allow complex data references
///
/// # Format
/// An pointer must be formated like `&XY`, with :
/// - `&` being a prefix (wich denote an address, a `@` would denote
/// an [`Address`])
/// - `X` and `Y` being respectively the horizontal and vertical axis of a
/// [`Position`] in textual form, see
/// [position representation](Position#representation) for more information
///
/// Example : `&AB` or `&Q+`
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pointer {
    position: Position,
}

impl Pointer {
    const PREFIX: char = '&';
    // TODO : Augment the depth to something like 64 idk
    const MAX_RECURSION_DEPTH: u32 = 3;

    /// Get an `Pointer` from a [`Position`]
    pub fn from_position(position: &Position) -> Self {
        Self {
            position: *position
        }
    }

    /// Get an `Pointer` from a [`Cell`] using the `&XY` format, see
    /// [address format](Address#format) for more information
    ///
    /// # Error
    /// Return an error if the `Cell` could not be parsed, either because it
    /// does not start with the right prefix (`&`) or because the following
    /// position is invalid, see
    /// [position representation](Position#representation) for more information
    pub fn from_cell(cell: &Cell) -> Result<Self, anyhow::Error> {
        let cell_content = cell.content();
        let pos = cell_content.strip_prefix(Self::PREFIX)
            .ok_or(anyhow!(format!("{} is not a valid `Address` format", cell_content)))?;
        let pos = Position::try_from(pos)?;

        Ok(Self::from_position(&pos))
    }

    fn resolve_recursively(&self, grid: &Grid) -> Cell {
        fn get(depth: u32, pointer: &Pointer, grid: &Grid) -> Cell {
            let pointed_cell = grid.get(pointer.position);

            if let Ok(pointer) = Pointer::from_cell(&pointed_cell) {
                if depth + 1 >= Pointer::MAX_RECURSION_DEPTH {
                    eprintln!("Couldn't resolve pointer chain further, max recursion depth reached ({}), last pointed cell used : `{}`", Pointer::MAX_RECURSION_DEPTH, &pointed_cell.content());
                    pointed_cell
                } else {
                    get(depth + 1, &pointer, grid)
                }
            } else {
                pointed_cell
            }
        }

        get(0, self, grid)
    }

    /// Return the first non-pointer operand
    pub fn resolve_to_operand(&self, grid: &Grid) -> Operand {
        Operand::from_cell(self.resolve_recursively(grid))
    }

    /// Return a [`Literal`], given a `Pointer` and a [`Grid`]
    pub fn resolve_to_literal(&self, grid: &Grid) -> Literal {
        // TODO : Might induce unchecked recursion ? should draw a graph of call to make sure
        self.resolve_to_operand(grid).resolve_to_literal(grid)
    }

    /// Return an [`Address`], given a `Pointer` and a [`Grid`]
    pub fn resolve_to_address(&self, grid: &Grid) -> Result<Address, anyhow::Error> {
        self.resolve_to_operand(grid).resolve_to_address(grid)
    }

    /// Return a [`Cell`] from an `Address`, using the `@XY` format,
    /// see [address format](Address#format) for more information
    pub fn as_cell(&self) -> Cell {
        let (x, y) = self.position.as_textual();
        Cell::new(
            format!("{}{}{}", Self::PREFIX, x, y).as_str()
        ).expect("A Position must always have a valid Cell representation")
    }

    pub fn as_literal(&self) -> Literal {
        Literal::from_cell(self.as_cell())
    }
}

/// An `Operand` is the element that is operated on.
/// A single operation can take multiples operands.
///
/// In Graliffer there are 3 types of operands :
/// - [`Literal`] : a string of character that represents data. Depending of
/// the operation and place, a literal can be parsed in different ways (as bool,
/// numbers ...)
/// - [`Address`] : a way to represent a position in a [`Grid`].
/// It can either be used to represent a designate a certain [`Cell`] in the
/// grid, where the operation can act on, or to reference another `Cell`s [`Literal`]
/// - [`Pointer`] : a way to designate another cell's operand, that will then
/// be interpreted. Multiples pointers can be chained to allow complex
/// data references
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Literal(Literal),
    Address(Address),
    Pointer(Pointer),
}

impl Operand {
    /// Get an `Operand` given a [`Cell`]
    ///
    /// Any [`Cell`] can be parsed as an `Operand`, because Literal is the default
    pub fn from_cell(cell: Cell) -> Self {
        let cell_content = cell.content();

        // If the cell could not be parsed as an address or pointer,
        // return a literal
        match cell_content.chars().next() {
            Some('@') => {
                if let Ok(address) = Address::from_cell(&cell) {
                    Self::Address(address)
                } else {
                    Self::Literal(Literal::from_cell(cell))
                }
            }
            Some('&') => {
                if let Ok(pointer) = Pointer::from_cell(&cell) {
                    Self::Pointer(pointer)
                } else {
                    Self::Literal(Literal::from_cell(cell))
                }
            }
            _ => {
                Self::Literal(Literal::from_cell(cell))
            }
        }
    }

    pub fn from_literal(literal: Literal) -> Self {
        Self::Literal(literal)
    }

    pub fn resolve_to_literal(&self, grid: &Grid) -> Literal {
        match self {
            Self::Literal(literal) => literal.clone(),
            Self::Address(address) => address.fetch_literal(grid),
            Self::Pointer(pointer) => pointer.resolve_to_literal(grid),
        }
    }

    pub fn resolve_to_address(&self, grid: &Grid) -> Result<Address, anyhow::Error> {
        match self {
            Self::Literal(literal) => Err(anyhow::anyhow!("cannot resolve to address, got literal : `{:?}`", literal)),
            Self::Address(address) => Ok(*address),
            Self::Pointer(pointer) => pointer.resolve_to_address(grid)
        }
    }

    pub fn as_literal(&self) -> Literal {
        match self {
            Self::Literal(literal) => literal.clone(),
            Self::Address(address) => address.as_literal(),
            Self::Pointer(pointer) => pointer.as_literal(),
        }
    }

    pub fn as_cell(&self) -> Cell {
        match self {
            Self::Literal(literal) => literal.as_cell(),
            Self::Address(address) => address.as_cell(),
            Self::Pointer(pointer) => pointer.as_cell(),
        }
    }

    // pub fn resolve_as_cell(&self, grid: &Grid) -> Cell {
    //     self.resolve_to_literal(grid).as_cell()
    // }

    // pub fn resolve_as_bool_with_default()

    // pub fn resolve_to_bool_with_default(&self, grid: &Grid) -> bool {
    //     self.resolve_to_literal(grid).as_bool()
    // }

    // pub fn resolve_as_numeric(&self, grid: &Grid) -> u32 {
    //     self.resolve_to_literal(grid).as_numeric()
    // }
}

impl From<Literal> for Operand {
    fn from(value : Literal) -> Self {
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
