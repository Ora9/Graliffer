use std::{any::Any, fmt::Debug};

pub trait Action: Any + ActionClone + Debug {}

pub trait ActionClone {
    fn dyn_clone(&self) -> Box<dyn Action>;
}

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
