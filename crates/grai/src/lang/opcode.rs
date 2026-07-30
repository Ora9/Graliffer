use std::str::FromStr;

use act::{Revert, State};

use crate::{
    Address, Cell, Direction, Frame, GridAction, HeadAction, Literal, NotAnAddress, Operand,
    ParseLiteralAsBoolError, ParseLiteralAsNumberError, PointerLoopError, ResolveToAddressError,
    StackAction, StackError,
};

#[derive(Debug, strum_macros::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Opcode {
    // Program
    Nop,

    // Grid manipulation
    Set,

    // Basic head movements
    Gup,
    Gri,
    Gdo,
    Gle,

    Jmp,

    // Arithmetic operations
    Add,
    Sub,
    Mul,
    Div,

    // Comparaison operations
    Equ,
    Neq,

    Grt,
    Lst,
    Grq,
    Lsq,
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

    #[error(transparent)]
    ParseAsNumber(#[from] ParseLiteralAsNumberError),

    #[error(transparent)]
    ParseAsBool(#[from] ParseLiteralAsBoolError),
}

fn pop_operand(frame: &mut Frame) -> Result<(Operand, Revert), EvaluationError> {
    if let Some(popped) = frame.stack.last() {
        Ok((popped.clone(), frame.stack.act(StackAction::Pop)?))
    } else {
        unreachable!("stack.pop() must only return None when StackAction::Pop return an Err");
    }
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

fn pop_literal(frame: &mut Frame) -> Result<(Literal, Revert), EvaluationError> {
    pop_operand(frame)
        .and_then(|(operand, revert)| Ok((operand.resolve_to_literal(&frame.grid)?, revert)))
}

fn pop_as_number(frame: &mut Frame) -> Result<(u32, Revert), EvaluationError> {
    pop_literal(frame).and_then(|(literal, revert)| Ok((literal.try_as_number()?, revert)))
}

fn pop_as_bool(frame: &mut Frame) -> Result<(bool, Revert), EvaluationError> {
    pop_literal(frame).and_then(|(literal, revert)| Ok((literal.try_as_bool()?, revert)))
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

            Gup | Gri | Gdo | Gle => {
                let direction = match self {
                    Gup => Direction::Up,
                    Gri => Direction::Left,
                    Gdo => Direction::Down,
                    Gle => Direction::Left,
                    _ => unreachable!(),
                };

                frame.act(HeadAction::DirectTo(direction))
            }

            Set => {
                let (at, at_pop) = pop_address(frame)?;
                let (lit, lit_pop) = pop_literal(frame)?;

                let set = frame.act(GridAction::Set(*at.position(), lit.as_cell().clone()))?;

                Ok(vec![at_pop, lit_pop, set].into())
            }

            Add | Sub | Mul | Div => {
                let (rhs, rhs_pop_revert) = pop_as_number(frame)?;
                let (lhs, lhs_pop_revert) = pop_as_number(frame)?;

                let result = match self {
                    Add => lhs.saturating_add(rhs),
                    Sub => lhs.saturating_sub(rhs),
                    Mul => lhs.saturating_mul(rhs),
                    Div => lhs.checked_div(rhs).unwrap_or(0),
                    _ => unreachable!(),
                };

                let push_revert =
                    frame.act(StackAction::Push(Literal::from_number_trim(result).into()))?;

                Ok(vec![rhs_pop_revert, lhs_pop_revert, push_revert].into())
            }

            Equ | Neq => {
                let (rhs, rhs_pop_revert) = pop_literal(frame)?;
                let (lhs, lhs_pop_revert) = pop_literal(frame)?;

                let result = match self {
                    Equ => lhs.eq(&rhs),
                    Neq => lhs.ne(&rhs),
                    _ => unreachable!(),
                };

                let push_revert =
                    frame.act(StackAction::Push(Literal::from_bool(result).into()))?;

                Ok(vec![rhs_pop_revert, lhs_pop_revert, push_revert].into())
            }

            Grt | Lst | Grq | Lsq => {
                let (rhs, rhs_pop_revert) = pop_as_number(frame)?;
                let (lhs, lhs_pop_revert) = pop_as_number(frame)?;

                let result = match self {
                    Grt => lhs.gt(&rhs),
                    Lst => lhs.lt(&rhs),
                    Grq => lhs.ge(&rhs),
                    Lsq => lhs.le(&rhs),
                    _ => unreachable!(),
                };

                let push_revert =
                    frame.act(StackAction::Push(Literal::from_bool(result).into()))?;

                Ok(vec![rhs_pop_revert, lhs_pop_revert, push_revert].into())
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
