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

    pub fn from_action(redo: Box<dyn Action>) -> Self {
        Self {
            actions: vec!(ReciprocalAction {
                redo: redo,
                undo: None,
            }),
        }
    }

    pub fn from_reciprocal(redo: Box<dyn Action>, undo: Box<dyn Action>) -> Self {
        Self {
            actions: vec!(ReciprocalAction {
                redo,
                undo: Some(undo),
            }),
        }
    }

    pub fn append_last(&mut self, other: Self) {
        self.actions.extend(other.actions);
    }

    pub fn add_action(&mut self, action: ReciprocalAction) {
        self.actions.push(action);
    }

    fn redo(self, frame: &mut Frame) {
        for action in self.actions {
            frame.act_by_ref(&*action.redo);
        }
    }

    fn undo(self, frame: &mut Frame) {
        for action in self.actions {
            if let Some(undo) = action.undo {
                frame.act_by_ref(&*undo);
            }
        }
    }
}
