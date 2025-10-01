use crate::{
    grid::{Grid, Position},
    utils::Direction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferredCharPosition {
    Unchanged,
    AtStart,
    AtEnd,
    ForwardBy(usize),
    BackwardBy(usize),
    AtMost(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreferredGridPosition {
    Unchanged,
    At(Position),
    InDirectionByOffset(Direction, usize),
    InDirectionUntilNonEmpty(Direction),
}

/// A cursor wandering around a [`Grid`]
/// For now the cursor has only one [`Position`], but will probably have two in the future to represent a selection
// Work to make the char_position cursor better :
// a prefered position, to be used when moving a new grid_pos, because we want to be a certain place of the cell content
// or when clicking on a cell, we want to be at this place when
#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    grid_position: Position,
    char_position: usize,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            grid_position: Position::ORIGIN,
            char_position: 0,
        }
    }
}

impl Cursor {
    fn new(grid_position: Position, char_position: usize) -> Self {
        Self {
            grid_position,
            char_position,
        }
    }

    fn new_at(grid_position: Position) -> Self {
        Self {
            grid_position,
            char_position: 0,
        }
    }

    pub fn grid_position(&self) -> Position {
        self.grid_position
    }

    pub fn char_position(&self) -> usize {
        self.char_position
    }

    #[must_use = "this function does not mutate the given cursor, it returns the result of the operation"]
    pub fn with_position(
        &self,
        preferred_grid_position: PreferredGridPosition,
        preferred_char_position: PreferredCharPosition,
        grid: &Grid,
    ) -> Result<Self, anyhow::Error> {
        let cursor = self.grid_with(preferred_grid_position, grid)?;
        let cursor = cursor.char_with(preferred_char_position, grid)?;

        Ok(cursor)
    }

    #[must_use = "this function does not mutate the given cursor, it returns the result of the operation"]
    pub fn grid_with(
        &self,
        preferred_grid_position: PreferredGridPosition,
        grid: &Grid,
    ) -> Result<Self, anyhow::Error> {
        use PreferredGridPosition::*;
        let grid_position = match preferred_grid_position {
            Unchanged => self.grid_position,
            At(position) => position,
            InDirectionByOffset(direction, offset) => {
                self.grid_position.checked_step(direction, offset as u32)?
            }
            InDirectionUntilNonEmpty(direction) => {
                let mut pos = self.grid_position;

                while let Ok(next) = pos.checked_step(direction, 1) {
                    pos = next;
                    if grid.get(pos).is_empty() {
                        continue;
                    } else {
                        break;
                    }
                }

                pos
            }
        };

        Ok(Self {
            grid_position,
            char_position: self.char_position,
        })
    }

    #[must_use = "this function does not mutate the given cursor, it returns the result of the operation"]
    pub fn char_with(
        &self,
        preferred_char_position: PreferredCharPosition,
        grid: &Grid,
    ) -> Result<Self, anyhow::Error> {
        use PreferredCharPosition::*;
        let char_position = match preferred_char_position {
            Unchanged => self.char_position,
            AtStart => 0,
            AtEnd => grid.get(self.grid_position).len(),
            AtMost(char_position) => {
                let max_length = grid.get(self.grid_position).len();
                char_position.min(max_length)
            }
            ForwardBy(offset) => {
                let cell_length = grid.get(self.grid_position).len();
                self.char_position.saturating_add(offset).min(cell_length)
            }
            BackwardBy(offset) => self.char_position.saturating_sub(offset),
        };

        Ok(Self {
            grid_position: self.grid_position,
            char_position,
        })
    }
}
