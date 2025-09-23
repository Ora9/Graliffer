//! Grid represent the Graliffer grid, it hold the data

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

mod position;
pub use position::{Position, PositionAxis};

mod cell;
pub use cell::Cell;


/// A `Grid` represents a 2d space filled with [`Cell`]s, theses cells are positioned by a [`Position`]
///
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Grid(HashMap<Position, Cell>);

impl Grid {
    /// Obtain an empty `Grid`
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Get a [`Cell`] given a certain [`Position`], returns
    /// Internaly [`Grid`] only store cells that currently holds text,
    /// but for any valid [`Position`] must always return a valid [`Cell`],
    /// even if it does not exists in internal hashmap, because it's empty
    pub fn get(&self, position: Position) -> Cell {
        let opt = self.0.get(&position);

        if let Some(cell) = opt {
            cell.clone()
        } else {
            Cell::default()
        }
    }

    // pub fn get_mut(&mut self, position: Position) -> &mut Cell {
    //     // Break the empty is inexistant by creating a empty cell in the hashset
    //     let entry = self.0.entry(position);

    //     entry.or_insert(Cell::new("").unwrap())
    // }

    pub fn set(&mut self, position: Position, cell: Cell) {
        // If we set an empty cell, remove that cell from grid
        if cell.is_empty() {
            self.0.remove(&position);
        } else {
            self.0.insert(position, cell);
        }
    }

    // pub fn to_json(&self) -> String {
    //     let serialized = serde_json::to_string(&self).unwrap();
    // }
}
