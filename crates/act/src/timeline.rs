use std::ops::{Deref, DerefMut};

use crate::{Action, TimelinedState};

mod apply;
pub use apply::*;

mod revert;
pub use revert::*;

mod undoes;
use undoes::*;

pub trait FromState<T> {
    fn from_state(value: T) -> Self;
}

pub trait IntoState<T> {
    fn into_state(self) -> T;
}

impl<T, U> IntoState<U> for T
where
    U: FromState<T>,
{
    fn into_state(self) -> U {
        U::from_state(self)
    }
}

#[derive(Debug)]
pub enum TimelineError {
    // ActionError(E),
    NothingToUndo,
    NothingToRedo,
}

#[derive(Debug)]
pub struct Timeline<S: TimelinedState> {
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

impl<S: TimelinedState> Timeline<S> {
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
                self.truncate_and_push(Undoable {
                    apply: Apply::new(action),
                    revert,
                });
            }
        })
    }

    fn act_apply(&mut self, apply: Apply<S>) -> Result<(), S::Error> {
        for action in apply.into_iter() {
            self.state.act(action);
        }

        Ok(())
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

    pub fn into_revert(self) -> Revert<S> {
        self.undoes.into_reverts()
    }

    pub fn undo(&mut self) -> Result<(), TimelineError> {
        let apply = self.undoes.undo()?.clone();
        self.act_apply(apply);

        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), TimelineError> {
        let apply = self.undoes.redo()?.clone();

        self.act_apply(apply);

        Ok(())
    }

    fn truncate_and_push(&mut self, undoable: Undoable<S>) {
        self.undoes.truncate_and_push(undoable);
    }
}

pub struct TimelineRef<'a, S>
where
    S: TimelinedState,
{
    state: &'a mut S,
    undoes: Undoes<S>,
}

impl<S: TimelinedState> Deref for TimelineRef<'_, S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.state
    }
}

impl<'a, S: TimelinedState> TimelineRef<'a, S> {
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
        self.undoes.truncate_and_push(undoable);
    }

    pub fn into_revert(self) -> Revert<S> {
        self.undoes.into_reverts()
    }
}
