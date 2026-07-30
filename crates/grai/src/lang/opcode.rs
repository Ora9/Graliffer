use std::str::FromStr;

use act::{Revert, State};

use crate::{
    Address, Cell, Direction, Frame, GridAction, HeadAction, Literal, NotAnAddress, Operand,
    PointerLoopError, ResolveToAddressError, StackAction, StackError,
};

#[derive(Debug, strum_macros::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Opcode {
    Nop,

    Set,

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

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum EvaluationError {
    #[error(transparent)]
    ResolveToAddress(#[from] ResolveToAddressError),

    #[error(transparent)]
    NotAnAddress(#[from] NotAnAddress),

    #[error(transparent)]
    PointerLoop(#[from] PointerLoopError),

    #[error("not an opcode, found {0}")]
    NotAnOpcode(String),

    #[error("stack error: {0}")]
    StackError(#[from] StackError),
}

fn pop_operand(frame: &mut Frame) -> Result<(Operand, Revert), EvaluationError> {
    if let Some(popped) = frame.stack.last() {
        Ok((popped.clone(), frame.stack.act(StackAction::Pop)?))
    } else {
        unreachable!("stack.pop() must only return None when StackAction::Pop return an Err");
    }
}

fn pop_literal(frame: &mut Frame) -> Result<(Literal, Revert), EvaluationError> {
    pop_operand(frame)
        .and_then(|(operand, revert)| Ok((operand.resolve_to_literal(&frame.grid)?, revert)))
}

fn pop_address(frame: &mut Frame) -> Result<(Address, Revert), EvaluationError> {
    pop_operand(frame).and_then(|(operand, revert)| {
        Ok((
            operand
                .resolve_to_address(&frame.grid)
                .map_err(|err| match err {
                    ResolveToAddressError::PointerLoop(err) => EvaluationError::PointerLoop(err),
                    ResolveToAddressError::NotAnAddress(err) => EvaluationError::NotAnAddress(err),
                })?,
            revert,
        ))
    })
}

impl Opcode {
    pub fn from_cell(cell: Cell) -> Result<Opcode, EvaluationError> {
        Opcode::from_str(&cell.as_str()).map_err(|_| EvaluationError::NotAnOpcode(cell.to_string()))
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

            Set => {
                let (at, at_pop) = pop_address(frame)?;
                let (lit, lit_pop) = pop_literal(frame)?;

                let set = frame.act(GridAction::Set(*at.position(), lit.as_cell().clone()))?;

                Ok(vec![at_pop, lit_pop, set].into())
            }

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
