use std::str::FromStr;

use act::{Revert, State, Timeline, TimelineRef};

use crate::{
    Address, Cell, Direction, Frame, FrameError, GridAction, HeadAction, Literal, NotAnAddress,
    Operand, ParseLiteralAsBoolError, ParseLiteralAsNumberError, PointerLoopError,
    ResolveToAddressError, StackAction, StackError,
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

fn pop_operand(frame: &mut TimelineRef<Frame>) -> Result<Operand, EvaluationError> {
    if let Some(popped) = frame.stack.last().cloned() {
        frame.act(StackAction::Pop).map_err(|err| match err {
            FrameError::Stack(stack_error) => EvaluationError::StackError(stack_error),
            _ => unreachable!("StackAction should only return a StackError"),
        });
        Ok(popped.clone())
    } else {
        unreachable!("stack.pop() must only return None when StackAction::Pop return an Err");
    }
}

fn pop_address(frame: &mut TimelineRef<Frame>) -> Result<Address, EvaluationError> {
    pop_operand(frame).and_then(|operand| {
        Ok(operand
            .resolve_to_address(&frame.grid)
            .map_err(|err| match err {
                ResolveToAddressError::PointerLoop(err) => EvaluationError::PointerLoop(err),
                ResolveToAddressError::NotAnAddress(err) => EvaluationError::NotAnAddress(err),
            })?)
    })
}

fn pop_literal(frame: &mut TimelineRef<Frame>) -> Result<Literal, EvaluationError> {
    pop_operand(frame).and_then(|operand| Ok(operand.resolve_to_literal(&frame.grid)?))
}

fn pop_as_number(frame: &mut TimelineRef<Frame>) -> Result<u32, EvaluationError> {
    pop_literal(frame).and_then(|literal| Ok(literal.try_as_number()?))
}

fn pop_as_bool(frame: &mut TimelineRef<Frame>) -> Result<bool, EvaluationError> {
    pop_literal(frame).and_then(|literal| Ok(literal.try_as_bool()?))
}

impl Opcode {
    pub fn from_cell(cell: Cell) -> Result<Opcode, EvaluationError> {
        Opcode::from_str(&cell.as_str()).map_err(|_| EvaluationError::NotAnOpcode(cell.to_string()))
    }

    pub fn evaluate(self, frame: &mut Frame) -> Result<Revert, <Frame as State>::Error> {
        use Opcode::*;
        let mut frame = TimelineRef::new(frame);

        dbg!(&self);

        match self {
            Nop => {}

            Gup | Gri | Gdo | Gle => {
                let direction = match self {
                    Gup => Direction::Up,
                    Gri => Direction::Left,
                    Gdo => Direction::Down,
                    Gle => Direction::Left,
                    _ => unreachable!(),
                };

                frame.act(HeadAction::DirectTo(direction))?;
            }

            Set => {
                let at = pop_address(&mut frame)?;
                let lit = pop_literal(&mut frame)?;

                frame.act(GridAction::Set(*at.position(), lit.as_cell().clone()))?;
            }

            Add | Sub | Mul | Div => {
                let rhs = pop_as_number(&mut frame)?;
                let lhs = pop_as_number(&mut frame)?;

                let result = match self {
                    Add => lhs.saturating_add(rhs),
                    Sub => lhs.saturating_sub(rhs),
                    Mul => lhs.saturating_mul(rhs),
                    Div => lhs.checked_div(rhs).unwrap_or(0),
                    _ => unreachable!(),
                };

                frame.act(StackAction::Push(Literal::from_number_trim(result).into()))?;
            }

            Equ | Neq => {
                let rhs = pop_literal(&mut frame)?;
                let lhs = pop_literal(&mut frame)?;

                let result = match self {
                    Equ => lhs.eq(&rhs),
                    Neq => lhs.ne(&rhs),
                    _ => unreachable!(),
                };

                frame.act(StackAction::Push(Literal::from_bool(result).into()))?;
            }

            Grt | Lst | Grq | Lsq => {
                let rhs = pop_as_number(&mut frame)?;
                let lhs = pop_as_number(&mut frame)?;

                let result = match self {
                    Grt => lhs.gt(&rhs),
                    Lst => lhs.lt(&rhs),
                    Grq => lhs.ge(&rhs),
                    Lsq => lhs.le(&rhs),
                    _ => unreachable!(),
                };

                frame.act(StackAction::Push(Literal::from_bool(result).into()))?;
            }

            Jmp => {
                let address = pop_address(&mut frame)?;
                frame.act(HeadAction::MoveTo(*address.position()))?;
            }
        };

        if !matches!(self, Jmp) {
            frame.act(HeadAction::Step)?;
        }

        Ok(frame.to_revert())
    }
}
