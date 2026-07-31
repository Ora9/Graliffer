use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::Cell;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Errored;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("invalid errored format: expected to find exactly `###`, found `{got}`")]
pub struct ErroredFormatError {
    pub got: String,
}

impl Errored {
    const CELL_STRING: &'static str = "###";

    pub fn new() -> Self {
        Errored
    }

    pub fn from_ref_cell(cell: &Cell) -> Result<Self, ErroredFormatError> {
        if cell.as_str() == Self::CELL_STRING {
            Ok(Errored)
        } else {
            Err(ErroredFormatError {
                got: cell.to_string(),
            })
        }
    }

    pub fn to_cell(&self) -> Cell {
        Cell::new_trim(Self::CELL_STRING)
    }
}

impl Display for Errored {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_cell().to_string())
    }
}
