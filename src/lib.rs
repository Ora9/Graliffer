//! Graliffer is an exotic programming language using a 2 dimensional grid to hold code and data
//!
//! The grid is 64 by 64 cells, holding 3 characters by cells, representing code or data
#![allow(dead_code)]

mod utils;

mod frame;
pub use frame::{Frame, grid, stack, head};

mod lang;
pub use lang::{Address, Literal, Opcode, Operand, Pointer, Word};

pub mod console;

mod action;

mod editor;
pub use editor::Editor;
