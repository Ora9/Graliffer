use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crate::{
    Editor,
    EditorAction,
    // editor::{EventContext, events::InputEvent},
};

/// A timeout for the next acceptable text input that would be
/// merged in undo history. This is used to merge closely entered
/// text input (timewise), and make undo/redo a bit less granular
///
/// `None` or any already passed timestamp would mean to create a new
/// history entry
#[derive(Debug, Default)]
pub struct HistoryMerge {
    input_timeout: Option<Instant>,
    deletion_timeout: Option<Instant>,
}

impl HistoryMerge {
    const MERGE_TIMEOUT: Duration = Duration::from_secs(3);

    pub fn should_merge_input(&self) -> bool {
        self.input_timeout
            .is_some_and(|timeout| Instant::now().checked_duration_since(timeout).is_none())
    }

    pub fn should_merge_deletion(&self) -> bool {
        self.deletion_timeout
            .is_some_and(|timeout| Instant::now().checked_duration_since(timeout).is_none())
    }

    pub fn update_input_timeout(&mut self) {
        self.input_timeout = Instant::now().checked_add(HistoryMerge::MERGE_TIMEOUT);
    }

    pub fn update_deletion_timeout(&mut self) {
        self.deletion_timeout = Instant::now().checked_add(HistoryMerge::MERGE_TIMEOUT);
    }

    pub fn cancel_input_merge(&mut self) {
        self.input_timeout = None;
    }

    pub fn cancel_deletion_merge(&mut self) {
        self.deletion_timeout = None;
    }

    pub fn cancel_all_merge(&mut self) {
        self.cancel_input_merge();
        self.cancel_deletion_merge();
    }
}
