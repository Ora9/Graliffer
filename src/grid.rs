//! Grid represent the Graliffer grid, it hold the data

use std::{collections::HashMap, fmt::Debug, ops::Range};
use egui::TextBuffer;
use serde::Serialize;
use unicode_segmentation::UnicodeSegmentation;

mod position;
pub use position::{
    PositionAxis,
    Position,
};

mod head;
pub use head::{
    Head,
    HeadAction,
};

use crate::{artifact::{Action, Artifact}, Frame};

/// A `Cell` represents a unit of a [`Grid`], it holds a string of 3 chars (more precislely unicode graphems)
#[derive(Default, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Cell(String);

impl Cell {
    /// Obtain a `Cell` based on a `&str`
    ///
    /// # Errors
    /// Return an error if the string is more than 3 graphems long
    pub fn new(string: &str) -> Result<Self, anyhow::Error> {
        // Todo : make an empty content be a no op
        if string.graphemes(true).count() > 3 {
            Err(anyhow::anyhow!("invalid cell content : the given content is more than 3 graphems long"))
        } else {
            Ok(Self(string.to_string()))
        }
    }

    /// Try to insert `string` into the `Cell` at the specified `char_index`
    ///
    /// # Notes
    /// `char_index` is a *character index*, not a byte index.
    ///
    /// # Returns
    /// Returns a result with `Ok` containing how many *characters* were successfully inserted
    ///
    /// # Errors
    /// Returns a result with `Err` if `string` could not be inserted into the `Cell`
    pub fn insert_at(&mut self, string: &str, char_index: usize) -> Result<usize, anyhow::Error> {
        let mut self_owned = self.0.to_owned();

        let byte_index = byte_index_from_char_index(self_owned.as_str(), char_index);

        self_owned.insert_str(byte_index, string);

        if let Err(error) = Self::new(self_owned.as_str()) {
            Err(error)
        } else {
            self.0 = self_owned;
            Ok(string.chars().count())
        }
    }

    /// Try to remove the specified `char_range` from the `Cell`
    ///
    /// # Notes
    /// `char_range` is a *character index*, not a byte index.
    ///
    /// # Returns
    /// Returns a result with `Ok` containing how many *characters* were successfully inserted
    ///
    /// # Errors
    ///
    // Code stolen from egui's TextBuffer implementation on string
    pub fn delete_char_range(&mut self, char_range: Range<usize>) -> Result<usize, anyhow::Error> {
        assert!(char_range.start <= char_range.end);

        // Get both byte indices
        let byte_start = byte_index_from_char_index(self.as_str(), char_range.start);
        let byte_end = byte_index_from_char_index(self.as_str(), char_range.end);

        Ok(self.0.drain(byte_start..byte_end).count())
    }

    /// Remove all character from the `Cell`
    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get_content_range(&self, char_range: Range<usize>) -> String {
        assert!(char_range.start <= char_range.end);

        let byte_start = byte_index_from_char_index(self.as_str(), char_range.start);
        let byte_end = byte_index_from_char_index(self.as_str(), char_range.end);

        self.0[byte_start..byte_end].to_string()
    }


    pub fn set(&mut self, content: &str) -> Result<&Self, anyhow::Error>{
        // TODO: just a test please code : be better
        let intern = Self::new(content)?;
        self.0 = intern.0;
        Ok(self)
    }

    /// Return true if self is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn content(&self) -> String {
        self.0.clone()
    }
}

impl egui::TextBuffer for Cell {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        self.0.as_str()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        self.insert_at(text, char_index).unwrap_or(0)
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        assert!(char_range.start <= char_range.end);

        // Get both byte indices
        let byte_start = byte_index_from_char_index(self.as_str(), char_range.start);
        let byte_end = byte_index_from_char_index(self.as_str(), char_range.end);

        // Then drain all characters within this range
        let _ = self.delete_char_range(byte_start..byte_end);
    }
}

// Code from https://docs.rs/egui/0.31.1/src/egui/text_selection/text_cursor_state.rs.html#322
pub fn byte_index_from_char_index(s: &str, char_index: usize) -> usize {
    for (ci, (bi, _)) in s.char_indices().enumerate() {
        if ci == char_index {
            return bi;
        }
    }
    s.len()
}

/// A `Grid` represents a 2d space filled with [`Cell`]s, theses cells are positioned by a [`Position`]
///
#[derive(Default, Debug, Serialize)]
pub struct Grid(HashMap<Position, Cell>);

impl Grid {
    /// Obtain an empty `Grid`
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Get a [`Cell`] given a certain [`Position`], returns
    /// Internaly [`Grid`] only store cells that currently holds text, but for any valid [`Position`] must always return a valid [`Cell`], even if it does not exists in internal hashmap, because it's empty
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

#[derive(Clone)]
pub enum GridAction {
    Set(Position, Cell),
}

impl Action for GridAction {
    fn act(&self, frame: &mut Frame) -> Artifact {
        match self {
            Self::Set(position, cell) => {
                let previous_cell = frame.grid.get(*position);

                frame.grid.set(*position, cell.clone());

                Artifact::from_redo_undo(
                    Box::new(self.to_owned()),
                    Box::new(Self::Set(*position, previous_cell))
                )
            }
        }
    }
}

impl Debug for GridAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Set(position, cell) => {
                write!(f, "GridAction::Set ({:?} at {:?}", cell, position)
            }
        }
    }
}
