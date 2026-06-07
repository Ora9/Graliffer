use std::{any::Any, cell::RefCell, fmt::Debug, rc::Rc};

use crate::{Cell, Frame, Position};

pub trait State: Debug {
    type Action;
    type Error;

    fn act(&mut self, action: &Self::Action) -> Result<Revert, Self::Error>;
}

pub trait Action: Any + Debug {}

#[derive(Debug)]
pub struct ActionBox(Box<dyn Action>);

impl ActionBox {
    pub fn new(action: impl Action) -> Self {
        Self(Box::new(action))
    }
}

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

pub struct Timeline<S>
where
    S: State,
{
    // act: Box<dyn FnMut(&mut S, S::Action) -> Result<Revert, S::Error>>,
    state: Rc<RefCell<S>>,
}

impl<S: State> Timeline<S> {
    pub fn new(state: Rc<RefCell<S>>) -> Self {
        Self { state }
    }

    pub fn test(&self, action: S::Action) -> Result<Revert, S::Error> {
        let mut state = self.state.try_borrow_mut().unwrap();

        let r = state.act(&action);
        dbg!(&state);

        r
        // state.act(S::Action::new

        //     Box::new(crate::GridAction::Set(
        //     Position::from_numeric(5, 20).unwrap(),
        //     Cell::new_trim("yey"),
        // )));

        // self.act()
        // panic!()
    }
}
