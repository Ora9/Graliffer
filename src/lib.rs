//! Graliffer is an exotic programming language using a 2 dimensional grid to hold code and data
//!
//! The grid is 64 by 64 cells, holding 3 characters by cells, representing code or data

pub mod grid;

pub mod stack;

mod lang;
pub use lang::{
    Word,
    Opcode,
    Operand,
    Literal,
    Address,
    Pointer,
};

mod action;
// pub use action;

mod run;
pub use run::{
    RunDescriptor,
    Frame,
};

mod app;
pub use app::{
    GralifferApp,
};
