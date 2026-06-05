use crate::{Cell, Position};

#[derive(Debug)]
pub struct Literal(Cell);

#[derive(Debug)]
pub struct Address(Position);

#[derive(Debug)]
pub struct Pointer(Position);

#[derive(Debug)]
pub enum Operand {
    Literal(Literal),
    Address(Address),
    Pointer(Pointer),
}
