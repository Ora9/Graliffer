use std::fmt::Debug;

use crate::Action;

pub trait TimelinedState: Debug {
    type Action: Action + Clone;
    type Error;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<super::Revert<Self>, Self::Error>
    where
        Self: Sized;
}
