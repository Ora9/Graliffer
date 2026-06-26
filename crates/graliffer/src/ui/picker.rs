use ratatui::{
    buffer::Buffer,
    layout::{Margin, Offset, Position, Rect, Size},
    widgets::{StatefulWidget, Widget},
};

use crate::ui::{Align2, Popup, PopupPosition};

#[derive(Debug)]
pub struct PickerItem {}

#[derive(Debug, Default)]
pub struct PickerState {
    items: Vec<PickerItem>,
    cursor: usize,
}

impl PickerState {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct Picker;

impl StatefulWidget for Picker {
    type State = PickerState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let width = 70;
        let height = 20;

        let popup = Popup::new(Size { width, height }).borders(Borders::empty());
        let popup_area = popup.area(area);

        let [input_area, item_area] = popup_area.layout(
            &Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)])
                .spacing(Spacing::Overlap(1)),
        );
        popup.render(area, buf);

        let input_block = Block::new()
            .borders(Borders::all())
            .border_set(border::ROUNDED)
            .merge_borders(MergeStrategy::Fuzzy)
            .render(input_area, buf);

        let items_block = Block::new()
            .borders(Borders::all())
            .border_set(border::ROUNDED)
            .merge_borders(MergeStrategy::Fuzzy)
            .render(item_area, buf);
    }
}
