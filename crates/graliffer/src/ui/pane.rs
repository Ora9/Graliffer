use log::debug;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::merge::MergeStrategy,
    text::Line,
    widgets::{Block, BorderType, Widget},
};

use crate::ui::{MenuLine, MenuLineAlignement, MenuLinePosition, MenuTitle};

#[derive(Debug)]
pub struct PaneBorder<'a> {
    menu_lines: Vec<MenuLine<'a>>,
    // bottom_menu_bar: Option<MenuLine<'a>>,
    // pub menu_title: MenuTitle,
}

impl<'a> PaneBorder<'a> {
    pub fn new() -> Self {
        PaneBorder {
            menu_lines: Vec::default(),
            // bottom_menu_bar: None,
        }
    }

    pub fn add_menu_line(mut self, menu_line: MenuLine<'a>) -> Self {
        self.menu_lines.push(menu_line);
        self
    }
}

impl<'a> Widget for PaneBorder<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use ratatui::symbols::line;

        let mut block = Block::bordered()
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Fuzzy);

        for menu_line in self.menu_lines {
            match menu_line.position {
                MenuLinePosition::Top => block = block.title_top(menu_line.as_border()),
                MenuLinePosition::Bottom => block = block.title_bottom(menu_line.as_border()),
            }
        }

        block.render(area, buf);
    }
}
