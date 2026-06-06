use std::{any::Any, collections::HashMap};

mod cell;
pub use cell::*;

mod position;
pub use position::*;

mod direction;
pub use direction::*;

use crate::{Action, State};

#[derive(Debug)]
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

#[derive(Debug)]
pub enum GridAction {
    Set(Position, Cell),
}

impl Action for GridAction {}

impl State for Grid {
    type Action = GridAction;

    fn act(&mut self, action: &Self::Action) {
        match action {
            GridAction::Set(position, cell) => {
                self.set(*position, cell.clone());
            }
        }
    }
}
