use crate::{AppAction, AppWidget, KeyContextPredicate};

mod grid;
pub use grid::*;

mod console;
pub use console::*;

mod stack;
pub use stack::*;

mod picker;
pub use picker::*;

mod about;
pub use about::*;

mod view_id;
pub use view_id::*;

impl AppWidget {
    pub fn register_views(&mut self) {}
}

#[derive(Debug, Default)]
pub struct InputSinkBindingList(Vec<InputSinkBinding>);

impl InputSinkBindingList {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn push(&mut self, insert_binding: InputSinkBinding) {
        self.0.push(insert_binding);
    }
}

impl From<Vec<InputSinkBinding>> for InputSinkBindingList {
    fn from(value: Vec<InputSinkBinding>) -> Self {
        Self(value)
    }
}

impl From<InputSinkBinding> for InputSinkBindingList {
    fn from(value: InputSinkBinding) -> Self {
        Self(vec![value])
    }
}

#[derive(Debug)]
pub struct InputSinkBinding {
    pub action: AppAction,
    pub context: KeyContextPredicate,
}

pub enum ViewType {
    Pane,
    Popup,
}

pub trait View {
    fn title() -> String;
    fn view_type() -> ViewType;

    fn view_id() -> ViewId {
        match Self::view_type() {
            ViewType::Pane => PaneId::from(Self::title().as_str()).into(),
            ViewType::Popup => PopupId::from(Self::title().as_str()).into(),
        }
    }

    #[allow(unused)]
    fn input_sink_action(input: String) -> Option<AppAction> {
        None
    }
}
