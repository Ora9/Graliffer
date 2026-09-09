use crate::{Apply, Revert, State, TimelineError};

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
    /// Truncate any action after the cursor (the "redo" part), then push and increment cursor
    pub fn truncate_and_push(&mut self, undoable: Undoable<S>) {
        self.undoes.truncate(self.cursor);
        self.undoes.push(undoable);
        self.cursor_at_end();
    }

    fn cursor_at_end(&mut self) {
        self.cursor = self.undoes.len();
    }

    fn decrement_cursor(&mut self) -> Result<(), TimelineError> {
        self.cursor = self
            .cursor
            .checked_sub(1)
            .ok_or(TimelineError::NothingToUndo)?;

        Ok(())
    }

    fn increment_cursor(&mut self) -> Result<(), TimelineError> {
        if let Some(cursor) = self.cursor.checked_add(1)
            && cursor < self.undoes.len()
        {
            self.cursor = cursor;
            Ok(())
        } else {
            Err(TimelineError::NothingToRedo)
        }
    }

    fn get_current_undoable(&self) -> &Undoable<S> {
        // SAFETY: cursor is always in bound
        self.undoes.get(self.cursor).unwrap()
    }

    pub fn undo(&mut self) -> Result<&Apply<S>, TimelineError> {
        self.decrement_cursor()?;
        Ok(&self.get_current_undoable().revert)
    }

    pub fn redo(&mut self) -> Result<&Apply<S>, TimelineError> {
        self.increment_cursor()?;
        Ok(&self.get_current_undoable().apply)
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
