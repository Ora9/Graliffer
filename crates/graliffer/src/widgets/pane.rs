use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    symbols::merge::MergeStrategy,
    text::Line,
    widgets::{Block, BorderType, Widget},
};

use crate::{
    Context, MenuLineAlignement, MenuTitle, NumberPrefix, ViewId,
    widgets::{MenuLine, MenuLinePosition},
};

#[derive(Debug)]
pub struct Pane {
    view_id: ViewId,
    menu_lines: Vec<MenuLine>,
}

impl Pane {
    pub fn new(view_id: ViewId, context: Context) -> Self {
        // TODO: determine numberprefix based on keymap

        let menu_line = MenuLine::from_title(MenuTitle::NumberPrefix {
            title: view_id.to_string(),
            style: Style::new(),
            prefix: NumberPrefix::Num2,
            highlighted: context.focus() == view_id,
        });

        Pane {
            view_id,
            menu_lines: vec![menu_line],
        }
    }

    pub fn add_menu_line(mut self, menu_line: MenuLine) -> Self {
        self.menu_lines.push(menu_line);
        self
    }
    //
    //     pub fn add_title_menu(self, context: Context) -> Self {
    //         self.add_menu_line(menu_line)
    //     }
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
