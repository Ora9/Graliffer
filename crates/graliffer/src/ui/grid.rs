use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Margin, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

#[derive(Debug)]
pub struct GridState {}

impl GridState {
    pub fn new() -> Self {
        GridState {}
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

        // grid_block.render(grid_area, frame.buffer_mut());

        let viewport_area = area.inner(Margin::from(1));

        let paragraph = Paragraph::new("Yey grid");

        paragraph.render(viewport_area, buf);
        grid_block.render(area, buf);
    }
}
