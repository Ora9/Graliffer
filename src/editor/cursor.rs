use crate::{
    grid::{Grid, Position},
    utils::Direction,
};

#[derive(Clone, Copy)]
pub enum PreferredCharPosition {
    AtStart,
    AtEnd,
    ForwardBy(usize),
    BackwardBy(usize),
    At(usize),
}

pub enum PreferredGridPosition {
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
            grid_position: Position::ZERO,
            char_position: 0,
        }
    }
}

impl Cursor {
    fn new() -> Self {
        Cursor::default()
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

    pub fn move_to(
        &mut self,
        preferred_grid_position: PreferredGridPosition,
        preferred_char_position: PreferredCharPosition,
        grid: &Grid,
    ) {
        self.grid_move_to(preferred_grid_position, grid);
        self.char_move_to(preferred_char_position, grid);
    }

    // todo: remove frame to pass just the cell content, or maybe in PreferredCharPosition::At(2, cell) ?
    pub fn char_move_to(&mut self, preferred_char_position: PreferredCharPosition, grid: &Grid) {
        use PreferredCharPosition::*;
        let char_position = match preferred_char_position {
            AtStart => 0,
            AtEnd => grid.get(self.grid_position).len(),
            At(char_position) => {
                let max_length = grid.get(self.grid_position).len();
                char_position.min(max_length)
            }
            ForwardBy(offset) => {
                let cell_length = grid.get(self.grid_position).len();
                self.char_position.saturating_add(offset).min(cell_length)
            }
            BackwardBy(offset) => self.char_position.saturating_sub(offset),
        };

        self.char_position = char_position;
    }

    fn grid_move_to(&mut self, preferred_grid_position: PreferredGridPosition, _grid: &Grid) {
        use PreferredGridPosition::*;
        let grid_position = match preferred_grid_position {
            At(position) => position,
            InDirectionByOffset(direction, offset) => {
                use Direction::*;
                let result = match direction {
                    Up => self.grid_position.checked_decrement_y(offset as u32),
                    Right => self.grid_position.checked_increment_x(offset as u32),
                    Down => self.grid_position.checked_increment_y(offset as u32),
                    Left => self.grid_position.checked_decrement_x(offset as u32),
                };

                if let Ok(position) = result {
                    position
                } else {
                    self.grid_position()
                }
            }
            InDirectionUntilNonEmpty(_direction) => {
                Position::from_numeric(5, 5).unwrap()

                // unimplemented!()
            }
        };
        self.grid_position = grid_position;
    }
}
