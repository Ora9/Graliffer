use std::convert::Infallible;

use act::{Action, Revert, State};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use grai::{Direction, HorizontalDirection};
use log::debug;
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Offset, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::merge::MergeStrategy,
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};
use serde::{Deserialize, Serialize};
use tui_input::{Input, InputRequest};

use crate::{AppAction, Context, View, ViewType};

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

enum CharCursorPosition {
    Unchanged,
    AtEnd,
    AtStart,
    AtMost(usize),
    InDirectionByOffset(HorizontalDirection, usize),
}

enum GridCursorPosition {
    Unchanged,
    At(grai::Position),
    InDirectionByOffset(Direction, u32),
    InDirectionUntilNonEmpty(Direction),
}

#[derive(Debug)]
struct GridInput {
    grid_cursor: grai::Position,
    input: Input,
}

impl GridInput {
    fn new(grid: &grai::Grid) -> Self {
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

#[derive(Debug, Default)]
enum DragState {
    #[default]
    Idle,
    Dragging {
        start_pointer_pos: Position,
        start_grid_offset: GridOffset,
    },
}

impl DragState {
    fn start_drag(&mut self, pointer_position: Position, grid_offset: GridOffset) {
        *self = Self::Dragging {
            start_pointer_pos: pointer_position,
            start_grid_offset: grid_offset,
        };
    }

    fn stop_drag(&mut self) {
        *self = Self::Idle;
    }

    fn dragging(&self) -> bool {
        matches!(
            self,
            DragState::Dragging {
                start_pointer_pos: _,
                start_grid_offset: _,
            }
        )
    }

    fn idle(&self) -> bool {
        matches!(self, DragState::Idle)
    }
}

const CELL_WIDTH: u16 = 3;
const CELL_HEIGHT: u16 = 1;
const CELL_BORDER: u16 = 1;

fn terminal_to_grid_x_axis(terminal_x: u16, area: Rect, offset: GridOffset) -> Option<u32> {
    Some(
        terminal_x
            .checked_add(offset.x as u16)?
            .saturating_sub(area.x)
            .saturating_sub(CELL_BORDER)
            .checked_div(CELL_WIDTH + CELL_BORDER)? as u32,
    )
    .and_then(
        |value| match grai::granary::GranaryDigit::is_valid_numeric(value) {
            false => None,
            true => Some(value),
        },
    )
}

fn terminal_to_grid_y_axis(terminal_y: u16, area: Rect, offset: GridOffset) -> Option<u32> {
    Some(
        terminal_y
            .checked_add(offset.y as u16)?
            .saturating_sub(area.y)
            .saturating_sub(CELL_BORDER)
            .checked_div(CELL_HEIGHT + CELL_BORDER)? as u32,
    )
    .and_then(
        |value| match grai::granary::GranaryDigit::is_valid_numeric(value) {
            false => None,
            true => Some(value),
        },
    )
}

fn terminal_to_grid_position(
    terminal_position: Position,
    area: Rect,
    offset: GridOffset,
) -> Option<grai::Position> {
    grai::Position::new(
        terminal_to_grid_x_axis(terminal_position.x, area, offset)?,
        terminal_to_grid_y_axis(terminal_position.y, area, offset)?,
    )
    .ok()
}

fn grid_to_terminal_position(
    grid_position: grai::Position,
    area: Rect,
    offset: GridOffset,
) -> Position {
    Position {
        x: (area.x as u32)
            .strict_add(grid_position.x() * (CELL_WIDTH + CELL_BORDER) as u32)
            .saturating_sub(offset.x as u32) as u16,

        y: (area.y as u32)
            .strict_add(grid_position.y() * (CELL_HEIGHT + CELL_BORDER) as u32)
            .saturating_sub(offset.y as u32) as u16,
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct GridOffset {
    x: usize,
    y: usize,
}

impl GridOffset {}

#[derive(Debug)]
pub struct GridView {
    #[allow(unused)]
    context: Context,

    frame: grai::FrameGuard,

    grid_input: GridInput,
    grid_offset: GridOffset,
    drag_state: DragState,

    layout: Option<Rect>,
}

impl GridView {
    pub fn new(frame: grai::FrameGuard, context: Context) -> Self {
        let grid_input = frame.read(|frame| GridInput::new(&frame.grid));

        GridView {
            context,
            frame,

            grid_input,
            grid_offset: GridOffset::default(),

            layout: None,

            drag_state: DragState::Idle,
        }
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        let Some(viewport_area) = self.layout() else {
            return;
        };

        let pointer_pos = Position {
            x: mouse_event.column,
            y: mouse_event.row,
        };

        // debug!(
        //     "{:?}",
        //     terminal_to_grid_position(pointer_pos, viewport_area, self.grid_offset)
        // );

        match mouse_event.kind {
            MouseEventKind::Down(mouse_button) if mouse_button == MouseButton::Left => {
                if let Some(grid_pos) =
                    terminal_to_grid_position(pointer_pos, viewport_area, self.grid_offset)
                {
                    self.cursor_movement(CursorMovement::Jump(grid_pos));
                }
            }
            MouseEventKind::ScrollUp
            | MouseEventKind::ScrollDown
            | MouseEventKind::ScrollLeft
            | MouseEventKind::ScrollRight => {
                let (x_offset, y_offset) = match mouse_event.kind {
                    MouseEventKind::ScrollLeft => (-1, 0),
                    MouseEventKind::ScrollRight => (1, 0),
                    MouseEventKind::ScrollUp => (0, -1),
                    MouseEventKind::ScrollDown => (0, 1),
                    _ => unreachable!(),
                };

                self.grid_offset.x = self.grid_offset.x.saturating_add_signed(x_offset);
                self.grid_offset.y = self.grid_offset.y.saturating_add_signed(y_offset);
            }
            MouseEventKind::Drag(button) if button.is_left() => {
                if self.drag_state.idle() {
                    self.drag_state.start_drag(pointer_pos, self.grid_offset);
                }

                if let DragState::Dragging {
                    start_pointer_pos,
                    start_grid_offset,
                } = self.drag_state
                {
                    self.grid_offset.x = start_grid_offset.x.saturating_add_signed(
                        (start_pointer_pos.x as i16).saturating_sub_unsigned(pointer_pos.x)
                            as isize,
                    );
                    self.grid_offset.y = start_grid_offset.y.saturating_add_signed(
                        (start_pointer_pos.y as i16).saturating_sub_unsigned(pointer_pos.y)
                            as isize,
                    );
                }
            }
            _ => {
                if self.drag_state.dragging() {
                    self.drag_state.stop_drag();
                }
            }
        }
    }

    pub fn handle_input(&mut self, input_request: InputRequest) {
        self.frame.write(|frame| {
            self.grid_input.handle(&mut frame.grid, input_request);
        });
    }

    pub fn cursor_movement(&mut self, movement: CursorMovement) {
        self.frame.read(|frame| {
            self.grid_input.with_movement(movement, &frame.grid);
        });

        // let Some(area) = self.layout() else {
        //     return;
        // };

        // let grid_pos = self.grid_input.grid_cursor;
        // let term_pos = grid_to_terminal_position(grid_pos, area, self.grid_offset);

        // if area.left() < term_pos.x {
        //     debug!("{:?}", term_pos);

        //     self.grid_offset.x = term_pos.x as usize;
        // }
    }

    pub fn layout(&self) -> Option<Rect> {
        self.layout
    }
}

#[derive(Debug)]
pub struct GridWidget;

impl GridWidget {
    pub fn new() -> Self {
        GridWidget
    }
}

impl StatefulWidget for GridWidget {
    type State = GridView;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.layout = Some(area);

        // A separate buffer is used to render the grid,
        // this is used to mask everything that is outside of the grid widget viewport
        // this is because widget drawn outside the buffer are clamped to the border, but we want to
        // have widgets drawn partialy onto the viewport
        let overdraw_cells: usize = 1;
        let overdraw_margin = Margin::new(
            (CELL_WIDTH + CELL_BORDER * 2 * overdraw_cells as u16) as u16,
            (CELL_HEIGHT + CELL_BORDER * 2 * overdraw_cells as u16) as u16,
        );
        let mut overdraw_buf = Buffer::empty(Rect {
            x: 0,
            y: 0,
            width: area.width + overdraw_margin.horizontal * 2,
            height: area.height + overdraw_margin.vertical * 2,
        });
        let overdraw_area = overdraw_buf.area().inner(overdraw_margin);

        let in_view_left =
            terminal_to_grid_x_axis(overdraw_area.left(), overdraw_area, state.grid_offset)
                .unwrap_or(grai::granary::GranaryDigit::MIN_NUMERIC);

        let in_view_right =
            terminal_to_grid_x_axis(overdraw_area.right(), overdraw_area, state.grid_offset)
                .unwrap_or(grai::granary::GranaryDigit::MAX_NUMERIC);

        let in_view_top =
            terminal_to_grid_y_axis(overdraw_area.top(), overdraw_area, state.grid_offset)
                .unwrap_or(grai::granary::GranaryDigit::MIN_NUMERIC);

        let in_view_bottom =
            terminal_to_grid_y_axis(overdraw_area.bottom(), overdraw_area, state.grid_offset)
                .unwrap_or(grai::granary::GranaryDigit::MAX_NUMERIC);

        for cell_x in in_view_left..=in_view_right {
            for cell_y in in_view_top..=in_view_bottom {
                let grid_pos = grai::Position::from_numeric(cell_x as u32, cell_y as u32)
                    .expect("should be able to construct a valid position");

                let term_pos =
                    grid_to_terminal_position(grid_pos, overdraw_area, state.grid_offset);

                let cell_area = Rect::new(
                    term_pos.x,
                    term_pos.y,
                    CELL_WIDTH + CELL_BORDER * 2,
                    CELL_HEIGHT + CELL_BORDER * 2,
                );

                let cell_content = state.frame.read(|frame| frame.grid.get(grid_pos));

                let block = Block::bordered()
                    .fg(Color::DarkGray)
                    .merge_borders(MergeStrategy::Fuzzy);

                Paragraph::new(cell_content.as_str())
                    .block(block)
                    .reset()
                    .render(cell_area, &mut overdraw_buf);
            }
        }

        let head_grid_pos = state.frame.read(|frame| frame.head.position);
        let head_term_pos =
            grid_to_terminal_position(head_grid_pos, overdraw_area, state.grid_offset);
        let head_area = Rect::new(
            head_term_pos.x,
            head_term_pos.y,
            CELL_WIDTH + CELL_BORDER * 2,
            CELL_HEIGHT + CELL_BORDER * 2,
        );
        Block::bordered()
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(Color::DarkGray))
            .merge_borders(MergeStrategy::Fuzzy)
            .render(head_area, &mut overdraw_buf);

        let cursor_grid_pos = state.grid_input.grid_cursor();
        let cursor_term_origin =
            grid_to_terminal_position(cursor_grid_pos, overdraw_area, state.grid_offset);
        let cursor_term_pos = Position::new(cursor_term_origin.x, cursor_term_origin.y)
            .offset(Offset::new(CELL_BORDER as i32, CELL_BORDER as i32))
            .offset(Offset::new(state.grid_input.char_cursor() as i32, 0));

        if let Some(cursor_cell) = overdraw_buf.cell_mut(cursor_term_pos) {
            cursor_cell.fg = if state.grid_input.char_at_max() {
                Color::DarkGray
            } else {
                Color::White
            };
            cursor_cell.modifier = cursor_cell.modifier.union(Modifier::REVERSED);
        }

        // our own implementation of Buffer::merge
        buffer_merge_areas(buf, area.as_position(), &overdraw_buf, overdraw_area);
    }
}

fn buffer_merge_areas(
    dest_buf: &mut Buffer,
    dest_pos: Position,
    from_buf: &Buffer,
    from_area: Rect,
) {
    // let size = from_area.area();
    for y in from_area.y..(from_area.y + from_area.height) {
        for x in from_area.x..(from_area.x + from_area.width) {
            let from_pos = Position::new(x, y);
            let from_cell = from_buf.cell(from_pos);

            let dest_pos = dest_pos.offset(Offset::new(
                x.saturating_sub(from_area.left()) as i32,
                y.saturating_sub(from_area.top()) as i32,
            ));

            let dest_cell = dest_buf.cell_mut(dest_pos);

            if let Some(dest_cell) = dest_cell
                && let Some(from_cell) = from_cell
            {
                dest_cell.set_symbol(from_cell.symbol());
                dest_cell.set_style(from_cell.style());
            }
        }
    }
}

#[derive(Debug, Clone, strum::EnumString, Serialize, Deserialize)]
pub enum GridAction {
    Insert(String),

    InsertOverflow(String),

    DeletePrevChar,
    DeleteNextChar,

    DeletePrevCharOrStepLeftGrid,

    DeleteTillStart,
    DeleteTillStartOrStepLeftGrid,

    CursorStepUpGrid,
    CursorStepDownGrid,
    CursorStepRightGrid,
    CursorStepLeftGrid,

    CursorStepRightCharThenGrid,
    CursorStepLeftCharThenGrid,

    CursorDashUpCharThenGrid,
    CursorDashDownCharThenGrid,
    CursorDashRightCharThenGrid,
    CursorDashLeftCharThenGrid,
}

impl Action for GridAction {}

impl State for GridView {
    type Action = GridAction;
    type Error = Infallible;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert, Self::Error> {
        let action = action.into();
        use GridAction::*;

        match action {
            Insert(input) => {
                for c in input.chars() {
                    self.frame
                        .write(|frame| self.grid_input.insert(&mut frame.grid, c));
                }
            }

            InsertOverflow(input) => {
                for c in input.chars() {
                    if self.grid_input.char_at_max() || c == ' ' {
                        self.cursor_movement(CursorMovement::StepGrid(Direction::Right));
                    }

                    // self.grid_input.insert(&mut frame.grid, c);
                    self.handle_input(InputRequest::InsertChar(c));
                }
            }

            DeletePrevCharOrStepLeftGrid => {
                if self.grid_input.char_cursor() != 0 {
                    self.handle_input(InputRequest::DeletePrevChar);
                } else {
                    self.cursor_movement(CursorMovement::StepGrid(Direction::Left));
                }
            }

            // todo: use the newer DeleteFromStart
            DeleteTillStart => {
                self.handle_input(InputRequest::DeletePrevWord);
            }

            DeleteTillStartOrStepLeftGrid => {
                if self.grid_input.char_cursor() != 0 {
                    self.handle_input(InputRequest::DeletePrevWord);
                } else {
                    self.cursor_movement(CursorMovement::StepGrid(Direction::Left));
                }
            }

            DeletePrevChar => {
                self.handle_input(InputRequest::DeletePrevChar);
            }

            DeleteNextChar => {
                self.handle_input(InputRequest::DeleteNextChar);
            }

            CursorStepUpGrid | CursorStepDownGrid | CursorStepLeftGrid | CursorStepRightGrid => {
                let direction = match action {
                    CursorStepUpGrid => Direction::Up,
                    CursorStepDownGrid => Direction::Down,
                    CursorStepRightGrid => Direction::Right,
                    CursorStepLeftGrid => Direction::Left,
                    _ => unreachable!(),
                };

                self.cursor_movement(CursorMovement::StepGrid(direction));
            }

            CursorStepLeftCharThenGrid | CursorStepRightCharThenGrid => {
                let direction = match action {
                    CursorStepRightCharThenGrid => Direction::Right,
                    CursorStepLeftCharThenGrid => Direction::Left,
                    _ => unreachable!(),
                };

                self.cursor_movement(CursorMovement::StepCharThenGrid(direction));
            }

            CursorDashUpCharThenGrid
            | CursorDashRightCharThenGrid
            | CursorDashDownCharThenGrid
            | CursorDashLeftCharThenGrid => {
                let direction = match action {
                    CursorDashUpCharThenGrid => Direction::Up,
                    CursorDashDownCharThenGrid => Direction::Down,
                    CursorDashRightCharThenGrid => Direction::Right,
                    CursorDashLeftCharThenGrid => Direction::Left,
                    _ => unreachable!(),
                };

                self.cursor_movement(CursorMovement::DashUntilBoundsOrNonEmpty(direction));
            }
        }
        Ok(Revert::None)
    }
}

impl View for GridView {
    fn title() -> String {
        String::from("Grid")
    }

    fn view_type() -> ViewType {
        ViewType::Pane
    }

    fn input_sink_action(input: String) -> Option<AppAction> {
        Some(AppAction::GridAction(GridAction::InsertOverflow(input)))
    }
}
