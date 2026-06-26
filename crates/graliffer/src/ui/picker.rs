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

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {}
}
