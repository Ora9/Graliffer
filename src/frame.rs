use serde::{Deserialize, Serialize};

pub mod grid;
pub mod head;
pub mod stack;

use crate::{
    history::Artifact, console::Console, grid::{Cell, Grid, Position}, head::Head, stack::Stack, utils::Direction, Operand, Word
};

/// A [`Frame`] represents a run
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Frame {
    pub head: Head,
    pub grid: Grid,
    pub stack: Stack,

    #[serde(skip)]
    pub console: Console,
}

impl Frame {
    // pub fn new() -> Self {
    //     Self {
    //         head: descriptor.head,
    //         grid: descriptor.grid,
    //         stack: descriptor.stack,

    //         console: descriptor.console,
    //     }
    // }

    /// Make a step, the minimal unit of a Graliffer execution :
    /// - Move head 1 cell in its direction
    /// - Parse the Cell under the head
    /// - If cell content :
    ///     - is empty, continue
    ///     - can correspond to an Opcode, push to stack
    ///     - can be parsed as an Adress Operand, push to stack
    ///     - can be parsed as a Pointer Operand, push to stack
    ///     - is non of the above, push to stack as a Literal Operand
    /// - Does the stack contains a valid operation
    ///     - if yes, evaluate the operation
    ///     - if not, hop
    ///
    pub fn step(&mut self) -> Artifact {
        let current_cell = self.grid.get(self.head.position);

        if current_cell.is_empty() {
            self.act(FrameAction::HeadStep)
        } else {
            let word = Word::from_cell(current_cell);

            match word {
                Word::Opcode(opcode) => {
                    println!("Opcode! : {:?}", opcode);
                    opcode.evaluate(self)
                }
                Word::Operand(operand) => {
                    let mut artifact = self.act(FrameAction::StackPush(operand));
                    artifact.push(self.act(FrameAction::HeadStep));

                    artifact
                }
            }
        }
    }

    #[must_use]
    pub fn act(&mut self, action: FrameAction) -> Artifact {
        action.act(self)
    }

    // #[must_use]
    // pub fn act_by_ref(&mut self, action: FrameAction) -> Artifact {
    //     action.act(self)
    // }
}

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
