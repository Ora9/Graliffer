use crate::{Apply, Revert, State};

#[derive(Debug)]
pub struct Undoable<S: State> {
    pub apply: Apply<S>,
    pub revert: Apply<S>,
}

#[derive(Debug)]
pub struct Undoes<S: State> {
    undoes: Vec<Undoable<S>>,
    cursor: usize,
}

impl<S: State> Default for Undoes<S> {
    fn default() -> Self {
        Self {
            undoes: Vec::default(),
            cursor: usize::default(),
        }
    }
}

impl<S: State> Undoes<S> {
    pub fn append(&mut self, undoable: Undoable<S>) {
        self.undoes.truncate(self.cursor);
        self.undoes.push(undoable);
        self.cursor = self.cursor.checked_add(1).unwrap();
    }

    pub fn into_reverts(self) -> Revert<S> {
        self.undoes
            .into_iter()
            .fold(Revert::None, |mut acc, undoable| {
                acc.extend(undoable.revert.into());
                acc
            })
    }
}
