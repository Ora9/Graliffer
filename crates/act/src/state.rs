use std::fmt::Debug;

use crate::Action;

pub trait State: Debug {
    type Action: Action + Clone;
    type Error;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<(), Self::Error>
    where
        Self: Sized;
}
