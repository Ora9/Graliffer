use crate::{Action, FromState, State};

#[derive(Debug)]
pub struct Apply<S: State>(Vec<S::Action>);

impl<S: State> Clone for Apply<S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<S: State> Apply<S> {
    pub fn new(action: S::Action) -> Self {
        Self(vec![action])
    }

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl<S: State> IntoIterator for Apply<S> {
    type Item = S::Action;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<S1: State, S2: State> FromState<Apply<S1>> for Apply<S2>
where
    <S2 as State>::Action: From<<S1 as State>::Action>,
{
    fn from_state(value: Apply<S1>) -> Self {
        Self(value.0.into_iter().map(Into::into).collect())
    }
}
