use ratatui::{
    buffer::Buffer,
    layout::{Margin, Position, Rect, Size},
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
        // remember to test with other areas (not positioned at 0,0)

        Popup::new("top_left", Size::new(30, 10))
            .position(PopupPosition::Edge {
                side: Align2::TOP_LEFT,
                margin: Margin::new(5, 3),
            })
            .render(area, buf);
        Popup::new("top_center", Size::new(30, 10))
            .position(PopupPosition::Edge {
                side: Align2::TOP_CENTER,
                margin: Margin::new(5, 3),
            })
            .render(area, buf);
        Popup::new("top_right", Size::new(30, 10))
            .position(PopupPosition::Edge {
                side: Align2::TOP_RIGHT,
                margin: Margin::new(5, 3),
            })
            .render(area, buf);

        Popup::new("center_left", Size::new(30, 10))
            .position(PopupPosition::Edge {
                side: Align2::CENTER_LEFT,
                margin: Margin::new(5, 3),
            })
            .render(area, buf);
        Popup::new("center_center", Size::new(30, 10))
            .position(PopupPosition::Edge {
                side: Align2::CENTER_CENTER,
                margin: Margin::new(5, 3),
            })
            .render(area, buf);
        Popup::new("center_right", Size::new(30, 10))
            .position(PopupPosition::Edge {
                side: Align2::CENTER_RIGHT,
                margin: Margin::new(5, 3),
            })
            .render(area, buf);

        Popup::new("bottom_left", Size::new(30, 10))
            .position(PopupPosition::Edge {
                side: Align2::BOTTOM_LEFT,
                margin: Margin::new(5, 3),
            })
            .render(area, buf);
        Popup::new("bottom_center", Size::new(30, 10))
            .position(PopupPosition::Edge {
                side: Align2::BOTTOM_CENTER,
                margin: Margin::new(5, 3),
            })
            .render(area, buf);
        Popup::new("bottom_right", Size::new(30, 10))
            .position(PopupPosition::Edge {
                side: Align2::BOTTOM_RIGHT,
                margin: Margin::new(5, 3),
            })
            .render(area, buf);

        Popup::new("at", Size::new(30, 10))
            .position(PopupPosition::At {
                position: Position { x: 10, y: 6 },
                anchor: Align2::TOP_LEFT,
            })
            .render(area, buf);
    }
}
