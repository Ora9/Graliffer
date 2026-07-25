use std::{any::Any, ops::Deref};

use crate::Action;

#[derive(Debug, Clone)]
pub struct AnyAction(Box<dyn Action>);

impl Action for AnyAction {}

impl Deref for AnyAction {
    type Target = Box<dyn Action>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AnyAction {
    pub fn new(action: impl Action) -> Self {
        Self(Box::new(action))
    }

    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        (self.0.deref() as &dyn Any).downcast_ref()
    }

    pub fn downcast<T: Any>(self) -> Result<Box<T>, Box<dyn Any>> {
        (self.0 as Box<dyn Any>).downcast()
    }
}
