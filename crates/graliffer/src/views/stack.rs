use crate::{View, ViewType};

pub struct StackView {}

impl View for StackView {
    fn title() -> String {
        String::from("Stack")
    }

    fn view_type() -> ViewType {
        ViewType::Pane
    }
}
