use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::merge::MergeStrategy,
    text::Line,
    widgets::{Block, BorderType, Widget},
};

use crate::ui::{MenuBar, MenuTitle};

#[derive(Debug)]
pub struct PaneBorder {
    pub menu_bar: MenuBar,
    // pub menu_title: MenuTitle,
}

impl PaneBorder {
    pub fn new(menu_bar: MenuBar) -> Self {
        PaneBorder { menu_bar }
    }
}

impl Widget for PaneBorder {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use ratatui::symbols::line;

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .merge_borders(MergeStrategy::Fuzzy)
            .title(self.menu_bar.as_border());

        block.render(area, buf);
    }
}

#[derive(Debug)]
pub enum FocusedPane {
    Grid,
    Console,
    Stack,
}

impl FocusedPane {
    pub fn grid(&self) -> bool {
        matches!(self, Self::Grid)
    }

    pub fn console(&self) -> bool {
        matches!(self, Self::Console)
    }

    pub fn stack(&self) -> bool {
        matches!(self, Self::Stack)
    }
}
