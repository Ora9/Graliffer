use std::{any::Any, fmt::Debug};

pub trait State {
    type Action;
    type Error;

    fn act(&mut self, action: &Self::Action) -> Result<Revert, Self::Error>;
}

pub trait Action: Any + Debug {}

#[derive(Debug)]
pub enum Revert {
    Action(Apply),
    None,
}

impl Revert {
    pub fn new(action: impl Action) -> Self {
        Self::Action(Apply::new(action))
    }
}

#[derive(Debug)]
pub struct Apply(Box<dyn Action>);

impl Apply {
    pub fn new(action: impl Action) -> Self {
        Self(Box::new(action))
    }
}
