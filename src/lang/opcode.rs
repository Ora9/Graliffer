use std::str::FromStr;
use strum_macros::EnumString;

use crate::{grid::{
    Direction,
    Head,
    Cell,
}, Literal, Word};

use super::{Frame, Operand};

/// Returns a [`Operand::Literal`] given a string
///
/// Errors :
/// Return an error if a Cell couldn't be constructed based on input string
macro_rules! lit {
    ($literal_string:expr) => {
        Cell::new($literal_string).map(|cell| {Operand::from_literal(Literal::from_cell(cell))})
    };
}

#[derive(Debug, Clone, Copy, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Opcode {
    // Debug
    Dbg,

    // Program management
    Hlt,

    // Basic head movements
    Gou,
    Gor,
    God,
    Gol,

    // Arithmetic operations
    Add,
    Sub,
    Mul,
    Div,

    // Comparaison operations
    // Equ,
    // Neq,
    // Grt,
    // Lst,
    // Leq,
    // Geq,

    Set


}

impl Opcode {
    pub fn from_cell(cell: Cell) -> Result<Opcode, anyhow::Error> {
        Opcode::from_str(&cell.content()).map_err(|_| anyhow::anyhow!(format!("not a valid opcode")))
    }

    pub fn is_cell_valid(cell: &Cell) -> bool {
        Self::from_str(&cell.content()).is_ok()
    }

    pub fn evaluate(self, frame: &mut Frame) -> Result<(), anyhow::Error>{
        use Opcode::*;
        match self {
            Hlt => {
                unimplemented!();
            }
            Dbg => {
                println!("---- DEBUG INFO : Frame dump ----");
                println!("{:?}", frame);
                println!("---- DEBUG INFO END ----");
                Ok(())
            }
            Gou => {
                frame.head.direct_to(Direction::Up);
                Ok(())
            }
            Gor => {
                frame.head.direct_to(Direction::Right);
                Ok(())
            }
            God => {
                frame.head.direct_to(Direction::Down);
                Ok(())
            }
            Gol => {
                frame.head.direct_to(Direction::Left);
                Ok(())
            }

            Add | Sub | Mul | Div => {
                let rhs = frame.stack.pop_err()?.as_numeric(&frame.grid);
                let lhs = frame.stack.pop_err()?.as_numeric(&frame.grid);

                let sum = match self {
                    Add => { lhs.checked_add(rhs).unwrap_or(0) },
                    Sub => { lhs.checked_sub(rhs).unwrap_or(0) },
                    Mul => { lhs.checked_mul(rhs).unwrap_or(0) },
                    Div => { lhs.checked_div(rhs).unwrap_or(0) },
                    _ => unreachable!()
                };

                frame.stack.push(lit!(&sum.to_string())?);

                Ok(())
            }

            Set => {
                let position = frame.stack.pop_err()?.resolve_to_address(&frame.grid)?.as_position();
                let cell = frame.stack.pop_err()?.as_cell();

                frame.grid.set(position, cell);

                Ok(())
            }
        }
    }
}
