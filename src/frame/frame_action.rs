use crate::{grid::{Cell, Position}, utils::Direction, Artifact, Frame, Operand};


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

                Artifact::from_redo_undo(self.to_owned(), Self::GridSet(*position, previous_cell))
            }

            StackPush(operand) => {
                frame.stack.push(operand.to_owned());

                Artifact::from_redo_undo(self.to_owned(), StackPop)
            }
            StackPop => {
                if let Some(popped) = frame.stack.pop() {
                    Artifact::from_redo_undo(self.to_owned(), StackPush(popped))
                } else {
                    Artifact::from_redo(self.to_owned())
                }
            }

            HeadMoveTo(position) => {
                let old_position = frame.head.position;

                frame.head.move_to(*position);

                Artifact::from_redo_undo(self.to_owned(), Self::HeadMoveTo(old_position))
            }
            HeadDirectTo(direction) => {
                let old_direction = frame.head.direction;

                frame.head.direct_to(*direction);

                Artifact::from_redo_undo(self.to_owned(), Self::HeadDirectTo(old_direction))
            }
            HeadStep => {
                let old_position = frame.head.position;

                let _ = frame.head.step();

                Artifact::from_redo_undo(self.to_owned(), Self::HeadMoveTo(old_position))
            }

            ConsolePrint(string) => {
                frame.console.print(string);

                Artifact::from_redo(self.to_owned())
            }
        }
    }
}
