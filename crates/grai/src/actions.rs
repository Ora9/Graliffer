use std::{any::Any, fmt::Debug};

pub trait State {
    type Action;
    type Error;

    fn act(&mut self, action: &Self::Action) -> Result<(), Self::Error>;
}

pub trait Action: Any + Debug {}
