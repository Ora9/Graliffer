use std::{any::Any, fmt::Debug};

pub trait State {
    type Action;

    fn act(&mut self, action: &Self::Action);
}

pub trait Action: Any + Debug {}
