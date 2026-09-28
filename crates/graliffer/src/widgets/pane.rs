use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::merge::MergeStrategy,
    text::Line,
    widgets::{Block, BorderType, Widget},
};

use crate::{
    MenuLineAlignement,
    widgets::{MenuLine, MenuLinePosition},
};

#[derive(Debug)]
pub struct Pane {
    menu_lines: Vec<MenuLine>,
}

impl Pane {
    pub fn new() -> Self {
        Pane {
            menu_lines: Vec::default(),
        }
    }

    pub fn add_menu_line(mut self, menu_line: MenuLine) -> Self {
        self.menu_lines.push(menu_line);
        self
    }
}

impl Widget for Pane {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut block = Block::bordered()
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Fuzzy);

        for menu_line in &self.menu_lines {
            let mut line = Line::from(menu_line.as_border());

            match menu_line.alignement {
                MenuLineAlignement::Left => line = line.left_aligned(),
                MenuLineAlignement::Center => line = line.centered(),
                MenuLineAlignement::Right => line = line.right_aligned(),
            };

            match menu_line.position {
                MenuLinePosition::Top => block = block.title_top(line),
                MenuLinePosition::Bottom => block = block.title_bottom(line),
            }
        }

        block.render(area, buf);
    }
}
