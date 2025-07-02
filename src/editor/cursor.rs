use crate::{
    grid::Position, utils::Direction, Frame
};

#[derive(Clone, Copy)]
pub enum PreferredCharPosition {
    AtStart,
    AtEnd,
    ForwardBy(usize),
    BackwardBy(usize),
    At(usize)
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
#[derive(Debug, Default, Clone, Copy)]
pub struct Cursor {
    grid_position: Position,
    char_position: usize,
}

impl Cursor {
    // fn new(grid_position: Position) -> Self {
    //     Self {
    //         grid_position,
    //         char_position: 0
    //     }
    // }

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
        frame: &Frame) {
            self.grid_move_to(preferred_grid_position, frame);
            self.char_move_to(preferred_char_position, frame);
        }

    // todo: remove frame to pass just the cell content, or maybe in PreferredCharPosition::At(2, cell) ?
    pub fn char_move_to(&mut self, preferred_char_position: PreferredCharPosition, frame: &Frame) {
        use PreferredCharPosition::*;
        let char_position = match preferred_char_position {
            AtStart => 0,
            AtEnd => frame.grid.get(self.grid_position).len(),
            At(char_position) => {
                let max_length = frame.grid.get(self.grid_position).len();
                char_position.min(max_length)
            }
            ForwardBy(offset) => {
                let cell_length = frame.grid.get(self.grid_position).len();
                self.char_position
                    .saturating_add(offset)
                    .min(cell_length)
            }
            BackwardBy(offset) => self.char_position.saturating_sub(offset),
        };

        self.char_position = char_position;
    }

    fn grid_move_to(&mut self, preferred_grid_position: PreferredGridPosition, frame: &Frame) {
        use PreferredGridPosition::*;
        let grid_position = match preferred_grid_position {
            At(position) => {position},
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
            },
            InDirectionUntilNonEmpty(direction) => {
                unimplemented!()
            }

                // let (search_range, perpendicular_origin) = match preferred_grid_position {
                //     UpUntilNonEmpty => (
                //         self.grid_position().y()..PositionAxis::MIN_NUMERIC,
                //         self.grid_position().x()
                //     )
                //     RightUntilNonEmpty => (
                //         self.grid_position().x()..PositionAxis::MAX_NUMERIC,
                //         self.grid_position().y()
                //     )
                //     DownUntilNonEmpty => (
                //         self.grid_position().y()..PositionAxis::MAX_NUMERIC,
                //         self.grid_position().x()
                //     ),
                //     LeftUntilNonEmpty => (
                //         self.grid_position().x()..PositionAxis::MIN_NUMERIC,
                //         self.grid_position().y()
                //     )
                // };

                // for i in search_range {
                //     let position = Position::from_numeric(perpendicular_origin, i)

                // }

                // frame.grid.get(Position::from_numeric(x, y))
                // self.grid_position.checked_increment_y(offset as u32)
            // AtStart => 0,
            // AtEnd => frame.grid.get(self.grid_position).len(),
            // At(char_position) => {
            //     let max_length = frame.grid.get(self.grid_position).len();
            //     char_position.min(max_length)
            // }
            // ForwardBy(offset) => {
            //     let max_length = frame.grid.get(self.grid_position).len();
            //     self.char_position
            //         .saturating_add(offset)
            //         .min(max_length)
            // }
            // BackwardBy(offset) => self.char_position.saturating_sub(offset),
        };

        self.grid_position = grid_position;
    }

    // /// Move the cursor to new [`Position`] placing self.char_position after the last char of new cell.
    // pub fn move_to(&mut self, grid_position: Position, preferred_char_position: PreferredCharPosition, frame: &Frame) {
    //     use PreferredCharPosition::*;
    //     let char_position = match preferred_char_position {
    //         AtStart => 0,
    //         AtEnd => frame.grid.get(grid_position).len(),
    //         At(char_position) => {
    //             let max_length = frame.grid.get(grid_position).len();
    //             char_position.min(max_length)
    //         }
    //         ForwardBy(offset) => {
    //             let max_length = frame.grid.get(grid_position).len();
    //             self.char_position
    //                 .saturating_add(offset)
    //                 .min(max_length)
    //         }
    //         BackwardBy(offset) => self.char_position.saturating_sub(offset),
    //     };

    //     self.grid_position = grid_position;
    //     self.char_position = char_position;
    // }
}
