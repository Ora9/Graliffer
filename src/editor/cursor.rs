use crate::{artifact::{Action, Artifact}, grid::{Direction, Grid, Position}, Frame};

#[derive(Debug, Clone, Copy)]
pub enum PreferredCharPosition {
    AtStart,
    AtEnd,
    ForwardBy(usize),
    BackwardBy(usize),
    At(usize)
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
    fn new(grid_position: Position) -> Self {
        Self {
            grid_position,
            char_position: 0
        }
    }

    pub fn grid_position(&self) -> Position {
        self.grid_position
    }

    pub fn char_position(&self) -> usize {
        self.char_position
    }

    /// Move the cursor to new [`Position`] placing self.char_position after the last char of new cell.
    fn move_to(&mut self, grid_position: Position, char_position: usize) {
        dbg!(grid_position, char_position);

        self.grid_position = grid_position;
        self.char_position = char_position
    }

    fn move_char_position_to(&mut self, char_position: usize) {
        self.char_position = char_position;
    }
}

#[derive(Debug, Clone)]
pub enum CursorAction {
    MoveTo(Position, PreferredCharPosition),
    CharMoveTo(PreferredCharPosition),
    GridStepInDirection(Direction, PreferredCharPosition),
}

impl Action for CursorAction {
    fn act(&self, frame: &mut Frame) -> Artifact {
        match self {
            Self::MoveTo(grid_position, preferred_char_position) => {
                let old_grid_position = frame.editor.cursor.grid_position;
                let old_char_position = frame.editor.cursor.char_position;

                use PreferredCharPosition::*;
                let char_position = match *preferred_char_position {
                    AtStart => 0,
                    AtEnd => frame.grid.get(*grid_position).len(),
                    At(char_position) => char_position,

                    // These don't really have a use in this context, but we implement them anyway
                    ForwardBy(offset) => old_char_position.saturating_add(offset),
                    BackwardBy(offset) => old_char_position.saturating_sub(offset),
                };

                dbg!(char_position);

                frame.editor.cursor.move_to(*grid_position, char_position);

                Artifact::from_redo_undo(
                    Box::new(self.to_owned()),
                    Box::new(Self::MoveTo(old_grid_position, *preferred_char_position))
                )
            }
            Self::CharMoveTo(preferred_char_position) => {
                let old_char_position = frame.editor.cursor.char_position;

                use PreferredCharPosition::*;
                let char_position = match *preferred_char_position {
                    AtStart => 0,
                    AtEnd => frame.grid.get(frame.editor.cursor.grid_position).len(),
                    At(char_position) => char_position,
                    ForwardBy(offset) => old_char_position.saturating_add(offset),
                    BackwardBy(offset) => old_char_position.saturating_sub(offset),
                };

                frame.editor.cursor.move_char_position_to(char_position);

                Artifact::from_redo_undo(
                    Box::new(self.to_owned()),
                    Box::new(Self::CharMoveTo(PreferredCharPosition::At(old_char_position)))
                )
            }
            Self::GridStepInDirection(direction, preferred_char_position) => {
                let old_grid_position = frame.editor.cursor.grid_position;
                let old_char_position = frame.editor.cursor.char_position;

                use Direction::*;
                if let Ok(grid_position) = match direction {
                    Right => old_grid_position.checked_increment_x(1),
                    Down => old_grid_position.checked_increment_y(1),
                    Left => old_grid_position.checked_decrement_x(1),
                    Up => old_grid_position.checked_decrement_y(1),
                } {
                    use PreferredCharPosition::*;
                    let char_position = match *preferred_char_position {
                        AtStart => 0,
                        AtEnd => frame.grid.get(grid_position).len(),
                        At(char_position) => char_position,
                        BackwardBy(offset) => old_char_position.saturating_add(offset),
                        ForwardBy(offset) => old_char_position.saturating_sub(offset),
                    };

                    frame.editor.cursor.move_to(grid_position, char_position);
                }

                Artifact::from_redo_undo(
                    Box::new(self.to_owned()),
                    Box::new(Self::MoveTo(old_grid_position, PreferredCharPosition::At(old_char_position)))
                )
            }
        }
    }
}
