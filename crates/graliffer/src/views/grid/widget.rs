use std::{
    convert::Infallible,
    ops::{Div, Neg},
};

use act::{Action, Revert, State};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use grai::Direction;
use log::debug;
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Offset, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::merge::MergeStrategy,
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};
use serde::{Deserialize, Serialize};
use tui_input::InputRequest;

use crate::{
    AppAction, Context, CursorMovement, FollowCursorConfig, FollowCursorMode, GridInput, View,
    ViewType,
};

const CELL_WIDTH: u16 = 3;
const CELL_HEIGHT: u16 = 1;
const CELL_BORDER: u16 = 1;

fn terminal_to_grid_position(
    terminal_position: Position,
    area: Rect,
    offset: GridOffset,
) -> Option<grai::Position> {
    grai::Position::new(
        terminal_position
            .x
            .checked_add(offset.x as u16)?
            .saturating_sub(area.x)
            .saturating_sub(CELL_BORDER)
            .checked_div(CELL_WIDTH + CELL_BORDER)? as u32,
        terminal_position
            .y
            .checked_add(offset.y as u16)?
            .saturating_sub(area.y)
            .saturating_sub(CELL_BORDER)
            .checked_div(CELL_HEIGHT + CELL_BORDER)? as u32,
    )
    .ok()
}

fn cursor_to_terminal_position(
    grid_input: &GridInput,
    area: Rect,
    grid_offset: GridOffset,
) -> Position {
    let cursor_term_origin = grid_to_terminal_position(grid_input.grid_cursor(), area, grid_offset);

    Position::new(cursor_term_origin.x, cursor_term_origin.y)
        .offset(Offset::new(CELL_BORDER as i32, CELL_BORDER as i32))
        .offset(Offset::new(grid_input.char_cursor() as i32, 0))
}

fn grid_to_terminal_position(
    grid_position: grai::Position,
    area: Rect,
    offset: GridOffset,
) -> Position {
    Position {
        x: area
            .x
            .strict_add(grid_position.x() as u16 * (CELL_WIDTH + CELL_BORDER))
            .saturating_sub(offset.x as u16),
        y: area
            .y
            .strict_add(grid_position.y() as u16 * (CELL_HEIGHT + CELL_BORDER))
            .saturating_sub(offset.y as u16),
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct GridOffset {
    x: u32,
    y: u32,
}

impl GridOffset {
    pub fn with(&mut self, x: u32, y: u32, area: Rect) {
        let max = grid_to_terminal_position(grai::Position::MAX, area, GridOffset::default())
            .offset(Offset {
                x: (area
                    .width
                    .saturating_sub(5)
                    .saturating_sub(CELL_WIDTH + CELL_BORDER) as i32)
                    .neg(),
                y: (area
                    .height
                    .saturating_sub(2)
                    .saturating_sub(CELL_HEIGHT + CELL_BORDER) as i32)
                    .neg(),
            });

        self.x = x.clamp(0, max.x as u32);
        self.y = y.clamp(0, max.y as u32);
    }

    pub fn follow_cursor(
        &mut self,
        grid_input: &GridInput,
        area: Rect,
        config: FollowCursorConfig,
    ) {
        // use the current offset to determine what side of the screen we should stick to
        // by calculating least travel?

        let cursor_term_pos = cursor_to_terminal_position(grid_input, area, GridOffset::default())
            .offset(Offset {
                x: (area.x as i32).neg(),
                y: (area.y as i32).neg(),
            });

        match config.follow_cursor_mode {
            FollowCursorMode::Centered => {
                self.with(
                    cursor_term_pos.x.saturating_sub(area.width.div(2)) as u32,
                    cursor_term_pos.y.saturating_sub(area.height.div(2)) as u32,
                    area,
                );
            }
            FollowCursorMode::Sticky => {
                let margin_x = ((CELL_WIDTH + CELL_BORDER) as u32)
                    .saturating_mul(config.follow_cursor_sticky_margin.0);
                let margin_y = ((CELL_HEIGHT + CELL_BORDER) as u32)
                    .saturating_mul(config.follow_cursor_sticky_margin.0);

                let top = (self.y.saturating_add(margin_y + CELL_BORDER as u32))
                    .saturating_sub(cursor_term_pos.y as u32);

                let right = (cursor_term_pos.x as u32).saturating_sub(
                    self.x
                        .saturating_add(area.width as u32)
                        .saturating_sub(margin_x + (CELL_WIDTH + CELL_BORDER) as u32),
                );

                let bottom = (cursor_term_pos.y as u32).saturating_sub(
                    self.y
                        .saturating_add(area.height as u32)
                        .saturating_sub(margin_y + (CELL_HEIGHT + CELL_BORDER) as u32),
                );

                let left = (self.x.saturating_add(margin_x + CELL_BORDER as u32))
                    .saturating_sub(cursor_term_pos.x as u32);

                debug!(
                    "cursor {:?}, offset {:?}, margin_x: {}, margin_y: {}",
                    cursor_term_pos, self, margin_x, margin_y
                );

                self.with(
                    self.x.saturating_sub(left).saturating_add(right),
                    self.y.saturating_sub(top).saturating_add(bottom),
                    area,
                );

                debug!("{}, {}, {}, {}", top, right, bottom, left);
            }
        }
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
        //     "{:?}, {:?}",
        //     pointer_pos,
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

                self.grid_offset.with(
                    self.grid_offset.x.saturating_add_signed(x_offset),
                    self.grid_offset.y.saturating_add_signed(y_offset),
                    viewport_area,
                );
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
                    self.grid_offset.with(
                        start_grid_offset.x.saturating_add_signed(
                            (start_pointer_pos.x as i16).saturating_sub_unsigned(pointer_pos.x)
                                as i32,
                        ),
                        start_grid_offset.y.saturating_add_signed(
                            (start_pointer_pos.y as i16).saturating_sub_unsigned(pointer_pos.y)
                                as i32,
                        ),
                        viewport_area,
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

    fn follow_cursor(&mut self) {
        let config = self.context.config(|config| config.grid.follow_cursor);

        if let Some(area) = self.layout() {
            self.grid_offset
                .follow_cursor(&self.grid_input, area, config);
        };
    }

    pub fn handle_insert(&mut self, input: char) {
        self.frame.write(|frame| {
            self.grid_input.insert(&mut frame.grid, input);
        });

        self.follow_cursor();
    }

    pub fn handle_input_request(&mut self, input_request: InputRequest) {
        self.frame.write(|frame| {
            self.grid_input.handle(&mut frame.grid, input_request);
        });

        self.follow_cursor();
    }

    pub fn cursor_movement(&mut self, movement: CursorMovement) {
        self.frame.read(|frame| {
            self.grid_input.with_movement(movement, &frame.grid);
        });

        self.follow_cursor();
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
        let overdraw_cells: u32 = 1;
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

        let left_top_cell = terminal_to_grid_position(
            Position::new(overdraw_area.left(), overdraw_area.top()),
            overdraw_area,
            state.grid_offset,
        )
        .unwrap_or(grai::Position::MIN);

        let right_bottom_cell = terminal_to_grid_position(
            Position::new(overdraw_area.right(), overdraw_area.bottom()),
            overdraw_area,
            state.grid_offset,
        )
        .unwrap_or(grai::Position::MAX);

        for cell_x in left_top_cell.x()..=right_bottom_cell.x() {
            for cell_y in left_top_cell.y()..=right_bottom_cell.y() {
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
            .border_style(Style::default().fg(Color::White))
            .merge_borders(MergeStrategy::Fuzzy)
            .render(head_area, &mut overdraw_buf);

        let cursor_term_pos =
            cursor_to_terminal_position(&state.grid_input, overdraw_area, state.grid_offset);

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
                    self.handle_insert(c);
                }
            }

            InsertOverflow(input) => {
                for c in input.chars() {
                    if self.grid_input.char_at_max() || c == ' ' {
                        self.cursor_movement(CursorMovement::StepGrid(Direction::Right));
                    }

                    // self.grid_input.insert(&mut frame.grid, c);
                    // self.handle_input_request(InputRequest::InsertChar(c));
                    self.handle_insert(c);
                }
            }

            DeletePrevCharOrStepLeftGrid => {
                if self.grid_input.char_cursor() != 0 {
                    self.handle_input_request(InputRequest::DeletePrevChar);
                } else {
                    self.cursor_movement(CursorMovement::StepGrid(Direction::Left));
                }
            }

            // todo: use the newer DeleteFromStart
            DeleteTillStart => {
                self.handle_input_request(InputRequest::DeletePrevWord);
            }

            DeleteTillStartOrStepLeftGrid => {
                if self.grid_input.char_cursor() != 0 {
                    self.handle_input_request(InputRequest::DeletePrevWord);
                } else {
                    self.cursor_movement(CursorMovement::StepGrid(Direction::Left));
                }
            }

            DeletePrevChar => {
                self.handle_input_request(InputRequest::DeletePrevChar);
            }

            DeleteNextChar => {
                self.handle_input_request(InputRequest::DeleteNextChar);
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
