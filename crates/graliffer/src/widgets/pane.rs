use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::merge::MergeStrategy,
    widgets::{Block, BorderType, Widget},
};

use crate::widgets::{MenuLine, MenuLinePosition};

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
            match menu_line.position {
                MenuLinePosition::Top => block = block.title_top(menu_line.as_border()),
                MenuLinePosition::Bottom => block = block.title_bottom(menu_line.as_border()),
            }
        }

        block.render(area, buf);
    }
}
