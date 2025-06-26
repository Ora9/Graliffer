//! Artifact is the actions system of Graliffer, it is used to manipulate data in a centralized way, enabling to go back in time like an undo-redo system
//!

use crate::Frame;

pub trait Action: std::fmt::Debug {
    fn act(&self, frame: &mut Frame) -> Artifact;
}

#[derive(Debug)]
struct ReciprocalAction {
    redo: Box<dyn Action>,
    undo: Option<Box<dyn Action>>,
}

#[derive(Debug)]
pub struct Artifact {
    actions: Vec<ReciprocalAction>,
}

impl Artifact {
    pub const EMPTY: Self = Self {
        actions: Vec::new(),
    };


    fn new(redo: Box<dyn Action>, undo: Option<Box<dyn Action>>) -> Self {
        Self {
            actions: vec![ReciprocalAction {
                redo,
                undo
            }]
        }
    }

    pub fn from_redo(redo: Box<dyn Action>) -> Self {
        Self::new(redo, None)
    }

    pub fn from_redo_undo(redo: Box<dyn Action>, undo: Box<dyn Action>) -> Self {
        Self::new(redo, Some(undo))
    }

    pub fn push(&mut self, other: Self) {
        self.actions.extend(other.actions);
    }

    fn add_action(&mut self, action: ReciprocalAction) {
        self.actions.push(action);
    }

    fn redo(&self, frame: &mut Frame) {
        for action in self.actions.iter() {
            let _ = frame.act_by_ref(&*action.redo);
        }
    }

    fn undo(&self, frame: &mut Frame) {
        for action in self.actions.iter().rev() {
            if let Some(undo) = &action.undo {
                let _ = frame.act_by_ref(&**undo);
            }
        }
    }
}

#[derive(Debug, Default)]
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
        if !artifact.actions.is_empty() {
            self.artifacts.push(artifact);
            self.cursor = self.artifacts.len() - 1;
        }
    }

    pub fn merge_with_last(&mut self, artifact: Artifact) {
        let last_index = self.artifacts.len();

        if !artifact.actions.is_empty() {
            if let Some(last_artifact) = self.artifacts.get_mut(last_index) {
                last_artifact.push(artifact);
            }
        }
    }

    pub fn undo(&mut self, frame: &mut Frame) {
        // skip empty artifacts
        if let Some(artifact) = self.artifacts.get(self.cursor) {
            dbg!(&self);
            artifact.undo(frame);
            self.cursor = self.cursor.saturating_sub(1);
        }
    }

    pub fn redo(&mut self, frame: &mut Frame) {
        // skip empty artifacts
        if let Some(artifact) = self.artifacts.get(self.cursor) {
            dbg!(&self);
            artifact.redo(frame);
            self.cursor = self.cursor.saturating_add(1);
        }
    }
}
