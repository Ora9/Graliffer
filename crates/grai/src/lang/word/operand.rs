use crate::{Cell, Position};

pub struct Literal(Cell);
pub struct Address(Position);
pub struct Pointer(Position);

pub enum Operand {
    Literal(Literal),
    Address(Address),
    Pointer(Pointer),
}
