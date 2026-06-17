use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEvent, MouseEvent, MouseEventKind};
use grai::granary::GranaryDigit;
use log::debug;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Margin, Offset, Position, Rect},
    style::{Color, Stylize},
    symbols::merge::MergeStrategy,
    text::Line,
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

#[derive(Debug, Default)]
pub struct Cursor(grai::Position);

#[derive(Debug)]
enum DragState {
    Idle,
    Dragging {
        start_pointer_pos: Position,
        start_offset_x: usize,
        start_offset_y: usize,
    },
}

impl DragState {
    fn start_drag(&mut self, pointer_position: Position, offset_x: usize, offset_y: usize) {
        *self = Self::Dragging {
            start_pointer_pos: pointer_position,
            start_offset_x: offset_x,
            start_offset_y: offset_y,
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
                start_offset_x: _,
                start_offset_y: _
            }
        )
    }

    fn idle(&self) -> bool {
        matches!(self, DragState::Idle)
    }
}

#[derive(Debug)]
pub struct GridState {
    frame: Rc<RefCell<grai::Frame>>,

    layout: Option<Rect>,

    drag_state: DragState,

    cursor: Cursor,
    offset_x: usize,
    offset_y: usize,
}

impl GridState {
    pub fn new(frame: Rc<RefCell<grai::Frame>>) -> Self {
        GridState {
            frame,

            layout: None,

            drag_state: DragState::Idle,
            cursor: Cursor::default(),

            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Right => {
                self.offset_x = self.offset_x.saturating_add(1);
            }
            KeyCode::Left => {
                self.offset_x = self.offset_x.saturating_sub(1);
            }
            KeyCode::Down => {
                self.offset_y = self.offset_y.saturating_add(1);
            }
            KeyCode::Up => {
                self.offset_y = self.offset_y.saturating_sub(1);
            }
            _ => {}
        }
    }

    pub fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {
        // debug!("{:?}", mouse_event);
        let Some(viewport_area) = self.layout() else {
            return;
        };

        let pointer_pos = Position {
            x: mouse_event.column.saturating_sub(viewport_area.top()),
            y: mouse_event.row.saturating_sub(viewport_area.left()),
        };
        debug!("{pointer_pos}");

        match mouse_event.kind {
            MouseEventKind::Drag(button) if button.is_left() => {
                if self.drag_state.idle() {
                    self.drag_state
                        .start_drag(pointer_pos, self.offset_x, self.offset_y);
                    debug!("start dragging");
                }

                if let DragState::Dragging {
                    start_pointer_pos,
                    start_offset_x,
                    start_offset_y,
                } = self.drag_state
                {
                    let pointer_x_delta =
                        (start_pointer_pos.x as i16).saturating_sub_unsigned(pointer_pos.x);

                    let pointer_y_delta =
                        (start_pointer_pos.y as i16).saturating_sub_unsigned(pointer_pos.y);

                    self.offset_x = start_offset_x.saturating_add_signed(pointer_x_delta as isize);
                    self.offset_y = start_offset_y.saturating_add_signed(pointer_y_delta as isize);

                    // let x_offset = (drag_from.x as i16).saturating_sub_unsigned(pointer_pos.x);
                    // let y_offset = (drag_from.y as i16).saturating_sub_unsigned(pointer_pos.y);

                    // self.offset_x = self.offset_x.saturating_add_signed(x_offset as isize);
                    // self.offset_y = self.offset_y.saturating_add_signed(y_offset as isize);

                    debug!("grid_offset x: {}, y: {}", self.offset_x, self.offset_y);
                }
            }
            _ => {
                if self.drag_state.dragging() {
                    self.drag_state.stop_drag();
                    debug!("stopped dragging");
                }
            }
        }
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
    type State = GridState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let grid_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(Line::from(vec![
                "┤".into(),
                "¹".blue().into(),
                "Grid".into(),
                "├".into(),
            ]))
            .title(
                Line::from(vec![
                    "┤".into(),
                    "²".blue().into(),
                    "Stack".into(),
                    "├".into(),
                ])
                .alignment(Alignment::Center),
            )
            .title(
                Line::from(vec![
                    "┤".into(),
                    "²".blue().into(),
                    "Stack".into(),
                    "├".into(),
                ])
                .alignment(Alignment::Center),
            );

        let pane_viewport = area.inner(Margin::from(1));
        state.layout = Some(pane_viewport);

        let cell_height = 1;
        let cell_width = 3;
        let border = 1;

        // A separate buffer is used to render the grid,
        // this is used to mask everything that is outside of the grid widget viewport
        // this is because widget drawn outside the buffer are clamped to the border, but we want to
        // have widgets drawn partialy onto the viewport
        let overdraw_cells = 1;
        let overdraw_margin = Margin::new(
            (cell_width * overdraw_cells * 2) as u16,
            (cell_height * overdraw_cells * 2) as u16,
        );
        let mut overdraw_buf = Buffer::empty(
            pane_viewport
                .offset(Offset::new(
                    overdraw_margin.horizontal as i32,
                    overdraw_margin.vertical as i32,
                ))
                .outer(overdraw_margin),
        );
        let overdraw_viewport = overdraw_buf.area().inner(overdraw_margin);

        let in_view_top = (state.offset_y / (cell_height + border)).saturating_sub(overdraw_cells);
        let in_view_left = (state.offset_x / (cell_width + border)).saturating_sub(overdraw_cells);

        let in_view_bottom = state
            .offset_y
            .saturating_add(overdraw_viewport.height as usize)
            .saturating_div(cell_height + border)
            .saturating_add(overdraw_cells)
            .min(GranaryDigit::MAX_NUMERIC as usize);

        let in_view_right = state
            .offset_x
            .saturating_add(overdraw_viewport.width as usize)
            .saturating_div(cell_width + border)
            .saturating_add(overdraw_cells)
            .min(GranaryDigit::MAX_NUMERIC as usize);

        let frame = state
            .frame
            .try_borrow()
            .expect("could not borrow the frame");

        for cell_x in in_view_left..in_view_right {
            for cell_y in in_view_top..in_view_bottom {
                let x = (overdraw_viewport.x as usize)
                    .saturating_add(cell_x * (cell_width + border))
                    .saturating_sub(state.offset_x) as u16;

                let y = (overdraw_viewport.y as usize)
                    .saturating_add(cell_y * (cell_height + border))
                    .saturating_sub(state.offset_y) as u16;

                let width = (cell_width + border * 2) as u16;
                let height = (cell_height + border * 2) as u16;

                let cell_area = Rect::new(x, y, width, height);

                let block = Block::bordered()
                    .fg(Color::DarkGray)
                    .merge_borders(MergeStrategy::Exact);

                let grid_pos = grai::Position::from_numeric(cell_x as u32, cell_y as u32)
                    .expect("should be able to construct a valid position");

                Paragraph::new(frame.grid.get(grid_pos).content())
                    .block(block)
                    .reset()
                    .render(cell_area, &mut overdraw_buf);
            }
        }

        let _ = frame;

        grid_block.render(area, buf);

        // our own implementation of Buffer::merge
        buffer_merge_areas(
            buf,
            pane_viewport.as_position(),
            &overdraw_buf,
            overdraw_viewport,
        );
    }
}

fn buffer_merge_areas(
    dest_buf: &mut Buffer,
    dest_pos: Position,
    from_buf: &Buffer,
    from_area: Rect,
) {
    let size = from_area.area();
    for y in from_area.y..(from_area.y + from_area.height) {
        for x in from_area.x..(from_area.x + from_area.width) {
            let from_pos = Position::new(x, y);
            let from_cell = from_buf.cell(from_pos);

            let dest_pos = dest_pos.offset(Offset::new(
                x.saturating_sub(from_area.left()) as i32,
                y.saturating_sub(from_area.top()) as i32,
            ));

            let mut dest_cell = dest_buf.cell_mut(dest_pos);

            if let Some(mut dest_cell) = dest_cell
                && let Some(from_cell) = from_cell
            {
                dest_cell.set_symbol(from_cell.symbol());
                dest_cell.set_style(from_cell.style());
            }
        }
    }
}
