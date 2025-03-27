use std::str::FromStr;
use strum_macros::EnumString;

use crate::{
    artifact::Artifact, grid::{Cell, Direction, GridAction, Head}, stack::StackAction, Literal, Word
};

use super::{Frame, Operand};

// /// Returns a [`Operand::Literal`] given a string
// ///
// /// Errors :
// /// Return an error if a Cell couldn't be constructed based on input string
// macro_rules! lit {
//     ($literal_string:expr) => {
//         Cell::new($literal_string).map(|cell| Operand::from_literal(Literal::from_cell(cell)))
//     };
// }

fn operand_from_string(string: String) -> Result<Operand, anyhow::Error> {
    Cell::new(string.as_str()).map(|cell| Operand::from_literal(Literal::from_cell(cell)))
}

// fn pop_operand(frame: &mut Frame) -> (Operand, Artifact) {

// }

fn pop_as_numeric(frame: &mut Frame) -> (u32, Artifact) {
    if let Some(last_ope) = frame.stack.get_last() {
        (
            last_ope.resolve_as_numeric(&frame.grid),
            frame.act(StackAction::Pop),
        )
    } else {
        (
            0,
            Artifact::EMPTY,
        )
    }
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

    Set,
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

        let mut move_after = true;

        use Opcode::*;
        let result = match self {
            Nop => {
                Artifact::EMPTY
            }
            Hlt => {
                unimplemented!();
            }
            Dbg => {
                println!("---- DEBUG INFO : Frame dump ----");
                // println!("{:?}", frame);
                println!("---- DEBUG INFO END ----");
                Artifact::EMPTY
            }
            Gou => {
                frame.head.direct_to(Direction::Up);
                Artifact::EMPTY
            }
            Gor => {
                frame.head.direct_to(Direction::Right);
                Artifact::EMPTY
            }
            God => {
                frame.head.direct_to(Direction::Down);
                Artifact::EMPTY
            }
            Gol => {
                frame.head.direct_to(Direction::Left);
                Artifact::EMPTY
            }

            Jmp => {
                // let position = frame
                //     .stack
                //     .pop_err()?
                //     .resolve_to_address(&frame.grid)?
                //     .as_position();

                // frame.head.move_to(position);
                // move_after = false;

                Artifact::EMPTY
            }

            Equ | Neq => {
                let rhs_opt = frame.stack.get_last().map(|ope| ope.to_owned());
                let mut rhs_artifact = frame.act(StackAction::Pop);

                let lhs_opt = frame.stack.get_last().map(|ope| ope.to_owned());
                let lhs_artifact = frame.act(StackAction::Pop);

                rhs_artifact.append_last(lhs_artifact);

                let result = if let (Some(rhs), Some(lhs)) = (&rhs_opt, &lhs_opt) {
                    match self {
                        Equ => lhs.eq(rhs),
                        Neq => lhs.ne(rhs),
                        _ => unreachable!(),
                    }
                } else {
                    false
                } as u8;

                // TODO: better default handling, Cell::NUMERIC_ZERO
                let result_operand = operand_from_string(result.to_string())
                    .unwrap_or(operand_from_string("0".to_string()).unwrap());

                let push_artifact = frame.act(StackAction::Push(result_operand));

                rhs_artifact.append_last(push_artifact);

                rhs_artifact
            }

            Grt | Lst | Grq | Lsq => {
                let rhs = frame.stack.get_last()
                    .map_or(0, |operand| operand.resolve_as_numeric(&frame.grid));
                let mut rhs_artifact = frame.act(StackAction::Pop);

                let lhs = frame.stack.get_last()
                    .map_or(0, |operand| operand.resolve_as_numeric(&frame.grid));
                let lhs_artifact = frame.act(StackAction::Pop);

                rhs_artifact.append_last(lhs_artifact);

                let result = match self {
                    Grt => lhs.gt(&rhs),
                    Lst => lhs.lt(&rhs),
                    Grq => lhs.ge(&rhs),
                    Lsq => lhs.le(&rhs),
                    _ => unreachable!(),
                } as u8;

                // TODO: better default handling, Cell::NUMERIC_ZERO
                let result_operand = operand_from_string(result.to_string())
                    .unwrap_or(operand_from_string("0".to_string()).unwrap());

                let push_artifact = frame.act(StackAction::Push(result_operand));

                rhs_artifact.append_last(push_artifact);

                rhs_artifact
            }

            Add | Sub | Mul | Div => {
                let rhs = frame.stack.get_last()
                    .map_or(0, |operand| operand.resolve_as_numeric(&frame.grid));
                let mut rhs_artifact = frame.act(StackAction::Pop);

                let lhs = frame.stack.get_last()
                    .map_or(0, |operand| operand.resolve_as_numeric(&frame.grid));
                let lhs_artifact = frame.act(StackAction::Pop);

                rhs_artifact.append_last(lhs_artifact);

                let result = match self {
                    Add => lhs.checked_add(rhs).unwrap_or(0),
                    Sub => lhs.checked_sub(rhs).unwrap_or(0),
                    Mul => lhs.checked_mul(rhs).unwrap_or(0),
                    Div => lhs.checked_div(rhs).unwrap_or(0),
                    _ => unreachable!(),
                };

                // TODO: better default handling, Cell::NUMERIC_ZERO
                let result_operand = operand_from_string(result.to_string())
                    .unwrap_or(operand_from_string("0".to_string()).unwrap());

                let push_artifact = frame.act(StackAction::Push(result_operand));

                rhs_artifact.append_last(push_artifact);

                rhs_artifact
            }

            Set => {
                let address_opt = frame.stack.get_last()
                    .map(|operand| operand.resolve_to_address(&frame.grid));
                let mut artifact = frame.act(StackAction::Pop);

                let cell_opt = frame.stack.get_last()
                    .map(|operand| {
                        operand.resolve_to_literal(&frame.grid).as_cell()
                    });
                artifact.append_last(frame.act(StackAction::Pop));

                if let (Some(Ok(address)), Some(cell)) = (address_opt, cell_opt) {
                    artifact.append_last(frame.act(GridAction::Set(address.as_position(), cell)));

                    artifact
                } else {
                    artifact
                }
            }
        };

        if move_after {
            let _ = frame.head.take_step();
        }

        result
    }
}
