use grai::{Direction, HorizontalDirection};
use tui_input::{Input, InputRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorMovement {
    /// The default when pressing an arrow key, either stepping to the next character in a cell, or
    /// to the next cell is the char cursor is at the end or start of cell
    StepCharThenGrid(Direction),

    /// A step to the next cell, ignoring the current char cursor position
    /// Used for the tab, enter and space keys
    StepGrid(Direction),

    /// A dash to either the cell's bound (start or end) or to the next non-empty cell in that
    /// direction
    DashUntilBoundsOrNonEmpty(Direction),

    /// Move the cursor to a given position in the grid
    Jump(grai::Position),
}

pub enum CharCursorPosition {
    Unchanged,
    AtEnd,
    AtStart,
    AtMost(usize),
    InDirectionByOffset(HorizontalDirection, usize),
}

pub enum GridCursorPosition {
    Unchanged,
    At(grai::Position),
    InDirectionByOffset(Direction, u32),
    InDirectionUntilNonEmpty(Direction),
}

#[derive(Debug)]
pub struct GridInput {
    grid_cursor: grai::Position,
    input: Input,
}

impl GridInput {
    pub fn new(grid: &grai::Grid) -> Self {
        let mut grid_input = Self {
            input: Input::default(),
            grid_cursor: grai::Position::default(),
        };

        grid_input.sync_input(grid);
        grid_input.set_char_position(CharCursorPosition::AtEnd, grid);
        grid_input
    }

    pub fn grid_cursor(&self) -> grai::Position {
        self.grid_cursor
    }

    pub fn char_cursor(&self) -> usize {
        self.input.visual_cursor()
    }

    pub fn char_at_start(&self) -> bool {
        self.char_cursor() == 0
    }

    pub fn char_at_end(&self, grid: &grai::Grid) -> bool {
        self.char_cursor() >= grid.get(self.grid_cursor).len()
    }

    pub fn char_at_max(&self) -> bool {
        self.char_cursor() >= 3
    }

    // TODO: this probably induce a bug when codepoint != visual length != graphem count
    pub fn input_full(&self) -> bool {
        self.input.value().len() >= 3
    }

    pub fn insert(&mut self, grid: &mut grai::Grid, input: char) {
        if !self.input_full() && input != ' ' {
            self.handle(grid, InputRequest::InsertChar(input));
        }
    }

    pub fn handle(&mut self, grid: &mut grai::Grid, input_request: InputRequest) {
        // TODO, BUG: when cursor at right border, inserting when cell full move the cursor back

        self.input.handle(input_request);
        grid.set(self.grid_cursor, grai::Cell::new_trim(self.input.value()));
        self.sync_input(grid);
    }

    pub fn with_movement(&mut self, movement: CursorMovement, grid: &grai::Grid) {
        let at_start = self.char_at_start();
        let at_end = self.char_at_end(grid);

        let grid_at_left = self.grid_cursor.x() == 0;
        let grid_at_right = self.grid_cursor.x() == grai::granary::GranaryDigit::MAX_NUMERIC;

        // debug!("at_start: {at_start}, at_end: {at_end}");

        let (grid_position, char_position) = match movement {
            CursorMovement::Jump(position) => {
                (GridCursorPosition::At(position), CharCursorPosition::AtEnd)
            }
            CursorMovement::StepGrid(direction) => (
                GridCursorPosition::InDirectionByOffset(direction, 1),
                match direction {
                    Direction::Up | Direction::Down => CharCursorPosition::Unchanged,
                    Direction::Left => CharCursorPosition::AtEnd,
                    Direction::Right => CharCursorPosition::AtStart,
                },
            ),
            CursorMovement::StepCharThenGrid(direction) => match direction {
                Direction::Up | Direction::Down => (
                    GridCursorPosition::InDirectionByOffset(direction, 1),
                    CharCursorPosition::AtMost(self.char_cursor()),
                ),
                Direction::Left if at_start && grid_at_left => {
                    (GridCursorPosition::Unchanged, CharCursorPosition::Unchanged)
                }
                Direction::Left if at_start => (
                    GridCursorPosition::InDirectionByOffset(direction, 1),
                    CharCursorPosition::AtEnd,
                ),
                Direction::Left => (
                    GridCursorPosition::Unchanged,
                    CharCursorPosition::InDirectionByOffset(HorizontalDirection::Left, 1),
                ),
                Direction::Right if at_end && grid_at_right => {
                    (GridCursorPosition::Unchanged, CharCursorPosition::Unchanged)
                }
                Direction::Right if at_end => (
                    GridCursorPosition::InDirectionByOffset(direction, 1),
                    CharCursorPosition::AtStart,
                ),
                Direction::Right => (
                    GridCursorPosition::Unchanged,
                    CharCursorPosition::InDirectionByOffset(HorizontalDirection::Right, 1),
                ),
            },
            CursorMovement::DashUntilBoundsOrNonEmpty(direction) => match direction {
                Direction::Up | Direction::Down => (
                    GridCursorPosition::InDirectionUntilNonEmpty(direction),
                    CharCursorPosition::AtMost(self.char_cursor()),
                ),
                Direction::Left if at_start && grid_at_left => {
                    (GridCursorPosition::Unchanged, CharCursorPosition::Unchanged)
                }
                Direction::Left if at_start => (
                    GridCursorPosition::InDirectionUntilNonEmpty(direction),
                    CharCursorPosition::AtEnd,
                ),
                Direction::Left => (GridCursorPosition::Unchanged, CharCursorPosition::AtStart),
                Direction::Right if at_end && grid_at_right => {
                    (GridCursorPosition::Unchanged, CharCursorPosition::Unchanged)
                }
                Direction::Right if at_end => (
                    GridCursorPosition::InDirectionUntilNonEmpty(direction),
                    CharCursorPosition::AtStart,
                ),
                Direction::Right => (GridCursorPosition::Unchanged, CharCursorPosition::AtEnd),
            },
        };

        self.set_positions(grid_position, char_position, grid);
        // debug!(
        //     "grid: {}, char {}, movement: {:?}",
        //     self.grid_cursor,
        //     self.input.visual_cursor(),
        //     movement
        // )
    }

    fn set_positions(
        &mut self,
        grid_position: GridCursorPosition,
        char_position: CharCursorPosition,
        grid: &grai::Grid,
    ) {
        self.set_grid_position(grid_position, grid);
        self.sync_input(grid);
        self.set_char_position(char_position, grid);
    }

    fn sync_input(&mut self, grid: &grai::Grid) {
        let cursor = self.input.cursor();
        self.input = Input::new(grid.get(self.grid_cursor).to_string());
        self.input.handle(InputRequest::SetCursor(cursor));
    }

    fn set_grid_position(&mut self, grid_position: GridCursorPosition, grid: &grai::Grid) {
        let position = match grid_position {
            GridCursorPosition::Unchanged => self.grid_cursor(),
            GridCursorPosition::At(position) => position,
            GridCursorPosition::InDirectionByOffset(direction, offset) => self
                .grid_cursor()
                .checked_step(direction, offset)
                .unwrap_or(self.grid_cursor()),
            GridCursorPosition::InDirectionUntilNonEmpty(direction) => {
                let mut pos = self.grid_cursor;
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

        self.grid_cursor = position
    }

    fn set_char_position(&mut self, char_position: CharCursorPosition, grid: &grai::Grid) {
        let cell = grid.get(self.grid_cursor);
        let cursor = match char_position {
            CharCursorPosition::Unchanged => self.char_cursor(),
            CharCursorPosition::AtStart => 0,
            CharCursorPosition::AtEnd => cell.len(),
            CharCursorPosition::AtMost(p) => p,
            CharCursorPosition::InDirectionByOffset(direction, offset) => match direction {
                HorizontalDirection::Left => self.char_cursor().saturating_sub(offset),
                HorizontalDirection::Right => self.char_cursor().saturating_add(offset),
            },
        }
        .min(cell.len());

        self.input.handle(InputRequest::SetCursor(cursor));
    }
}
