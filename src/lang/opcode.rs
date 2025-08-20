use anyhow::anyhow;
use std::str::FromStr;
use strum_macros::EnumString;

use crate::{
    Address, Literal,
    action::Artifact,
    console::ConsoleAction,
    grid::{Cell, GridAction},
    head::HeadAction,
    stack::StackAction,
    utils::Direction,
};

use super::{Frame, Operand};

fn pop_operand(frame: &mut Frame) -> (Result<Operand, anyhow::Error>, Artifact) {
    if let Some(popped) = frame.stack.get_last() {
        (Ok(popped.to_owned()), frame.act(Box::new(StackAction::Pop)))
    } else {
        (
            Err(anyhow!("Could not pop the stack further")),
            Artifact::EMPTY,
        )
    }
}

fn pop_literal(frame: &mut Frame) -> (Result<Literal, anyhow::Error>, Artifact) {
    let (operand_res, artifact) = pop_operand(frame);

    (
        operand_res.map(|operand| operand.resolve_to_literal(&frame.grid)),
        artifact,
    )
}

fn pop_address(frame: &mut Frame) -> (Result<Address, anyhow::Error>, Artifact) {
    let (operand_res, artifact) = pop_operand(frame);

    (
        match operand_res {
            Ok(operand) => operand.resolve_to_address(&frame.grid),
            Err(err) => Err(err),
        },
        artifact,
    )
}

fn pop_as_numeric_with_default(frame: &mut Frame) -> (u32, Artifact) {
    let (operand_res, artifact) = pop_operand(frame);

    (
        operand_res.map_or(0, |operand| {
            operand
                .resolve_to_literal(&frame.grid)
                .as_numeric_with_default()
        }),
        artifact,
    )
}

fn pop_as_numeric(frame: &mut Frame) -> (Result<u32, anyhow::Error>, Artifact) {
    let (operand_res, artifact) = pop_operand(frame);

    (
        match operand_res {
            Ok(operand) => operand.resolve_to_literal(&frame.grid).as_numeric(),
            Err(err) => Err(err),
        },
        artifact,
    )
}

fn pop_as_bool(frame: &mut Frame) -> (Result<bool, anyhow::Error>, Artifact) {
    let (operand_res, artifact) = pop_operand(frame);

    (
        match operand_res {
            Ok(operand) => operand.resolve_to_literal(&frame.grid).as_bool(),
            Err(err) => Err(err),
        },
        artifact,
    )
}

// TODO : should we really return true as a default value when no operand could be popped ? (stack empty)
fn pop_as_bool_with_default(frame: &mut Frame) -> (bool, Artifact) {
    let (operand_res, artifact) = pop_operand(frame);

    (
        operand_res.map_or(true, |operand| {
            operand
                .resolve_to_literal(&frame.grid)
                .as_bool_with_default()
        }),
        artifact,
    )
}

#[derive(Debug, Clone, Copy, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Opcode {
    // Debug
    Dbg,

    // Program management
    Hlt,
    Nop,

    // Basic head movements
    Gou,
    Gor,
    God,
    Gol,
    Jmp,

    // Conditionnal head movements
    Igu,
    Igr,
    Igd,
    Igl,
    Ijp,

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

    // Grid manipulation
    Set,

    // Console output
    Prt,
}

impl Opcode {
    pub fn from_cell(cell: Cell) -> Result<Opcode, anyhow::Error> {
        Opcode::from_str(&cell.content())
            .map_err(|_| anyhow::anyhow!(format!("not a valid opcode")))
    }

    pub fn is_cell_valid(cell: &Cell) -> bool {
        Self::from_str(&cell.content()).is_ok()
    }

    pub fn evaluate(self, frame: &mut Frame) -> Artifact {
        use Opcode::*;
        let mut artifact = match self {
            Nop => Artifact::EMPTY,
            Hlt => {
                unimplemented!();
            }
            Dbg => {
                println!("---- DEBUG INFO : Frame dump ----");
                println!("{:?}", frame);
                println!("---- DEBUG INFO END ----");
                Artifact::EMPTY
            }

            Gou | Gor | God | Gol => {
                let direction = match self {
                    Gou => Direction::Up,
                    Gor => Direction::Left,
                    God => Direction::Down,
                    Gol => Direction::Left,
                    _ => unreachable!(),
                };

                frame.act(Box::new(HeadAction::DirectTo(direction)))
            }

            Jmp => {
                let address_opt = frame
                    .stack
                    .get_last()
                    .map(|operand| operand.resolve_to_address(&frame.grid));
                let mut artifact = frame.act(Box::new(StackAction::Pop));

                if let Some(Ok(address)) = address_opt {
                    artifact.push(frame.act(Box::new(HeadAction::MoveTo(address.position))));
                }

                artifact
            }

            Igu | Igr | Igd | Igl => {
                let (value_res, mut artifact) = pop_as_bool(frame);

                if let Ok(value) = value_res
                    && value
                {
                    let direction = match self {
                        Igu => Direction::Up,
                        Igr => Direction::Right,
                        Igd => Direction::Down,
                        Igl => Direction::Left,
                        _ => unreachable!(),
                    };

                    artifact.push(frame.act(Box::new(HeadAction::DirectTo(direction))));
                }

                artifact
            }

            Ijp => {
                let (address_res, mut artifact) = pop_address(frame);
                let (operand, ope_artifact) = pop_as_bool_with_default(frame);
                artifact.push(ope_artifact);

                if let Ok(address) = address_res
                    && operand
                {
                    artifact.push(frame.act(Box::new(HeadAction::MoveTo(address.position))));
                }

                artifact
            }

            Equ | Neq => {
                let (rhs_res, mut artifact) = pop_literal(frame);
                let (lhs_res, lhs_artifact) = pop_literal(frame);
                artifact.push(lhs_artifact);

                let result = if let (Ok(rhs), Ok(lhs)) = (rhs_res, lhs_res) {
                    match self {
                        Equ => lhs.eq(&rhs),
                        Neq => lhs.ne(&rhs),
                        _ => unreachable!(),
                    }
                } else {
                    false
                };

                let result_operand = Literal::from_bool(result);
                let push_artifact = frame.act(Box::new(StackAction::Push(result_operand.into())));
                artifact.push(push_artifact);

                artifact
            }

            Grt | Lst | Grq | Lsq => {
                let (rhs, mut artifact) = pop_as_numeric_with_default(frame);
                let (lhs, lhs_artifact) = pop_as_numeric_with_default(frame);
                artifact.push(lhs_artifact);

                let result = match self {
                    Grt => lhs.gt(&rhs),
                    Lst => lhs.lt(&rhs),
                    Grq => lhs.ge(&rhs),
                    Lsq => lhs.le(&rhs),
                    _ => unreachable!(),
                };

                let result_operand = Literal::from_bool(result);
                let push_artifact = frame.act(Box::new(StackAction::Push(result_operand.into())));
                artifact.push(push_artifact);

                artifact
            }

            Add | Sub | Mul | Div => {
                let (rhs, mut artifact) = pop_as_numeric_with_default(frame);
                let (lhs, lhs_artifact) = pop_as_numeric_with_default(frame);
                artifact.push(lhs_artifact);

                let result = match self {
                    Add => lhs.checked_add(rhs).unwrap_or(0),
                    Sub => lhs.saturating_sub(rhs),
                    Mul => lhs.checked_mul(rhs).unwrap_or(0),
                    Div => lhs.checked_div(rhs).unwrap_or(0),
                    _ => unreachable!(),
                };

                let result_operand = Literal::from_number(result);
                let push_artifact = frame.act(Box::new(StackAction::Push(result_operand.into())));
                artifact.push(push_artifact);

                artifact
            }

            Set => {
                let (address_res, mut artifact) = pop_address(frame);
                let (literal_res, lit_artifact) = pop_literal(frame);
                artifact.push(lit_artifact);

                if let (Ok(address), Ok(literal)) = (address_res, literal_res) {
                    let set_artifact = frame.act(Box::new(GridAction::Set(
                        address.position,
                        literal.as_cell(),
                    )));
                    artifact.push(set_artifact);
                }

                artifact
            }

            Prt => {
                let (operand_res, mut artifact) = pop_operand(frame);

                if let Ok(operand) = operand_res {
                    let prt_artifact =
                        frame.act(Box::new(ConsoleAction::Print(operand.as_cell().content())));
                    artifact.push(prt_artifact);
                }

                artifact
            }
        };

        if !matches!(self, Jmp | Hlt | Ijp) {
            artifact.push(frame.act(Box::new(HeadAction::TakeStep())));
        }

        artifact
    }
}
