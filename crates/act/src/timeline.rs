use std::ops::{Deref, DerefMut};

use crate::{Action, AnyAction, State};

pub trait ConvertState<T> {
    fn convert_state(value: T) -> Self;
}

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

impl<S1: State, S2: State> ConvertState<Revert<S1>> for Revert<S2>
where
    <S2 as State>::Action: From<<S1 as State>::Action>,
{
    fn convert_state(value: Revert<S1>) -> Self {
        match value {
            Revert::Apply(apply) => Revert::Apply(ConvertState::convert_state(apply)),
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

impl<S1: State, S2: State> ConvertState<Apply<S1>> for Apply<S2>
where
    <S2 as State>::Action: From<<S1 as State>::Action>,
{
    fn convert_state(value: Apply<S1>) -> Self {
        Self(value.0.into_iter().map(|action| action.into()).collect())
    }
}

#[derive(Debug)]
pub struct Undoable<S: State> {
    apply: Apply<S>,
    revert: Apply<S>,
}

#[derive(Debug, Default)]
pub struct Undoes<S: State> {
    undoes: Vec<Undoable<S>>,
    cursor: usize,
}

impl<S: State> Undoes<S> {
    fn append(&mut self, undoable: Undoable<S>) {
        self.undoes.truncate(self.cursor);
        self.undoes.push(undoable);
        self.cursor = self.cursor.checked_add(1).unwrap();
    }

    fn into_reverts(self) -> Revert<S> {
        self.undoes
            .into_iter()
            .fold(Revert::None, |mut acc, undoable| {
                acc.extend(undoable.revert.into());
                acc
            })
    }
}

#[derive(Debug)]
pub enum TimelineError {
    // ActionError(E),
    NothingToUndo,
}

#[derive(Debug)]
pub struct Timeline<S: State> {
    state: S,
    undoes: Undoes<S>,
}

// impl<S: State> Deref for Timeline<S> {
//     type Target = S;

//     fn deref(&self) -> &Self::Target {
//         &self.state
//     }
// }

// impl<S: State> DerefMut for Timeline<S> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.state
//     }
// }

impl<S: State + Default> Timeline<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            undoes: Undoes::default(),
        }
    }

    pub fn act(&mut self, action: impl Into<S::Action>) -> Result<(), S::Error> {
        let action = action.into();

        self.state.act(action.clone()).map(|revert| {
            if let Revert::Apply(revert) = revert {
                self.append(Undoable {
                    apply: Apply::new(action),
                    revert,
                });
            }
        })
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut S {
        &mut self.state
    }

    pub fn undoes(&self) -> &Undoes<S> {
        &self.undoes
    }

    fn append(&mut self, undoable: Undoable<S>) {
        self.undoes.append(undoable);
    }

    pub fn into_revert(self) -> Revert<S> {
        self.undoes.into_reverts()
    }
}

pub struct TimelineRef<'a, S>
where
    S: State,
{
    state: &'a mut S,
    undoes: Undoes<S>,
}

impl<S: State> Deref for TimelineRef<'_, S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.state
    }
}

impl<'a, S: State + Default> TimelineRef<'a, S> {
    pub fn new(state: &'a mut S) -> Self {
        Self {
            state,
            undoes: Undoes::default(),
        }
    }

    pub fn act(&mut self, action: impl Into<S::Action>) -> Result<(), S::Error> {
        let action = action.into();

        self.state.act(action.clone()).map(|revert| {
            if let Revert::Apply(revert) = revert {
                self.append(Undoable {
                    apply: Apply::new(action),
                    revert,
                });
            }
        })
    }

    fn append(&mut self, undoable: Undoable<S>) {
        self.undoes.append(undoable);
    }

    pub fn into_revert(self) -> Revert<S> {
        self.undoes.into_reverts()
    }
}
