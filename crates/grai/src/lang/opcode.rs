use std::str::FromStr;

use act::{Revert, State};

use crate::{Cell, Direction, Frame, HeadAction, Operand, StackAction, StackError};

#[derive(Debug, strum_macros::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Opcode {
    Nop,

    Gup,
    Gri,
    Gdo,
    Gle,

    Jmp,
}

#[derive(Debug, thiserror::Error)]
pub enum OpcodeError {
    #[error("not an opcode, found {0}")]
    NotAnOpcode(String),
}

fn pop(frame: &mut Frame) -> Result<(Operand, Revert), StackError> {
    if let Some(popped) = frame.stack.last() {
        Ok((popped.clone(), frame.stack.act(StackAction::Pop)?))
    } else {
        unreachable!("stack.pop() must only return None when StackAction::Pop return an Err");
    }
}

impl Opcode {
    pub fn from_cell(cell: Cell) -> Result<Opcode, OpcodeError> {
        Opcode::from_str(&cell.as_str()).map_err(|_| OpcodeError::NotAnOpcode(cell.to_string()))
    }

    pub fn evaluate(self, frame: &mut Frame) -> Result<Revert, <Frame as State>::Error> {
        use Opcode::*;

        match self {
            Nop => Ok(Revert::None),

            Gup => frame.act(HeadAction::DirectTo(Direction::Up)),
            Gri => frame.act(HeadAction::DirectTo(Direction::Right)),
            Gdo => frame.act(HeadAction::DirectTo(Direction::Down)),
            Gle => frame.act(HeadAction::DirectTo(Direction::Left)),

            Jmp => {
                let (address, pop_revert) = pop(frame)?;

                dbg!(address);

                // frame.act(HeadAction::MoveTo(address))

                Ok(Revert::None)
            }
        }
    }
}
