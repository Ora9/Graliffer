use crate::{Apply, FromState, IntoState, State};

#[derive(Debug)]
#[must_use = "this `Revert` may be an `Apply` variant, which should be handled"]
pub enum Revert<S: State> {
    Apply(Apply<S>),
    None,
}

impl<S: State> Revert<S> {
    pub fn new(action: S::Action) -> Self {
        Self::Apply(Apply::new(action))
    }

    #[must_use]
    pub fn is_none(&self) -> bool {
        matches!(self, Revert::None)
    }

    #[must_use]
    pub fn is_apply(&self) -> bool {
        matches!(self, Revert::Apply(_))
    }

    pub fn apply(self) -> Option<Apply<S>> {
        match self {
            Revert::Apply(apply) => Some(apply),
            Revert::None => None,
        }
    }

    /// Push a `Revert` to `self`
    ///
    /// Same as [`Self::extend()`]
    pub fn push(&mut self, other: Self) {
        self.extend(other);
    }

    /// Extend `self` with another `Revert`
    pub fn extend(&mut self, other: Self) {
        match other {
            Self::None => {} // nothing to extend
            Self::Apply(rhs) => match self {
                Self::None => *self = Self::Apply(rhs),
                Self::Apply(lhs) => lhs.extend(rhs),
            },
        }
    }
}

impl<S1: State, S2: State> FromState<Revert<S1>> for Revert<S2>
where
    <S2 as State>::Action: From<<S1 as State>::Action>,
{
    fn from_state(value: Revert<S1>) -> Self {
        match value {
            Revert::Apply(apply) => Revert::Apply(apply.into_state()),
            Revert::None => Revert::None,
        }
    }
}

impl<S: State> From<Apply<S>> for Revert<S> {
    fn from(value: Apply<S>) -> Self {
        Self::Apply(value)
    }
}

impl<S: State> From<Vec<Revert<S>>> for Revert<S> {
    fn from(reverts: Vec<Revert<S>>) -> Self {
        reverts.into_iter().fold(Revert::None, |mut acc, revert| {
            acc.extend(revert);
            acc
        })
    }
}
