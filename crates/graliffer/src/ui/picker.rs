use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect, Size, Spacing},
    style::{
        Color::{Black, White},
        Style,
    },
    symbols::{border, merge::MergeStrategy},
    text::{Line, Text},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::ui::{Align2, Popup, PopupPosition};

#[derive(Debug, Clone)]
pub struct PickerItem {
    title: String,
}

impl PickerItem {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

impl From<String> for PickerItem {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl From<&str> for PickerItem {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Default)]
pub struct PickerState {
    items: Vec<PickerItem>,
    cursor: usize,
}

impl PickerState {
    pub fn new() -> Self {
        Self {
            items: vec![
                "lorem".into(),
                "ipsum".into(),
                "constructeris".into(),
                "sit".into(),
            ],
            cursor: 0,
        }
    }

    pub fn items_len(&self) -> usize {
        self.items.len()
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor = self.cursor.saturating_add(1).min(self.items_len());
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }
}

pub struct Picker;

impl StatefulWidget for Picker {
    type State = PickerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let width = 70;
        let height = 20;

        let popup = Popup::new(Size { width, height }).borders(Borders::empty());
        let popup_inner = popup.inner(area);

        let [input_area, item_area] = popup_inner.layout(
            &Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)])
                .spacing(Spacing::Overlap(1)),
        );

        let mut item_list: Vec<Line> = Vec::new();

        for (i, item) in state.items.iter().enumerate() {
            let style = if state.cursor == i {
                Style::new().bg(White).fg(Black)
            } else {
                Style::default()
            };

            let mut title = item.title.clone();
            title.insert(0, ' ');

            item_list.push(Line::raw(title).style(style));
        }

        // let mut items_list = Text::default();
        // items_list.lines = items;

        popup.render(area, buf);

        let border_block = Block::new()
            .borders(Borders::all())
            .border_set(border::ROUNDED)
            .merge_borders(MergeStrategy::Fuzzy);

        border_block.clone().render(input_area, buf);
        border_block.clone().render(item_area, buf);

        Text::from(item_list).render(border_block.inner(item_area), buf);
    }
}
