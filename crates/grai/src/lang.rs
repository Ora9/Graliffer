mod opcode;
pub use opcode::*;

mod operand;
pub use operand::*;

pub enum Word {
    Operand(Operand),
    Opcode(Opcode),
}
