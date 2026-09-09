use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Offset, Position, Rect, Size},
    style::{Color, Modifier, Style, Stylize},
    symbols::merge::MergeStrategy,
    text::Span,
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};
use tui_input::InputRequest;

use act::{Apply, Timeline};
use grai::FrameGuard;
use granary::GranaryDigit;

use crate::Context;

mod config;
pub use config::*;

mod input;
pub use input::*;

mod offset;
use offset::*;

mod utils;
use utils::*;

mod impl_state;
pub use impl_state::GridAction;

mod impl_view;

#[derive(Debug)]
pub struct GridView {
    context: Context,

    frame_timeline: Timeline<FrameGuard>,

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

            frame_timeline: Timeline::new(frame.clone()),

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
            MouseEventKind::Down(MouseButton::Left) => {
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

    pub fn handle_insert(&mut self, input: char) {
        self.grid_input.insert(&mut self.frame_timeline, input);
        self.follow_cursor();
    }

    pub fn handle_input_request(&mut self, input_request: InputRequest) {
        self.grid_input
            .handle(&mut self.frame_timeline, input_request);
        self.follow_cursor();
    }

    pub fn cursor_movement(&mut self, movement: CursorMovement) {
        self.frame_timeline
            .state()
            .read(|frame| self.grid_input.with_movement(movement, &frame.grid));

        self.follow_cursor();
    }

    pub fn layouts(&self) -> Option<GridLayout> {
        self.layouts
    }

    fn move_cursor_with_timeline(&mut self, apply: Apply<FrameGuard>) {
        if let Some(action) = apply.iter().last()
            && let grai::FrameAction::Grid(grid_action) = action
            && let grai::GridAction::Set(position, _) = grid_action
        {
            self.frame_timeline.state().read(|frame| {
                self.grid_input
                    .with_movement(CursorMovement::Jump(*position), &frame.grid);
            })
        }
    }

    fn undo(&mut self) {
        if let Ok(apply) = self.frame_timeline.undo() {
            self.move_cursor_with_timeline(apply)
        }
    }

    fn redo(&mut self) {
        if let Ok(apply) = self.frame_timeline.redo() {
            self.move_cursor_with_timeline(apply)
        }
    }
}

impl GridView {
    pub fn step(&mut self) {
        let _ = self.frame_timeline.act(grai::FrameAction::Step);
    }
}

#[derive(Debug, Default)]
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
            CELL_WIDTH + CELL_BORDER * 2 * OVERDRAW_CELL,
            CELL_HEIGHT + CELL_BORDER * 2 * OVERDRAW_CELL,
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
                let grid_pos = grai::Position::from_numeric(cell_x, cell_y)
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

                let cell_content = state
                    .frame_timeline
                    .state()
                    .read(|frame| frame.grid.get(grid_pos));

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
                let x_coord = GranaryDigit::from_numeric(cell_x)
                    .expect("should be able to construct a valid position");

                let term_pos = grid_to_terminal_position(
                    grai::Position::from_granary_digits(x_coord, GranaryDigit::MIN),
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
                let y_coord = GranaryDigit::from_numeric(cell_y)
                    .expect("should be able to construct a valid position");

                let term_pos = grid_to_terminal_position(
                    grai::Position::from_granary_digits(GranaryDigit::MIN, y_coord),
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

        let head_grid_pos = state
            .frame_timeline
            .state()
            .read(|frame| frame.head.position);
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
