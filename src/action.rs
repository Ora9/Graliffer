use crate::{
    editor::{EventContext, InputEvent}, grid::{Cell, Position}, utils::Direction, Editor, Frame, Operand
};
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

#[derive(Debug, Clone)]
pub enum FrameAction {
    GridSet(Position, Cell),

    StackPush(Operand),
    StackPop,

    HeadMoveTo(Position),
    HeadDirectTo(Direction),
    HeadStep,

    ConsolePrint(String),
}

impl FrameAction {
    pub fn act(&self, frame: &mut Frame) -> Artifact {
        use FrameAction::*;
        match self {
            GridSet(position, cell) => {
                let previous_cell = frame.grid.get(*position);

                frame.grid.set(*position, cell.clone());

                Artifact::from_redo_undo(
                    self.to_owned(),
                    Self::GridSet(*position, previous_cell)
                )
            }

            StackPush(operand) => {
                frame.stack.push(operand.to_owned());

                Artifact::from_redo_undo(
                    self.to_owned(),
                    StackPop
                )
            }
            StackPop => {
                if let Some(popped) = frame.stack.pop() {
                    Artifact::from_redo_undo(
                        self.to_owned(),
                        StackPush(popped)
                    )
                } else {
                    Artifact::from_redo(self.to_owned())
                }
            }

            HeadMoveTo(position) => {
                let old_position = frame.head.position;

                frame.head.move_to(*position);

                Artifact::from_redo_undo(
                    self.to_owned(),
                    Self::HeadMoveTo(old_position)
                )
            }
            HeadDirectTo(direction) => {
                let old_direction = frame.head.direction;

                frame.head.direct_to(*direction);

                Artifact::from_redo_undo(
                    self.to_owned(),
                    Self::HeadDirectTo(old_direction)
                )
            }
            HeadStep => {
                let old_position = frame.head.position;

                let _ = frame.head.step();

                Artifact::from_redo_undo(
                    self.to_owned(),
                    Self::HeadMoveTo(old_position)
                )
            }

            ConsolePrint(string) => {
                frame.console.print(string);

                Artifact::from_redo(self.to_owned())
            }
        }
    }
}

#[derive(Clone)]
struct ReciprocalAction {
    redo: Option<FrameAction>,
    undo: Option<FrameAction>,
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
