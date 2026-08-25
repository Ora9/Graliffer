use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{List, StatefulWidget, Widget},
};

use crate::{View, ViewType};

#[derive(Debug)]
pub struct StackView {
    frame: grai::FrameGuard,
}

impl StackView {
    pub fn new(frame: grai::FrameGuard) -> Self {
        StackView { frame }
    }
}

impl View for StackView {
    fn title() -> String {
        String::from("Stack")
    }

    fn view_type() -> ViewType {
        ViewType::Pane
    }
}

#[derive(Debug)]
pub struct StackWidget;

impl StackWidget {
    pub fn new() -> Self {
        StackWidget
    }
}

impl StatefulWidget for StackWidget {
    type State = StackView;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut list_items: Vec<String> = state
            .frame
            .read(|frame| frame.stack.clone().into_iter())
            .map(|operand| format!("- {}", operand.to_string()))
            .collect();

        list_items.insert(0, "Head #1 :".to_string());

        Widget::render(List::new(list_items), area, buf);
    }
}
