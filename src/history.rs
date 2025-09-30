use crate::{Frame, FrameAction};
use std::fmt::Debug;

#[derive(Clone)]
struct ReciprocalAction {
    redo: Option<FrameAction>,
    undo: Option<FrameAction>,
}

impl Debug for ReciprocalAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReciprocalAction (\n    undo: {:?},\n    redo: {:?}\n)",
            self.undo, self.redo
        )
    }
}

#[derive(Clone)]
pub struct Artifact {
    actions: Vec<ReciprocalAction>,
}

impl Artifact {
    pub const EMPTY: Self = Self {
        actions: Vec::new(),
    };

    fn new(redo: Option<FrameAction>, undo: Option<FrameAction>) -> Self {
        Self {
            actions: vec![ReciprocalAction { redo, undo }],
        }
    }

    pub fn from_redo(redo: FrameAction) -> Self {
        Self::new(Some(redo), None)
    }

    pub fn from_redo_undo(redo: FrameAction, undo: FrameAction) -> Self {
        Self::new(Some(redo), Some(undo))
    }

    pub fn push(&mut self, other: Self) {
        self.actions.extend(other.actions);
    }

    // fn add_action(&mut self, action: ReciprocalAction) {
    //     self.actions.push(action);
    // }

    pub fn last_redo_action(&self) -> Option<FrameAction> {
        self.actions.last().and_then(|action| action.redo.clone())
    }

    pub fn last_undo_action(&self) -> Option<FrameAction> {
        self.actions.last().and_then(|action| action.undo.clone())
    }

    fn redo(&self, frame: &mut Frame) {
        for action in self.actions.iter() {
            if let Some(redo) = action.redo.to_owned() {
                let _ = frame.act(redo.to_owned());
            }
        }
    }

    fn undo(&self, frame: &mut Frame) {
        for action in self.actions.iter().rev() {
            if let Some(undo) = action.undo.to_owned() {
                let _ = frame.act(undo);
            }
        }
    }

    // fn invert_actions(&self) -> Self {
    //     Self {
    //         actions: self.to_owned().actions.into_iter().rev().map(|action| action.invert_actions()).collect()
    //     }
    // }
}

impl Debug for Artifact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Artifact {:#?}", self.actions)
    }
}

#[derive(Default)]
pub struct History {
    artifacts: Vec<Artifact>,
    cursor: usize,
}

impl History {
    // pub fn new() -> Self {
    //     Self {
    //         artifacts: Vec::new(),
    //         cursor: 0,
    //     }
    // }

    pub fn append(&mut self, artifact: Artifact) {
        // Don't append empty artifacts
        if artifact.actions.is_empty() {
            return;
        }

        self.artifacts.truncate(self.cursor);
        self.artifacts.push(artifact);
        self.cursor = self.cursor.saturating_add(1);

        // self.cursor = self.artifacts.len();
    }

    pub fn merge_with_last(&mut self, artifact: Artifact) {
        // Don't merge empty artifacts
        if artifact.actions.is_empty() {
            return;
        }

        if let Some(last_artifact) = self.artifacts.last_mut() {
            last_artifact.push(artifact);
        } else {
            self.append(artifact);
        }
    }

    pub fn undo(&mut self, frame: &mut Frame) -> Artifact {
        if let Some(last_artifact) = self.cursor.checked_sub(1)
            && let Some(artifact) = self.artifacts.get(last_artifact)
        {
            artifact.undo(frame);
            self.cursor = last_artifact;

            artifact.clone()
        } else {
            Artifact::EMPTY
        }
    }

    /// Redo the last undone action, and return the artifact
    pub fn redo(&mut self, frame: &mut Frame) -> Artifact {
        if let Some(artifact) = self.artifacts.get(self.cursor) {
            artifact.redo(frame);

            // Append the action of redoing
            // self.artifacts.push(artifact.to_owned());
            self.cursor = self.cursor.saturating_add(1);

            artifact.clone()
        } else {
            Artifact::EMPTY
        }

        // // skip empty artifacts
        // if self.cursor == self.artifacts.len().saturating_sub(1) { return; }

        // if let Some(artifact) = self.artifacts.get(self.cursor) {
        //     artifact.redo(frame);
        //     self.cursor = self.cursor.saturating_add(1);
        // }
    }
}

impl Debug for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "History (\n    cursor: {},\n    artifacts: {:#?})",
            self.cursor, self.artifacts
        )
    }
}
