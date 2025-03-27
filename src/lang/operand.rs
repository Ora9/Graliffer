use crate::grid::{Grid, Cell, Position};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    value: Cell,
}

impl Literal {
    pub fn new(value: Cell) -> Self {
        Self {
            value
        }
    }

    pub fn from_cell(cell: Cell) -> Literal {
        Self::new(cell)
    }

    pub fn as_cell(&self) -> Cell {
        self.value.clone()
    }

    /// Return a bool evaluation of the literal :
    /// - "1"  is considered boolean true
    /// - Anything else is considered boolean false
    pub fn as_bool(&self) -> bool {
        self.value.content() == "1"
    }

    /// Return a numeric (u32) evaluation of the literal :
    /// - A positive integer if the string could be evaluated
    /// - 0 if the string was not a number
    pub fn as_numeric(&self) -> u32 {
        self.value.content().parse().unwrap_or(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Address {
    position: Position
}

impl Address {
    const IDENTIFIER: char = '@';

    pub fn from_position(position: &Position) -> Self {
        Self {
            position: *position
        }
    }

    pub fn from_cell(cell: &Cell) -> Result<Self, anyhow::Error> {
        let cell_content = cell.content();
        let pos = cell_content.trim_start_matches(Self::IDENTIFIER);
        let pos = Position::try_from(pos)?;

        Ok(Self::from_position(&pos))
    }

    pub fn as_cell(&self) -> Cell {
        let (x, y) = self.position.as_textual();
        let cell = Cell::new(format!("{}{}{}", Self::IDENTIFIER, x, y).as_str()).expect("A Position must always have a valid Cell representation");

        cell
    }

    pub fn as_position(&self) -> Position {
        self.position
    }

    pub fn as_literal(&self) -> Literal {
        Literal::from_cell(self.as_cell())
    }

    pub fn fetch_operand(&self, grid: &Grid) -> Operand {
        Operand::from_cell(grid.get(self.position))
    }

    pub fn fetch_literal(&self, grid: &Grid) -> Literal {
        Literal::from_cell(grid.get(self.position))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pointer {
    position: Position,
}

impl Pointer {
    const IDENTIFIER: char = '&';
    const MAX_RECURSION_DEPTH: u32 = 5;

    pub fn from_position(position: &Position) -> Self {
        Self {
            position: *position
        }
    }

    pub fn from_cell(cell: &Cell) -> Result<Self, anyhow::Error> {
        let cell_content = cell.content();
        let pos = cell_content.trim_start_matches(Self::IDENTIFIER);
        let pos = Position::try_from(pos)?;

        Ok(Self::from_position(&pos))
    }

    fn fetch_cell(&self, grid: &Grid) -> Cell {
        fn get(depth: u32, pointer: &Pointer, grid: &Grid) -> Cell {
            let pointed_cell = grid.get(pointer.position);

            if let Ok(pointer) = Pointer::from_cell(&pointed_cell) {
                if depth + 1 >= Pointer::MAX_RECURSION_DEPTH {
                    eprintln!("Max recursion reached!");
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

    pub fn fetch_operand(&self, grid: &Grid) -> Operand {
        Operand::from_cell(self.fetch_cell(grid))
    }

    pub fn fetch_literal(&self, grid: &Grid) -> Literal {
        // Might induce recursion ? should draw a graph of call to make sure
        self.fetch_operand(grid).resolve_to_literal(grid)
    }

    pub fn fetch_address(&self, grid: &Grid) -> Result<Address, anyhow::Error> {
        self.fetch_operand(grid).resolve_to_address(grid)
    }

    pub fn as_cell(&self) -> Cell {
        let (x, y) = self.position.as_textual();
        let cell = Cell::new(format!("{}{}{}", Self::IDENTIFIER, x, y).as_str()).expect("A Position must always have a valid Cell representation");

        cell
    }

    pub fn as_literal(&self) -> Literal {
        Literal::from_cell(self.as_cell())
    }
}

/// Operand
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Literal(Literal),
    Address(Address),
    Pointer(Pointer),
}

impl Operand {
    /// Get an `Operand` given a [`Cell`]
    /// Any [`Cell`] can be parsed as an `Operand`
    /// Either as an :
    /// - `Operand::Address`, used to access or represent a cell's position in a [Grid](crate::grid::Grid). Must be in format `@xy`, with `x` and `y` being valid textual representation of [`Position`] axies
    /// - `Operand::Pointer` used to
    /// - `Operand::Literal` containing a `String`
    pub fn from_cell(cell: Cell) -> Self {
        let cell_content = cell.content();

        match cell_content.chars().next() {
            Some('@') => Self::Address(Address::from_cell(&cell).unwrap()),
            Some('&') => Self::Pointer(Pointer::from_cell(&cell).unwrap()),
            _ => Self::Literal(Literal::from_cell(cell)),
        }
    }

    pub fn from_literal(literal: Literal) -> Self {
        Self::Literal(literal)
    }

    pub fn resolve_to_literal(&self, grid: &Grid) -> Literal {
        match self {
            Self::Literal(literal) => literal.clone(),
            Self::Address(address) => address.fetch_literal(grid),
            Self::Pointer(pointer) => pointer.fetch_literal(grid),
        }
    }

    pub fn resolve_to_address(&self, grid: &Grid) -> Result<Address, anyhow::Error> {
        match self {
            Self::Literal(literal) => Err(anyhow::anyhow!("cannot resolve to address, got literal : `{:?}`", literal)),
            Self::Address(address) => Ok(*address),
            Self::Pointer(pointer) => pointer.fetch_address(grid)
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

    pub fn resolve_as_cell(&self, grid: &Grid) -> Cell {
        self.resolve_to_literal(grid).as_cell()
    }

    pub fn resolve_as_bool(&self, grid: &Grid) -> bool {
        self.resolve_to_literal(grid).as_bool()
    }

    pub fn resolve_as_numeric(&self, grid: &Grid) -> u32 {
        self.resolve_to_literal(grid).as_numeric()
    }
}
