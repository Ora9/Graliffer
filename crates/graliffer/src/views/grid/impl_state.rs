use std::convert::Infallible;

use serde::{Deserialize, Serialize};
use tui_input::InputRequest;

use act::{Action, State, TimelinedState};
use grai::Direction;

use crate::{CursorMovement, GridView};

#[derive(Debug, Clone, strum::EnumString, Serialize, Deserialize)]
pub enum GridAction {
    #[strum(disabled)]
    #[serde(skip)]
    GraiGridAction(grai::GridAction),

    Undo,
    Redo,

    Insert(String),

    InsertOverflow(String),

    DeletePrevChar,
    DeleteNextChar,

    DeletePrevCharOrStepLeftGrid,

    DeleteTillStart,
    DeleteTillStartOrStepLeftGrid,

    CursorStepUpGrid,
    CursorStepDownGrid,
    CursorStepRightGrid,
    CursorStepLeftGrid,

    CursorStepRightCharThenGrid,
    CursorStepLeftCharThenGrid,

    CursorDashUpCharThenGrid,
    CursorDashDownCharThenGrid,
    CursorDashRightCharThenGrid,
    CursorDashLeftCharThenGrid,
}

impl Action for GridAction {}

impl From<grai::GridAction> for GridAction {
    fn from(value: grai::GridAction) -> Self {
        Self::GraiGridAction(value)
    }
}

impl State for GridView {
    type Action = GridAction;
    type Error = Infallible;

    fn act(&mut self, action: impl Into<Self::Action>) -> Result<(), Self::Error> {
        let action = action.into();
        use GridAction::*;

        match action {
            GraiGridAction(grai_grid_action) => {
                let _ = self
                    .frame_timeline
                    .state_mut()
                    .write(|frame| frame.grid.act(grai_grid_action));
            }

            Undo => self.undo(),
            Redo => self.redo(),

            Insert(input) => {
                for c in input.chars() {
                    self.handle_insert(c);
                }
            }

            InsertOverflow(input) => {
                for c in input.chars() {
                    if self.grid_input.char_at_max() || c == ' ' {
                        self.cursor_movement(CursorMovement::StepGrid(Direction::Right));
                    }

                    self.handle_insert(c);
                }
            }

            DeletePrevCharOrStepLeftGrid => {
                if self.grid_input.char_cursor() != 0 {
                    self.handle_input_request(InputRequest::DeletePrevChar);
                } else {
                    self.cursor_movement(CursorMovement::StepGrid(Direction::Left));
                }
            }

            // todo: use the newer DeleteFromStart
            DeleteTillStart => {
                self.handle_input_request(InputRequest::DeletePrevWord);
            }

            DeleteTillStartOrStepLeftGrid => {
                if self.grid_input.char_cursor() != 0 {
                    self.handle_input_request(InputRequest::DeletePrevWord);
                } else {
                    self.cursor_movement(CursorMovement::StepGrid(Direction::Left));
                }
            }

            DeletePrevChar => self.handle_input_request(InputRequest::DeletePrevChar),

            DeleteNextChar => self.handle_input_request(InputRequest::DeleteNextChar),

            CursorStepUpGrid | CursorStepDownGrid | CursorStepLeftGrid | CursorStepRightGrid => {
                let direction = match action {
                    CursorStepUpGrid => Direction::Up,
                    CursorStepDownGrid => Direction::Down,
                    CursorStepRightGrid => Direction::Right,
                    CursorStepLeftGrid => Direction::Left,
                    _ => unreachable!(),
                };

                self.cursor_movement(CursorMovement::StepGrid(direction));
            }

            CursorStepLeftCharThenGrid | CursorStepRightCharThenGrid => {
                let direction = match action {
                    CursorStepRightCharThenGrid => Direction::Right,
                    CursorStepLeftCharThenGrid => Direction::Left,
                    _ => unreachable!(),
                };

                self.cursor_movement(CursorMovement::StepCharThenGrid(direction));
            }

            CursorDashUpCharThenGrid
            | CursorDashRightCharThenGrid
            | CursorDashDownCharThenGrid
            | CursorDashLeftCharThenGrid => {
                let direction = match action {
                    CursorDashUpCharThenGrid => Direction::Up,
                    CursorDashDownCharThenGrid => Direction::Down,
                    CursorDashRightCharThenGrid => Direction::Right,
                    CursorDashLeftCharThenGrid => Direction::Left,
                    _ => unreachable!(),
                };

                self.cursor_movement(CursorMovement::DashUntilBoundsOrNonEmpty(direction));
            }
        };

        Ok(())
    }
}
