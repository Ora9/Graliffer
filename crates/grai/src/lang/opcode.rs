use std::str::FromStr;

use act::{Revert, State};

use crate::{Address, Cell, Direction, Frame, HeadAction, Operand, Stack, StackAction, StackError};

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

    #[error("not an address, found {0}")]
    NotAnAddress(Operand),

    #[error("stack error: {0}")]
    StackError(#[from] StackError),
}

fn pop(frame: &mut Frame) -> Result<(Operand, Revert), StackError> {
    if let Some(popped) = frame.stack.last() {
        Ok((popped.clone(), frame.stack.act(StackAction::Pop)?))
    } else {
        unreachable!("stack.pop() must only return None when StackAction::Pop return an Err");
    }
}

fn pop_address(frame: &mut Frame) -> Result<(Address, Revert), OpcodeError> {
    pop(frame)
        .map_err(|err| OpcodeError::StackError(err))
        .and_then(|(operand, revert)| {
            let operand = operand
                .as_address()
                .ok_or(OpcodeError::NotAnAddress(operand.clone()))?;

            Ok((*operand, revert))
        })
}

impl Opcode {
    pub fn from_cell(cell: Cell) -> Result<Opcode, OpcodeError> {
        Opcode::from_str(&cell.as_str()).map_err(|_| OpcodeError::NotAnOpcode(cell.to_string()))
    }

    pub fn evaluate(self, frame: &mut Frame) -> Result<Revert, <Frame as State>::Error> {
        use Opcode::*;

        dbg!(&self);

        let mut revert = match self {
            Nop => Ok(Revert::None),

            Gup => frame.act(HeadAction::DirectTo(Direction::Up)),
            Gri => frame.act(HeadAction::DirectTo(Direction::Right)),
            Gdo => frame.act(HeadAction::DirectTo(Direction::Down)),
            Gle => frame.act(HeadAction::DirectTo(Direction::Left)),

            Jmp => {
                let (address, pop_revert) = pop_address(frame)?;

                let jmp_revert = frame.act(HeadAction::MoveTo(*address.position()))?;

                Ok(vec![pop_revert, jmp_revert].into())
            }
        }?;

        if !matches!(self, Jmp) {
            revert.extend(frame.act(HeadAction::Step)?);
        }

        Ok(revert)
    }
}
