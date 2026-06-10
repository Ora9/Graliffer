use std::{any::Any, collections::HashMap};

mod cell;
pub use cell::*;

mod position;
pub use position::*;

mod direction;
pub use direction::*;
use serde::{Deserialize, Serialize};

use crate::{Action, Apply, Revert, State};

#[derive(Debug, Serialize, Deserialize)]
pub struct Grid(HashMap<Position, Cell>);

impl Grid {
    /// Obtain a new empty `Grid`
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Insert given [`Cell`] at [`Position`]
    pub fn set(&mut self, position: Position, cell: Cell) {
        if cell.is_empty() {
            self.0.remove(&position);
        } else {
            self.0.insert(position, cell);
        }
    }

    /// Get the [`Cell`] at the given [`Position`]
    pub fn get(&self, position: Position) -> Cell {
        if let Some(cell) = self.0.get(&position) {
            cell.clone()
        } else {
            Cell::default()
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum GridAction {
    Set(Position, Cell),
}

impl Action for GridAction {}

impl State for Grid {
    type Action = GridAction;
    type Error = ();

    fn act(&mut self, action: &GridAction) -> Result<Revert, Self::Error> {
        match action {
            GridAction::Set(position, cell) => {
                let last_cell = self.get(*position);
                self.set(*position, cell.clone());

                Ok(Revert::new(GridAction::Set(*position, last_cell)))
            }
        }
    }
}
