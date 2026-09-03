use std::{
    convert::Infallible,
    ops::{Div, Neg},
};

use act::{Action, Revert, State};
use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use grai::Direction;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Offset, Position, Rect, Size},
    style::{Color, Modifier, Style, Stylize},
    symbols::merge::MergeStrategy,
    text::Span,
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};
use serde::{Deserialize, Serialize};
use tui_input::InputRequest;

use crate::{
    AppAction, Context, CursorMovement, FollowCursorConfig, FollowCursorMode, GridInput,
    GutterSizeConfig, View, ViewType,
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
                let config_margin = config
                    .follow_cursor_sticky_margin
                    .try_into()
                    .unwrap_or(u16::MAX);

                let margin_x = (CELL_WIDTH + CELL_BORDER)
                    .saturating_mul(config_margin)
                    .max(1);

                let margin_y = (CELL_HEIGHT + CELL_BORDER)
                    .saturating_mul(config_margin)
                    .max(1);

                let cursor_box = Rect::new(
                    self.x as u16,
                    self.y as u16,
                    area.width.saturating_sub(1),
                    area.height.saturating_sub(1),
                )
                .inner(Margin::new(margin_x, margin_y));

                let offset_x = if cursor_box.width <= (CELL_WIDTH + CELL_BORDER * 2) {
                    // default to Centered mode
                    cursor_term_pos.x.saturating_sub(area.width.div(2)) as u32
                } else {
                    let right = cursor_term_pos
                        .x
                        .saturating_add(CELL_WIDTH)
                        .saturating_sub(cursor_box.right());

                    let left = cursor_box
                        .left()
                        .saturating_sub(cursor_term_pos.x.saturating_sub(CELL_BORDER));

                    self.x
                        .saturating_add(right as u32)
                        .saturating_sub(left as u32)
                };

                let offset_y = if cursor_box.height <= (CELL_HEIGHT + CELL_BORDER * 2) {
                    // default to Centered mode
                    cursor_term_pos.y.saturating_sub(area.height.div(2)) as u32
                } else {
                    let top = cursor_box
                        .top()
                        .saturating_sub(cursor_term_pos.y.saturating_sub(CELL_BORDER));

                    let bottom = cursor_term_pos
                        .y
                        .saturating_add(CELL_HEIGHT)
                        .saturating_sub(cursor_box.bottom());

                    self.y
                        .saturating_add(bottom as u32)
                        .saturating_sub(top as u32)
                };

                self.with(offset_x, offset_y, area);
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
    context: Context,

    frame: grai::FrameGuard,

    grid_input: GridInput,
    grid_offset: GridOffset,
    drag_state: DragState,

    layouts: Option<GridLayout>,
}

#[derive(Debug, Clone, Copy)]
pub struct GridLayout {
    pub grid_area: Rect,
    pub horizontal_gutter_area: Option<Rect>,
    pub vertical_gutter_area: Option<Rect>,
}

impl GridLayout {
    pub fn union(&self) -> Rect {
        let mut union = self.grid_area;

        if let Some(x_gutter) = self.horizontal_gutter_area {
            union = union.union(x_gutter);
        };

        if let Some(y_gutter) = self.vertical_gutter_area {
            union = union.union(y_gutter);
        };

        union
    }
}

impl GridView {
    pub fn new(frame: grai::FrameGuard, context: Context) -> Self {
        let grid_input = frame.read(|frame| GridInput::new(&frame.grid));

        GridView {
            context,
            frame,

            grid_input,
            grid_offset: GridOffset::default(),

            layouts: None,

            drag_state: DragState::Idle,
        }
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        let Some(view_layout) = self.layouts() else {
            return;
        };

        let pointer_pos = Position {
            x: mouse_event.column,
            y: mouse_event.row,
        };

        match mouse_event.kind {
            MouseEventKind::Down(mouse_button) if mouse_button == MouseButton::Left => {
                if let Some(grid_pos) =
                    terminal_to_grid_position(pointer_pos, view_layout.grid_area, self.grid_offset)
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
                    view_layout.grid_area,
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
                        view_layout.grid_area,
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

        if let Some(view_layout) = self.layouts() {
            self.grid_offset
                .follow_cursor(&self.grid_input, view_layout.grid_area, config);
        };
    }

    pub fn handle_insert(&mut self, input: char) -> Revert {
        let revert = self
            .frame
            .write(|frame| self.grid_input.insert(&mut frame.grid, input));

        self.follow_cursor();
        revert
    }

    pub fn handle_input_request(&mut self, input_request: InputRequest) -> Revert {
        let revert = self
            .frame
            .write(|frame| self.grid_input.handle(&mut frame.grid, input_request));

        self.follow_cursor();
        revert
    }

    pub fn cursor_movement(&mut self, movement: CursorMovement) {
        self.frame
            .read(|frame| self.grid_input.with_movement(movement, &frame.grid));

        self.follow_cursor();
    }

    pub fn layouts(&self) -> Option<GridLayout> {
        self.layouts
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

    fn render(self, view_area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let gutter_config = state.context.config(|config| config.grid.gutter);
        let gutter_margin = match gutter_config.size {
            GutterSizeConfig::Proportional => Margin {
                horizontal: 2,
                vertical: 1,
            },
            GutterSizeConfig::Minimal => Margin {
                horizontal: 1,
                vertical: 1,
            },
        };

        let (grid_area, horizontal_gutter_area, vertical_gutter_area) = if gutter_config.show {
            let [horizontal_gutter_area, horizontal] = view_area.layout(&Layout::vertical(vec![
                Constraint::Length(gutter_margin.vertical),
                Constraint::Fill(1),
            ]));

            let [vertical_gutter_area, grid_area] = horizontal.layout(&Layout::horizontal(vec![
                Constraint::Length(gutter_margin.horizontal),
                Constraint::Fill(1),
            ]));

            (
                grid_area,
                Some(horizontal_gutter_area),
                Some(vertical_gutter_area),
            )
        } else {
            (view_area, None, None)
        };

        state.layouts = Some(GridLayout {
            grid_area,
            horizontal_gutter_area,
            vertical_gutter_area,
        });

        // A separate buffer is used to render the grid,
        // this is used to mask everything that is outside of the grid widget viewport
        // this is because widget drawn outside the buffer are clamped to the border, but we want to
        // have widgets drawn partialy onto the viewport
        const OVERDRAW_CELL: u16 = 1;
        let overdraw_margin = Margin::new(
            (CELL_WIDTH + CELL_BORDER * 2 * OVERDRAW_CELL) as u16,
            (CELL_HEIGHT + CELL_BORDER * 2 * OVERDRAW_CELL) as u16,
        );

        let overdraw_grid_area = Rect {
            x: grid_area.x.saturating_sub(view_area.x),
            y: grid_area.y.saturating_sub(view_area.y),
            width: grid_area.width,
            height: grid_area.height,
        }
        .offset(Offset::new(
            overdraw_margin.horizontal.into(),
            overdraw_margin.vertical.into(),
        ));

        let mut overdraw_buf = Buffer::empty(overdraw_grid_area.outer(overdraw_margin));

        let left_top_cell = terminal_to_grid_position(
            Position::new(grid_area.left(), grid_area.top()),
            grid_area,
            state.grid_offset,
        )
        .unwrap_or(grai::Position::MIN);

        let right_bottom_cell = terminal_to_grid_position(
            Position::new(grid_area.right(), grid_area.bottom()),
            grid_area,
            state.grid_offset,
        )
        .unwrap_or(grai::Position::MAX);

        for cell_x in left_top_cell.x()..=right_bottom_cell.x() {
            for cell_y in left_top_cell.y()..=right_bottom_cell.y() {
                let grid_pos = grai::Position::from_numeric(cell_x as u32, cell_y as u32)
                    .expect("should be able to construct a valid position");

                let term_pos =
                    grid_to_terminal_position(grid_pos, overdraw_grid_area, state.grid_offset);

                let cell_area = Rect::from((
                    term_pos,
                    Size {
                        width: CELL_WIDTH + CELL_BORDER * 2,
                        height: CELL_HEIGHT + CELL_BORDER * 2,
                    },
                ));

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

        if let Some(horizontal_gutter_area) = horizontal_gutter_area {
            for cell_x in left_top_cell.x()..=right_bottom_cell.x() {
                let x_coord = grai::granary::GranaryDigit::from_numeric(cell_x)
                    .expect("should be able to construct a valid position");

                let term_pos = grid_to_terminal_position(
                    grai::Position::from_granary_digits(x_coord, grai::granary::GranaryDigit::MIN),
                    grid_area,
                    state.grid_offset,
                )
                .offset(Offset {
                    x: (CELL_BORDER + CELL_WIDTH / 2).into(),
                    y: 1,
                });

                if term_pos.x < grid_area.left() || term_pos.x >= grid_area.right() {
                    continue;
                }

                let area = Rect {
                    x: term_pos.x,
                    y: horizontal_gutter_area.y,
                    width: 1,
                    height: 1,
                };

                let fg = if x_coord == state.grid_input.grid_cursor().granary_x() {
                    Color::White
                } else {
                    Color::DarkGray
                };

                Span::raw(x_coord.as_textual().to_string())
                    .fg(fg)
                    .render(area, buf);
            }
        }

        if let Some(vertical_gutter_area) = vertical_gutter_area {
            for cell_y in left_top_cell.y()..=right_bottom_cell.y() {
                let y_coord = grai::granary::GranaryDigit::from_numeric(cell_y)
                    .expect("should be able to construct a valid position");

                let term_pos = grid_to_terminal_position(
                    grai::Position::from_granary_digits(grai::granary::GranaryDigit::MIN, y_coord),
                    grid_area,
                    state.grid_offset,
                )
                .offset(Offset {
                    x: 0,
                    y: (CELL_BORDER).into(),
                });

                if term_pos.y < grid_area.top() || term_pos.y >= grid_area.bottom() {
                    continue;
                }

                let area = Rect {
                    x: vertical_gutter_area.x,
                    y: term_pos.y,
                    width: 1,
                    height: 1,
                };

                let fg = if y_coord == state.grid_input.grid_cursor().granary_y() {
                    Color::White
                } else {
                    Color::DarkGray
                };

                Span::raw(y_coord.as_textual().to_string())
                    .fg(fg)
                    .render(area, buf);
            }
        }

        let head_grid_pos = state.frame.read(|frame| frame.head.position);
        let head_term_pos =
            grid_to_terminal_position(head_grid_pos, overdraw_grid_area, state.grid_offset);
        let head_area = Rect::from((
            head_term_pos,
            Size {
                width: CELL_WIDTH + CELL_BORDER * 2,
                height: CELL_HEIGHT + CELL_BORDER * 2,
            },
        ));
        Block::bordered()
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(Color::White))
            .merge_borders(MergeStrategy::Fuzzy)
            .render(head_area, &mut overdraw_buf);

        let cursor_term_pos =
            cursor_to_terminal_position(&state.grid_input, overdraw_grid_area, state.grid_offset);
        if let Some(cursor_cell) = overdraw_buf.cell_mut(cursor_term_pos) {
            cursor_cell.fg = if state.grid_input.char_at_max() {
                Color::DarkGray
            } else {
                Color::White
            };
            cursor_cell.modifier = cursor_cell.modifier.union(Modifier::REVERSED);
        }

        // our own implementation of Buffer::merge
        buffer_merge_areas(
            &overdraw_buf,
            overdraw_grid_area,
            buf,
            grid_area.as_position(),
        );
    }
}

/// TODO: mabye open a pull request to ratatui to propose this buffer method
fn buffer_merge_areas(
    from_buf: &Buffer,
    from_area: Rect,
    dest_buf: &mut Buffer,
    dest_pos: Position,
) {
    for from_pos in from_area.positions() {
        let dest_pos = dest_pos.offset(Offset::new(
            from_pos.x.saturating_sub(from_area.x) as i32,
            from_pos.y.saturating_sub(from_area.y) as i32,
        ));

        if let Some(from_cell) = from_buf.cell(from_pos)
            && let Some(dest_cell) = dest_buf.cell_mut(dest_pos)
        {
            dest_cell.set_symbol(from_cell.symbol());
            dest_cell.set_style(from_cell.style());
        }
    }
}

#[derive(Debug, Clone, strum::EnumString, Serialize, Deserialize)]
pub enum GridAction {
    Set(String),

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

        let revert = match action {
            Insert(input) => {
                let mut revert = Revert::None;

                for c in input.chars() {
                    revert.extend(self.handle_insert(c));
                }

                revert
            }

            InsertOverflow(input) => {
                let mut revert = Revert::None;

                for c in input.chars() {
                    if self.grid_input.char_at_max() || c == ' ' {
                        self.cursor_movement(CursorMovement::StepGrid(Direction::Right));
                    }

                    revert.extend(self.handle_insert(c));
                }

                revert
            }

            DeletePrevCharOrStepLeftGrid => {
                if self.grid_input.char_cursor() != 0 {
                    self.handle_input_request(InputRequest::DeletePrevChar)
                } else {
                    self.cursor_movement(CursorMovement::StepGrid(Direction::Left));
                    Revert::None
                }
            }

            // todo: use the newer DeleteFromStart
            DeleteTillStart => self.handle_input_request(InputRequest::DeletePrevWord),

            DeleteTillStartOrStepLeftGrid => {
                if self.grid_input.char_cursor() != 0 {
                    self.handle_input_request(InputRequest::DeletePrevWord)
                } else {
                    self.cursor_movement(CursorMovement::StepGrid(Direction::Left));
                    Revert::None
                }
            }

            DeletePrevChar => self.handle_input_request(InputRequest::DeletePrevChar),

            DeleteNextChar => self.handle_input_request(InputRequest::DeleteNextChar),

            CursorStepUpGrid | CursorStepDownGrid | CursorStepLeftGrid | CursorStepRightGrid => {
                let direction = match action {
                    CursorStepUpGrid => Direction::Up,
                    CursorStepDownGrid => Direction::Down,
                    CursorStepRightGrid => Direction::Right,
                    CursorStepLeftGrid => Direction::Left,
                    _ => unreachable!(),
                };

                self.cursor_movement(CursorMovement::StepGrid(direction));

                Revert::None
            }

            CursorStepLeftCharThenGrid | CursorStepRightCharThenGrid => {
                let direction = match action {
                    CursorStepRightCharThenGrid => Direction::Right,
                    CursorStepLeftCharThenGrid => Direction::Left,
                    _ => unreachable!(),
                };

                self.cursor_movement(CursorMovement::StepCharThenGrid(direction));

                Revert::None
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

                Revert::None
            }
        };

        Ok(revert)
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
