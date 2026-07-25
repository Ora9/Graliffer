use std::ops::Add;

use crate::{Action, AnyAction, State};

#[derive(Debug)]
pub enum Revert {
    Apply(Apply),
    None,
}

impl Revert {
    pub fn new(action: impl Action) -> Self {
        Self::Apply(Apply::new(action))
    }
}

#[derive(Debug)]
pub struct Apply(AnyAction);

impl Apply {
    pub fn new(action: impl Action) -> Self {
        Self(AnyAction::new(action))
    }
}

impl From<AnyAction> for Apply {
    fn from(value: AnyAction) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct Undoable {
    apply: Apply,
    revert: Revert,
}

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

    pub fn act(&mut self, action: S::Action) -> Result<(), TimelineError<S::Error>> {
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

        dbg!(self);
    }
}
