use std::fmt::Display;

use crate::{
    Axis,
    granary::{GranaryDigit, GranaryError},
};

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

    /// Obtain a `Position` given two valid `u32` numbers
    ///
    /// # Errors
    /// Returns an error if one of the given numerical representation is invalid.
    /// See [`granary`](granary#representation).
    ///
    /// # Examples
    /// ```
    /// # use grai::Position;
    /// let pos = Position::from_numeric(0, 0).unwrap();
    /// assert_eq!(pos.as_numeric(), (0, 0));
    ///
    /// let pos = Position::from_numeric(16, 32).unwrap();
    /// assert_eq!(pos.as_numeric(), (16, 32));
    ///
    /// assert!(Position::from_numeric(63, 0).is_ok());
    /// assert!(Position::from_numeric(64, 0).is_err());
    /// ```
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

    /// Obtain a `Position` given two valid textual representations
    ///
    /// # Errors
    /// Returns an error if one or more the given textual representation are invalid.
    /// See [`Granary`](granary#representation).
    ///
    /// # Examples
    /// ```
    /// # use grai::Position;
    /// let pos = Position::from_textual('A', 'A').unwrap();
    /// assert_eq!(pos.as_numeric(), (0, 0));
    ///
    /// let pos = Position::from_textual('a', '5').unwrap();
    /// assert_eq!(pos.as_numeric(), (26, 57));
    ///
    /// assert!(Position::from_textual('+', 'A').is_ok());
    /// assert!(Position::from_textual('-', 'A').is_err());
    /// ```
    pub fn from_textual(x: char, y: char) -> Result<Self, PositionError> {
        let x = GranaryDigit::from_textual(x).map_err(|err| PositionError::GranaryError {
            axis: Axis::Horizontal,
            granary_error: err,
        })?;
        let y = GranaryDigit::from_textual(y).map_err(|err| PositionError::GranaryError {
            axis: Axis::Vertical,
            granary_error: err,
        })?;

        Ok(Self::from_granary_digits(x, y))
    }
}
