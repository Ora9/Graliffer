use std::str::FromStr;

use act::{Revert, State, TimelineRef};

use crate::{
    Address, Cell, Direction, Errored, ErroredEncountered, Frame, FrameError, GridAction,
    HeadAction, Literal, LiteralFormatError, NotAnAddress, Operand, ParseLiteralAsBoolError,
    ParseLiteralAsNumberError, PointerLoopError, ResolveToAddressError, ResolveToLiteralError,
    StackAction, StackError,
};

// TODO: Split to have a multiples enums for each types of operands
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

    // Conditional jumps
    Jif,

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
pub enum PopOperandError {
    #[error(transparent)]
    Stack(#[from] StackError),

    #[error(transparent)]
    ErroredEncountered(#[from] ErroredEncountered),

    #[error(transparent)]
    PointerLoop(#[from] PointerLoopError),

    #[error(transparent)]
    NotAnAddress(#[from] NotAnAddress),

    #[error("could not parse number : {0}")]
    LiteralFromNumber(#[from] LiteralFormatError),

    #[error(transparent)]
    ParseAsNumber(#[from] ParseLiteralAsNumberError),

    #[error(transparent)]
    ParseAsBool(#[from] ParseLiteralAsBoolError),
}

// #[derive(Debug, thiserror::Error, PartialEq, Eq)]
// pub enum OperandError {}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum EvaluationError {
    #[error(transparent)]
    PopOperand(#[from] PopOperandError),

    #[error(transparent)]
    Stack(#[from] StackError),
}

impl From<FrameError> for EvaluationError {
    fn from(value: FrameError) -> Self {
        match value {
            FrameError::Evaluation(err) => err,
            FrameError::Stack(err) => Self::Stack(err),
            // FrameError::FetchOperand(err) => Self::FetchOperand(err),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("not an opcode, got {got}")]
pub struct NotAnOpcode {
    got: String,
}

fn pop_operand(frame: &mut TimelineRef<Frame>) -> Result<Operand, PopOperandError> {
    let popped = frame.stack.last().cloned();

    frame.act(StackAction::Pop).map_err(|err| match err {
        FrameError::Stack(stack_error) => PopOperandError::Stack(stack_error),
        _ => unreachable!("StackAction should only return a StackError"),
    })?;

    Ok(popped.expect("stack.pop() must only return None when StackAction::Pop returned an Err"))
}

fn pop_to_address(frame: &mut TimelineRef<Frame>) -> Result<Address, PopOperandError> {
    pop_operand(frame).and_then(|operand| {
        Ok(operand
            .resolve_to_address(&frame.grid)
            .map_err(|err| match err {
                ResolveToAddressError::PointerLoop(err) => PopOperandError::PointerLoop(err),
                ResolveToAddressError::NotAnAddress(err) => PopOperandError::NotAnAddress(err),
                ResolveToAddressError::ErroredEncountered(_) => {
                    PopOperandError::NotAnAddress(NotAnAddress {
                        got: Operand::Errored(Errored::new()),
                    })
                }
            })?)
    })
}

fn pop_to_literal(frame: &mut TimelineRef<Frame>) -> Result<Option<Literal>, PopOperandError> {
    pop_operand(frame).and_then(|operand| match operand.resolve_to_literal(&frame.grid) {
        Ok(literal) => Ok(Some(literal)),
        Err(ResolveToLiteralError::ErroredEncountered(_)) => Ok(None),
        Err(ResolveToLiteralError::PointerLoop(err)) => Err(PopOperandError::PointerLoop(err)),
    })
}

fn pop_as_cell(frame: &mut TimelineRef<Frame>) -> Result<Cell, PopOperandError> {
    pop_operand(frame).map(|operand| operand.to_cell())
}

fn pop_as_number(frame: &mut TimelineRef<Frame>) -> Result<Option<u32>, PopOperandError> {
    match pop_to_literal(frame)? {
        Some(literal) => Ok(Some(literal.try_as_number()?)),
        None => Ok(None),
    }
}

fn pop_as_bool(frame: &mut TimelineRef<Frame>) -> Result<Option<bool>, PopOperandError> {
    match pop_to_literal(frame)? {
        Some(literal) => Ok(Some(literal.try_as_bool()?)),
        None => Ok(None),
    }
}

impl Opcode {
    pub fn from_cell(cell: Cell) -> Result<Opcode, NotAnOpcode> {
        Opcode::from_str(&cell.as_str()).map_err(|_| NotAnOpcode {
            got: cell.to_string(),
        })
    }

    pub fn evaluate(self, frame: &mut Frame) -> Result<Revert, EvaluationError> {
        use Opcode::*;
        let mut frame = TimelineRef::new(frame);

        match self {
            Nop => {}

            Gup | Gri | Gdo | Gle => {
                let direction = match self {
                    Gup => Direction::Up,
                    Gri => Direction::Right,
                    Gdo => Direction::Down,
                    Gle => Direction::Left,
                    _ => unreachable!(),
                };

                frame.act(HeadAction::DirectTo(direction))?;
            }

            Set => {
                let at = pop_to_address(&mut frame)?;
                let lit = pop_as_cell(&mut frame)?;

                frame.act(GridAction::Set(*at.position(), lit.clone()))?;
            }

            Add | Sub | Mul | Div => {
                let rhs_opt = pop_as_number(&mut frame)?;
                let lhs_opt = pop_as_number(&mut frame)?;

                let operand = match (rhs_opt, lhs_opt) {
                    (Some(rhs), Some(lhs)) => {
                        let value_opt = match self {
                            Add => lhs.checked_add(rhs),
                            Sub => lhs.checked_sub(rhs),
                            Mul => lhs.checked_mul(rhs),
                            Div => lhs.checked_div(rhs),
                            _ => unreachable!(),
                        };

                        if let Some(value) = value_opt
                            && let Ok(value_lit) = Literal::try_from_number(value)
                        {
                            value_lit.into()
                        } else {
                            Errored::new().into()
                        }
                    }
                    (None, _) | (_, None) => Errored::new().into(),
                };

                frame.act(StackAction::Push(operand))?;
            }

            Equ | Neq => {
                let rhs_opt = pop_to_literal(&mut frame)?;
                let lhs_opt = pop_to_literal(&mut frame)?;

                let operand = match (rhs_opt, lhs_opt) {
                    (Some(rhs), Some(lhs)) => {
                        let value = match self {
                            Equ => lhs.eq(&rhs),
                            Neq => lhs.ne(&rhs),
                            _ => unreachable!(),
                        };

                        Literal::from_bool(value).into()
                    }
                    (None, None) => Literal::from_bool(true).into(),
                    (None, Some(_)) | (Some(_), None) => Errored::new().into(),
                };

                frame.act(StackAction::Push(operand))?;
            }

            Grt | Lst | Grq | Lsq => {
                let rhs_opt = pop_as_number(&mut frame)?;
                let lhs_opt = pop_as_number(&mut frame)?;

                let operand = match (rhs_opt, lhs_opt) {
                    (Some(rhs), Some(lhs)) => {
                        let value = match self {
                            Grt => lhs.gt(&rhs),
                            Lst => lhs.lt(&rhs),
                            Grq => lhs.ge(&rhs),
                            Lsq => lhs.le(&rhs),
                            _ => unreachable!(),
                        };

                        Literal::from_bool(value).into()
                    }
                    (None, _) | (_, None) => Errored::new().into(),
                };

                frame.act(StackAction::Push(operand))?;
            }

            Jmp => {
                let address = pop_to_address(&mut frame)?;
                frame.act(HeadAction::MoveTo(*address.position()))?;
            }

            Jif => {
                let address = pop_to_address(&mut frame)?;
                let condition_opt = pop_as_bool(&mut frame)?;

                if condition_opt.is_some_and(|condition| condition) {
                    frame.act(HeadAction::MoveTo(*address.position()))?;
                }
            }
        };

        if !matches!(self, Jmp | Jif) {
            frame.act(HeadAction::Step)?;
        }

        Ok(frame.into_revert())
    }
}
