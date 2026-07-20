use action::State;

use crate::{AnyAppAction, KeyContextPredicate};

mod console;
pub use console::*;

mod grid;
pub use grid::*;

mod picker;
pub use picker::*;

#[derive(Debug, Default)]
pub struct InsertBindingList(Vec<InsertBinding>);

impl InsertBindingList {
    pub fn push(&mut self, insert_binding: InsertBinding) {
        self.0.push(insert_binding);
    }
}

impl From<Vec<InsertBinding>> for InsertBindingList {
    fn from(value: Vec<InsertBinding>) -> Self {
        Self(value)
    }
}

impl From<InsertBinding> for InsertBindingList {
    fn from(value: InsertBinding) -> Self {
        Self(vec![value])
    }
}

#[derive(Debug)]
pub struct InsertBinding {
    pub action: AnyAppAction,
    pub context: KeyContextPredicate,
}

pub trait Focusable: State {
    #[allow(unused)]
    fn insert_sink_binding(input: String) -> InsertBindingList {
        InsertBindingList::default()
    }
}
