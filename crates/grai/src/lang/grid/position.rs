use std::fmt::Display;

use crate::{Axis, GranaryDigit, GranaryError};

// #[derive(Debug, Hash, PartialEq, Eq)]
// pub struct PositionAxis(GranaryDigit);

// Vimpl PositionAxis {
//     pub const ORIGIN: Self = Self(GranaryDigit::ZERO);
// }

#[derive(Debug, thiserror::Error)]
pub enum PositionError {
    #[error("invalid granary for the {axis} axis")]
    GranaryError {
        axis: Axis,
        #[source]
        granary_error: GranaryError,
    },
}

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

    pub fn from_granary_digits(x: GranaryDigit, y: GranaryDigit) -> Self {
        Self { x, y }
    }

    pub fn from_numeric(x: u32, y: u32) -> Result<Self, PositionError> {
        let x = GranaryDigit::from_numeric(x).map_err(|err| PositionError::GranaryError {
            axis: Axis::Horizontal,
            granary_error: err,
        })?;
        let y = GranaryDigit::from_numeric(y).map_err(|err| PositionError::GranaryError {
            axis: Axis::Vertical,
            granary_error: err,
        })?;

        Ok(Self::from_granary_digits(x, y))
    }
}
