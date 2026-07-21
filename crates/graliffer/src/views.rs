use std::collections::HashMap;

use crate::{AnyAppAction, App, KeyContextPredicate};

mod grid;
use action::State;
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

mod app_render;

impl App {
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
    pub action: AnyAppAction,
    pub context: KeyContextPredicate,
}

// pub struct ViewRegistery(HashMap<ViewId, ViewInfo>);

// impl ViewRegistery {
//     pub fn new() -> Self {
//         Self(HashMap::default())
//     }
// }

// pub struct ViewInfo {
//     title: String,
//     insert_binding_list: InsertBindingList,
// }

pub enum ViewType {
    Pane,
    Popup,
}

// pub struct View<T: State>(T);

// pub struct V(HashMap<ViewId, Box<View<dyn State>>>);

pub trait View {
    fn title() -> String;
    fn view_type() -> ViewType;

    fn view_id() -> ViewId {
        match Self::view_type() {
            ViewType::Pane => PaneId::from(Self::title().as_str()).into(),
            ViewType::Popup => PopupId::from(Self::title().as_str()).into(),
        }
    }

    // fn input_sink_binding_list(input: String) -> InputSinkBindingList {
    //     InputSinkBindingList::none()
    // }

    #[allow(unused)]
    fn input_sink_action(input: String) -> Option<AnyAppAction> {
        None
    }
}
