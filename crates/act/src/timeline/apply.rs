use crate::{FromState, State};

#[derive(Debug)]
pub struct Apply<S: State>(Vec<S::Action>);

impl<S: State> Apply<S> {
    pub fn new(action: S::Action) -> Self {
        Self(vec![action])
    }

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl<S1: State, S2: State> FromState<Apply<S1>> for Apply<S2>
where
    <S2 as State>::Action: From<<S1 as State>::Action>,
{
    fn from_state(value: Apply<S1>) -> Self {
        Self(value.0.into_iter().map(|action| action.into()).collect())
    }
}
