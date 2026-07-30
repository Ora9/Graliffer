use std::ops::Add;

use crate::{Action, AnyAction, State};

#[derive(Debug)]
#[must_use = "this `Revert` may be an `Apply` variant, which should be handled"]
pub enum Revert {
    Apply(Apply),
    None,
}

impl Revert {
    pub fn new(action: impl Action) -> Self {
        Self::Apply(Apply::new(action))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Revert::None)
    }

    pub fn is_apply(&self) -> bool {
        matches!(self, Revert::Apply(_))
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

impl From<Vec<Revert>> for Revert {
    fn from(reverts: Vec<Revert>) -> Self {
        reverts.into_iter().fold(Revert::None, |mut acc, revert| {
            acc.extend(revert);
            acc
        })
    }
}

#[derive(Debug)]
pub struct Apply(Vec<AnyAction>);

impl Apply {
    pub fn new(action: impl Action) -> Self {
        Self(vec![AnyAction::new(action)])
    }

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }
}

impl From<Vec<AnyAction>> for Apply {
    fn from(value: Vec<AnyAction>) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct Undoable {
    apply: Apply,
    revert: Revert,
}

#[derive(Debug)]
pub enum TimelineError<E> {
    ActionError(E),
    NothingToUndo,
}

#[derive(Debug)]
pub struct Timeline<S>
where
    S: State,
{
    state: S,
    undoes: Vec<Undoable>,
    cursor: usize,
}

impl<S: State> Timeline<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            undoes: Vec::new(),
            cursor: 0,
        }
    }

    pub fn act(&mut self, action: impl Into<S::Action>) -> Result<(), TimelineError<S::Error>> {
        let action = action.into();

        match self.state.act(action.clone()) {
            Ok(revert) => {
                self.append(Undoable {
                    apply: Apply::new(action),
                    revert,
                });

                Ok(())
            }
            Err(err) => Err(TimelineError::ActionError(err)),
        }
    }

    fn append(&mut self, undoable: Undoable) {
        self.undoes.truncate(self.cursor);
        self.undoes.push(undoable);
        self.cursor = self.cursor.checked_add(1).unwrap();

        // dbg!(self);
    }
}
