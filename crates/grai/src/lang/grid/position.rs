use std::fmt::Display;

use crate::GranaryDigit;

// #[derive(Debug, Hash, PartialEq, Eq)]
// pub struct PositionAxis(GranaryDigit);

// Vimpl PositionAxis {
//     pub const ORIGIN: Self = Self(GranaryDigit::ZERO);
// }

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Position {
    x: GranaryDigit,
    y: GranaryDigit,
}

impl Position {
    pub const ORIGIN: Self = Self {
        x: GranaryDigit::ZERO,
        y: GranaryDigit::ZERO,
    };
}
