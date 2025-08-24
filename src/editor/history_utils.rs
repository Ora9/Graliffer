use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use crate::{
    Editor,
    action::EditorAction,
    editor::{EventContext, events::InputEvent},
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

#[derive(Clone)]
pub enum HistoryAction {
    Undo,
    Redo,
}

impl EditorAction for HistoryAction {
    fn act(&self, editor: &mut Editor) {
        let mut frame = editor
            .frame
            .lock()
            .expect("Should be able to get the frame");

        match self {
            Self::Redo => {
                editor.history.redo(&mut frame);
            }
            Self::Undo => {
                editor.history.undo(&mut frame);
            }
        }
    }

    fn events_and_context(&self) -> Option<(InputEvent, EventContext)> {
        match self {
            Self::Redo => Some((
                InputEvent::Key {
                    key: egui::Key::Y,
                    modifiers: egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
                },
                EventContext::None,
            )),
            Self::Undo => Some((
                InputEvent::Key {
                    key: egui::Key::Z,
                    modifiers: egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
                },
                EventContext::None,
            )),
        }
    }

    fn text(&self) -> (Option<&'static str>, Option<&'static str>) {
        match self {
            Self::Redo => (Some("Redo"), Some("Redo the last undone operation")),
            Self::Undo => (
                Some("Undo"),
                Some("Undo the last grid operation or evaluation step"),
            ),
        }
    }
}

impl Debug for HistoryAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Redo => {
                write!(f, "HistoryAction::Redo")
            }
            Self::Undo => {
                write!(f, "HistoryAction::Undo")
            }
        }
    }
}
