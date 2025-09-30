//! Graliffer is an exotic programming language using a 2 dimensional grid to hold code and data
//!
//! The grid is 64 by 64 cells, holding 3 characters by cells, representing code or data
#![allow(dead_code)]

mod utils;

mod frame;
pub use frame::{Frame, FrameAction, console, grid, head, stack};

mod lang;
pub use lang::{Address, Literal, Opcode, Operand, Pointer, Word};

mod history;
pub use history::{Artifact, History};

mod editor;
pub use editor::Editor;
