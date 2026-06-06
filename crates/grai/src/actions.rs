use std::{any::Any, fmt::Debug};

pub trait State {
    type Action;
    type Error;

    fn act(&mut self, action: &Self::Action) -> Result<Revert, Self::Error>;
}

pub trait Action: Any + Debug {}

pub enum Revert {
    Action(ActionBox),
    None,
}

impl Revert {
    pub fn new(action: impl Action) -> Self {
        Self::Action(ActionBox::new(action))
    }
}

pub enum ActionBox {
    None,
    Action(Box<dyn Action>),
}

impl ActionBox {
    pub fn new(action: impl Action) -> Self {
        Self::Action(Box::new(action))
    }
}
