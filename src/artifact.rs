//! Artifact is the actions system of Graliffer, it is used to manipulate data in a centralized way, enabling to go back in time like an undo-redo system
//!

use std::fmt::Debug;

use crate::Frame;

pub trait Action: std::fmt::Debug + CloneAction {
    fn act(&self, frame: &mut Frame) -> Artifact;
}

trait CloneAction {
    fn clone_action<'a>(&self) -> Box<dyn Action>;
}

impl<T> CloneAction for T
where
    T: Action + Clone + 'static,
{
    fn clone_action(&self) -> Box<dyn Action> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Action> {
    fn clone(&self) -> Self {
        self.clone_action()
    }
}

#[derive(Clone)]
struct ReciprocalAction {
    redo: Option<Box<dyn Action>>,
    undo: Option<Box<dyn Action>>,
}

impl ReciprocalAction {
    fn invert_actions(self) -> Self {
        Self {
            redo: self.undo,
            undo: self.redo,
        }
    }
}

impl Debug for ReciprocalAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReciprocalAction (\n    undo: {:?},\n    redo: {:?}\n)", self.undo, self.redo)
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

    fn new(redo: Option<Box<dyn Action>>, undo: Option<Box<dyn Action>>) -> Self {
        Self {
            actions: vec![ReciprocalAction {
                redo,
                undo
            }]
        }
    }

    pub fn from_redo(redo: Box<dyn Action>) -> Self {
        Self::new(Some(redo), None)
    }

    pub fn from_redo_undo(redo: Box<dyn Action>, undo: Box<dyn Action>) -> Self {
        Self::new(Some(redo), Some(undo))
    }

    pub fn push(&mut self, other: Self) {
        self.actions.extend(other.actions);
    }

    fn add_action(&mut self, action: ReciprocalAction) {
        self.actions.push(action);
    }

    fn redo(&self, frame: &mut Frame) {
        for action in self.actions.iter() {
            if let Some(redo) = &action.redo {
                let _ = frame.act_by_ref(&**redo);
            }
        }
    }

    fn undo(&self, frame: &mut Frame) {
        for action in self.actions.iter().rev() {
            if let Some(undo) = &action.undo {
                let _ = frame.act_by_ref(&**undo);
            }
        }
    }

    fn invert_actions(&self) -> Self {
        Self {
            actions: self.to_owned().actions.into_iter().map(|action| action.invert_actions()).collect()
        }
    }
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
    pub fn new() -> Self {
        Self {
            artifacts: Vec::new(),
            cursor: 0,
        }
    }

    pub fn append(&mut self, artifact: Artifact) {
        // Don't append empty artifacts
        if artifact.actions.is_empty() { return; }

        self.artifacts.push(artifact);
        self.cursor = self.artifacts.len() - 1;
    }

    pub fn merge_with_last(&mut self, artifact: Artifact) {
        // Don't merge empty artifacts
        if artifact.actions.is_empty() { return; }

        // If no artifact is already present in the list, it will be pushed
        if let Some(last_artifact) = self.artifacts.last_mut() {
            last_artifact.push(artifact);
        } else {
            self.artifacts.push(artifact);
        }
    }

    pub fn undo(&mut self, frame: &mut Frame) {
        if let Some(artifact) = self.artifacts.get(self.cursor) {
            artifact.undo(frame);

            // Append the action of undoing
            self.artifacts.push(artifact.invert_actions());

            self.cursor = self.cursor.saturating_sub(1);
        }
    }

    pub fn redo(&mut self, frame: &mut Frame) {
        // skip empty artifacts
        if let Some(artifact) = self.artifacts.get(self.cursor) {
            artifact.redo(frame);
            self.cursor = self.cursor.saturating_add(1);
        }
    }
}

impl Debug for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "History (\n    cursor: {},\n    artifacts: {:#?})", self.cursor, self.artifacts)
    }
}
