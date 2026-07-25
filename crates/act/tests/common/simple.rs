use std::{mem, ops::Deref};

use act::{Action, Revert, State};

#[derive(Debug, Clone)]
pub enum SimpleAction {
    IncrementFoo,
    DecrementFoo,
    SetBar(String),
    ToggleBaz,
}

impl Action for SimpleAction {}

#[derive(Debug)]
pub enum TestError {}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Simple {
    pub foo: u8,
    pub bar: String,
    pub baz: bool,
}

impl State for Simple {
    type Action = SimpleAction;
    type Error = TestError;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<Revert, Self::Error> {
        match action.into() {
            SimpleAction::IncrementFoo => {
                self.foo = self.foo.wrapping_add(1);

                Ok(Revert::new(SimpleAction::DecrementFoo))
            }
            SimpleAction::DecrementFoo => {
                self.foo = self.foo.wrapping_sub(1);

                Ok(Revert::new(SimpleAction::IncrementFoo))
            }
            SimpleAction::SetBar(new_bar) => {
                let old_bar = mem::replace(&mut self.bar, new_bar);

                Ok(Revert::new(SimpleAction::SetBar(old_bar)))
            }
            SimpleAction::ToggleBaz => {
                self.baz = !self.baz;

                Ok(Revert::new(SimpleAction::ToggleBaz))
            }
        }
    }
}
