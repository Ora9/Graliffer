use egui::KeyboardShortcut;

use crate::{editor::{EventContext, InputEvent}, Editor, Frame};
use std::fmt::Debug;

pub trait CloneEditorAction {
    fn clone_action(&self) -> Box<dyn EditorAction>;
}

impl<T> CloneEditorAction for T
where
    T: EditorAction + Clone + 'static,
{
    fn clone_action(&self) -> Box<dyn EditorAction> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn EditorAction> {
    fn clone(&self) -> Self {
        self.clone_action()
    }
}

pub trait EditorAction: std::fmt::Debug + CloneEditorAction {
    fn act(&self, editor: &mut Editor);
    fn events_and_context(&self) -> Option<(InputEvent, EventContext)> {
        None
    }
    fn text(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
}

// impl<T> CloneAction for T
// where
//     T: EditorAction + Clone + 'static,
// {
//     fn clone_action(&self) -> Box<dyn EditorAction> {
//         Box::new(self.clone())
//     }
// }

// impl Clone for Box<dyn FrameAction> {
//     fn clone(&self) -> Self {
//         self.clone_action()
//     }
// }


pub trait FrameAction: std::fmt::Debug + CloneFrameAction {
    fn act(&self, frame: &mut Frame) -> Artifact;
}

pub trait CloneFrameAction {
    fn clone_action(&self) -> Box<dyn FrameAction>;
}

impl<T> CloneFrameAction for T
where
    T: FrameAction + Clone + 'static,
{
    fn clone_action(&self) -> Box<dyn FrameAction> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn FrameAction> {
    fn clone(&self) -> Self {
        self.clone_action()
    }
}

#[derive(Clone)]
struct ReciprocalAction {
    redo: Option<Box<dyn FrameAction>>,
    undo: Option<Box<dyn FrameAction>>,
}

impl ReciprocalAction {
    // fn invert_actions(self) -> Self {
    //     Self {
    //         redo: self.undo,
    //         undo: self.redo,
    //     }
    // }
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

    fn new(redo: Option<Box<dyn FrameAction>>, undo: Option<Box<dyn FrameAction>>) -> Self {
        Self {
            actions: vec![ReciprocalAction { redo, undo }],
        }
    }

    pub fn from_redo(redo: Box<dyn FrameAction>) -> Self {
        Self::new(Some(redo), None)
    }

    pub fn from_redo_undo(redo: Box<dyn FrameAction>, undo: Box<dyn FrameAction>) -> Self {
        Self::new(Some(redo), Some(undo))
    }

    pub fn push(&mut self, other: Self) {
        self.actions.extend(other.actions);
    }

    // fn add_action(&mut self, action: ReciprocalAction) {
    //     self.actions.push(action);
    // }

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

    pub fn undo(&mut self, frame: &mut Frame) {
        let Some(last_artifact) = self.cursor.checked_sub(1) else {
            return;
        };

        if let Some(artifact) = self.artifacts.get(last_artifact) {
            artifact.undo(frame);

            // Append the action of undoing
            // self.artifacts.push(artifact.invert_actions());
            self.cursor = last_artifact;
        }
    }

    pub fn redo(&mut self, frame: &mut Frame) {
        if let Some(artifact) = self.artifacts.get(self.cursor) {
            artifact.redo(frame);

            // Append the action of redoing
            // self.artifacts.push(artifact.to_owned());
            self.cursor = self.cursor.saturating_add(1);
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
