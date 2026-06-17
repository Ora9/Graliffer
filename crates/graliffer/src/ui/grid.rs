use std::{cell::RefCell, rc::Rc};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Margin, Offset, Position, Rect},
    style::{Color, Stylize},
    symbols::{border, merge::MergeStrategy},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

// struct CellWidget {
//     cell: grai::Cell,
// }

// impl CellWidget {
//     const WIDTH: usize = 5;
//     const HEIGHT: usize = 3;

//     pub fn new(cell: grai::Cell) -> Self {
//         Self { cell }
//     }
// }

// impl Widget for CellWidget {
//     fn render(self, area: Rect, buf: &mut Buffer)
//     where
//         Self: Sized,
//     {
//         let borders = Block::bordered().merge_borders(MergeStrategy::Exact);

//         Paragraph::new(self.cell.content())
//             .block(borders)
//             .render(area, buf);
//     }
// }

#[derive(Debug, Default)]
pub struct Cursor(grai::Position);

#[derive(Debug)]
pub struct GridState {
    frame: Rc<RefCell<grai::Frame>>,

    cursor: Cursor,
    offset_x: usize,
    offset_y: usize,
}

impl GridState {
    pub fn new(frame: Rc<RefCell<grai::Frame>>) -> Self {
        GridState {
            frame,

            cursor: Cursor::default(),

            offset_x: 0,
            offset_y: 0,
        }
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
            .white()
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

        let viewport_area = area.inner(Margin::from(1));

        state.offset_x = 3;
        state.offset_y = 6;

        let cell_height = 1;
        let cell_width = 3;
        let border = 1;

        let in_view_top = (state.offset_y / (cell_height + border));
        let in_view_left = (state.offset_x / (cell_width + border));

        let in_view_bottom =
            viewport_area.height as usize / (cell_height + border) + in_view_top + 1;
        let in_view_right = viewport_area.width as usize / (cell_width + border) + in_view_left + 1;

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

        buf.merge(&grid_buf);
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
