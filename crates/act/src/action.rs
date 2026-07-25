use std::{
    any::Any,
    cell::RefCell,
    fmt::Debug,
    ops::{Add, Deref},
    rc::Rc,
};

use crate::Revert;

pub trait Action: Any + ActionClone + Debug {}

pub trait ActionClone {
    fn dyn_clone(&self) -> Box<dyn Action>;
}

// impl<T: Action> From<T> for AnyAction {
//     fn from(value: T) -> Self {
//         AnyAction::new(value)
//     }
// }

impl<T: Clone + Action> ActionClone for T {
    fn dyn_clone(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}

impl Action for Box<dyn Action> {}

impl Clone for Box<dyn Action> {
    fn clone(&self) -> Self {
        (**self).dyn_clone()
    }
}
