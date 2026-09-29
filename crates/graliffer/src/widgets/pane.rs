use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::merge::MergeStrategy,
    text::Line,
    widgets::{Block, BorderType, Widget},
};

use crate::{
    AppAction, Context, MenuLineAlignement, MenuTitle, ViewId,
    widgets::{MenuLine, MenuLinePosition},
};

#[derive(Debug)]
pub struct Pane {
    view_titles: MenuLine,
    menu_lines: Vec<MenuLine>,
}

impl Pane {
    pub fn new(views: Vec<(ViewId, AppAction)>, context: &Context) -> Self {
        let mut view_titles = MenuLine::default();

        for view in views {
            view_titles =
                view_titles.push_title(MenuTitle::from_pane_title(view.0, view.1, context))
        }

        Pane {
            view_titles,
            menu_lines: Vec::new(),
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

        // ignore MenuLinePosition and MenuLineAlignement
        block = block.title_top(self.view_titles.as_border());

        for menu_line in &self.menu_lines {
            let mut line = Line::from(menu_line.as_border());

            match menu_line.alignement {
                MenuLineAlignement::Left => line = line.left_aligned(),
                MenuLineAlignement::Center => line = line.centered(),
                MenuLineAlignement::Right => line = line.right_aligned(),
            };

            // Block titles with the same alignement and position are rendered in order, following
            // each other with a space (or border char) in between them
            match menu_line.position {
                MenuLinePosition::Top => block = block.title_top(line),
                MenuLinePosition::Bottom => block = block.title_bottom(line),
            }
        }

        block.render(area, buf);
    }
}
