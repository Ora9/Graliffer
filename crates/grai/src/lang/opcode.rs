use std::str::FromStr;

use action::{AnyAction, Revert, State};

use crate::{Cell, Direction, Frame, FrameError, HeadAction};

#[derive(Debug, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Opcode {
    Nop,

    Gup,
    Gri,
    Gdo,
    Gle,
}

#[derive(Debug, thiserror::Error)]
pub enum OpcodeError {
    #[error("not an opcode, found {0}")]
    NotAnOpcode(String),
}

impl Opcode {
    pub fn from_cell(cell: Cell) -> Result<Opcode, OpcodeError> {
        Opcode::from_str(&cell.content()).map_err(|_| OpcodeError::NotAnOpcode(cell.content()))
    }

    pub fn evaluate(self, frame: &mut Frame) -> Result<Revert, FrameError> {
        use Opcode::*;

        match self {
            Nop => Ok(Revert::None),

            Gup => frame.act(AnyAction::new(HeadAction::DirectTo(Direction::Up))),
            Gri => frame.act(AnyAction::new(HeadAction::DirectTo(Direction::Right))),
            Gdo => frame.act(AnyAction::new(HeadAction::DirectTo(Direction::Down))),
            Gle => frame.act(AnyAction::new(HeadAction::DirectTo(Direction::Left))),
        }
    }
}
