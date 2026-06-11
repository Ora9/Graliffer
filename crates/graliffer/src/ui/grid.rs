use std::{cell::RefCell, rc::Rc};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Offset, Rect, Spacing},
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
        let mut grid_buf = Buffer::empty(area);

        let frame = state
            .frame
            .try_borrow()
            .expect("could not borrow the frame");

        for cell_x in 0..(in_view_right - in_view_left) {
            for cell_y in 0..(in_view_bottom - in_view_top) {
                let cell_area = Rect {
                    x: (viewport_area.x as usize + (cell_x * (cell_width + border))) as u16,
                    y: (viewport_area.y as usize + (cell_y * (cell_height + border))) as u16,
                    width: (cell_width + border * 2) as u16,
                    height: (cell_height + border * 2) as u16,
                };

                let block = Block::bordered()
                    .fg(Color::DarkGray)
                    .merge_borders(MergeStrategy::Exact);

                let pos = grai::Position::from_numeric(cell_x as u32, cell_y as u32)
                    .expect("should be able to construct a valid position");

                Paragraph::new(frame.grid.get(pos).content())
                    .block(block)
                    .white()
                    .render(cell_area, &mut grid_buf);
            }
        }

        let _ = frame;

        buf.merge(&grid_buf);
        grid_block.render(area, buf);
    }
}
