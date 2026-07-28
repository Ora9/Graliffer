use std::str::FromStr;

use act::{Revert, State};

use crate::{
    Address, Cell, Direction, Frame, HeadAction, Literal, Operand, OperandError, Stack,
    StackAction, StackError,
};

#[derive(Debug, strum_macros::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Opcode {
    Nop,

    Gup,
    Gri,
    Gdo,
    Gle,

    Add,
    Sub,
    Mul,
    Div,

    Jmp,
}

#[derive(Debug, thiserror::Error)]
pub enum OpcodeError {
    #[error("operand error: {0}")]
    Operand(OperandError),

    #[error("not an opcode, found {0}")]
    NotAnOpcode(String),

    #[error("not an address, found {0}")]
    NotAnAddress(String),

    #[error("stack error: {0}")]
    StackError(#[from] StackError),
}

fn pop_operand(frame: &mut Frame) -> Result<(Operand, Revert), OpcodeError> {
    if let Some(popped) = frame.stack.last() {
        Ok((popped.clone(), frame.stack.act(StackAction::Pop)?))
    } else {
        unreachable!("stack.pop() must only return None when StackAction::Pop return an Err");
    }
}

fn pop_literal(frame: &mut Frame) -> Result<(Literal, Revert), OpcodeError> {
    pop_operand(frame).and_then(|(operand, revert)| {
        Ok((
            operand
                .resolve_to_literal(&frame.grid)
                .map_err(|err| OpcodeError::Operand(err))?,
            revert,
        ))
    })
}

fn pop_address(frame: &mut Frame) -> Result<(Address, Revert), OpcodeError> {
    pop_operand(frame).and_then(|(operand, revert)| {
        Ok((
            operand
                .resolve_to_address(&frame.grid)
                .map_err(|err| match err {
                    OperandError::CouldNotResolveAsAddress {
                        operand_kind: _,
                        got,
                    } => OpcodeError::NotAnAddress(got),
                    _ => OpcodeError::Operand(err),
                })?,
            revert,
        ))
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

            Add | Sub | Mul | Div => {
                // let (lhs, lhs_pop_revert) = pop(frame)

                Ok(Revert::None)
            }

            Jmp => {
                let (address, mut revert) = pop_address(frame)?;
                revert.extend(frame.act(HeadAction::MoveTo(*address.position()))?);

                Ok(revert)
            }
        }?;

        if !matches!(self, Jmp) {
            revert.extend(frame.act(HeadAction::Step)?);
        }

        Ok(revert)
    }
}
