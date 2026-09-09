use std::slice::Iter;

use crate::{Action, FromState, TimelinedState};

#[derive(Debug)]
pub struct Apply<S: TimelinedState>(Vec<S::Action>);

impl<S: TimelinedState> Clone for Apply<S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<S: TimelinedState> Apply<S> {
    pub fn new(action: S::Action) -> Self {
        Self(vec![action])
    }

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    pub fn iter(&self) -> impl Iterator<Item = &S::Action> {
        self.0.iter()
    }

    // pub fn into_iter(self) -> impl Iterator<Item = S::Action> {
    //     self.0.into_iter()
    // }
}

// pub fn into_iter(&self) -> Iter<>

impl<S: TimelinedState> IntoIterator for Apply<S> {
    type Item = S::Action;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

// impl<S: TimelinedState> IntoIterator for Apply<S> {
//     type Item = &S::Action;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.iter()
//     }
// }

impl<S1: TimelinedState, S2: TimelinedState> FromState<Apply<S1>> for Apply<S2>
where
    <S2 as TimelinedState>::Action: From<<S1 as TimelinedState>::Action>,
{
    fn from_state(value: Apply<S1>) -> Self {
        Self(value.0.into_iter().map(Into::into).collect())
    }
}
