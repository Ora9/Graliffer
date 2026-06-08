use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

/// A system to merge consecutive and close text insertion or deletion in
/// undo / redo history, making it a bit more blocky and usable.
///
/// Internaly it uses two separate timeouts to track insertion and deletion.
/// And provide through `should_merge_insertion` and `should_merge_insertion` a
/// decision to append to the history or merge with last history entry
///
/// `None` or any already passed timestamp would mean to create a new
/// history entry
#[derive(Debug, Default)]
pub struct HistoryMerge {
    insertion_timeout: Option<Instant>,
    deletion_timeout: Option<Instant>,
}

impl HistoryMerge {
    /// The default maximum duration between two consecutive insertion/deletion
    const MERGE_TIMEOUT: Duration = Duration::from_secs(3);

    pub fn should_merge_insertion(&self) -> bool {
        self.insertion_timeout
            .is_some_and(|timeout| Instant::now().checked_duration_since(timeout).is_none())
    }

    pub fn should_merge_deletion(&self) -> bool {
        self.deletion_timeout
            .is_some_and(|timeout| Instant::now().checked_duration_since(timeout).is_none())
    }

    pub fn update_insertion_timeout(&mut self) {
        self.insertion_timeout = Instant::now().checked_add(HistoryMerge::MERGE_TIMEOUT);
    }

    pub fn update_deletion_timeout(&mut self) {
        self.deletion_timeout = Instant::now().checked_add(HistoryMerge::MERGE_TIMEOUT);
    }

    pub fn cancel_insertion_merge(&mut self) {
        self.insertion_timeout = None;
    }

    pub fn cancel_deletion_merge(&mut self) {
        self.deletion_timeout = None;
    }

    pub fn cancel_all_merge(&mut self) {
        self.cancel_insertion_merge();
        self.cancel_deletion_merge();
    }
}
