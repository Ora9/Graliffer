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

    /// Returns a `Position` given two [`GranaryDigits`]
    ///
    /// # Example
    /// ```
    /// # use grai::{Position, granary::GranaryDigit};
    /// let x = GranaryDigit::from_numeric(0).unwrap();
    /// let y = GranaryDigit::from_numeric(0).unwrap();
    /// let pos = Position::from_granary_digits(x, y);
    /// assert_eq!(pos.as_numeric(), (0, 0));
    ///
    /// let x = GranaryDigit::from_numeric(5).unwrap();
    /// let y = GranaryDigit::from_numeric(10).unwrap();
    /// let pos = Position::from_granary_digits(x, y);
    /// assert_eq!(pos.as_numeric(), (5, 10));
    /// ```
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

    /// Returns the textual representation of a `Position` as a tuple in form `(x, y)`
    pub fn as_textual(&self) -> (char, char) {
        (self.x_as_textual(), self.y_as_textual())
    }

    /// Returns the textual representation of a `Position` as `String`
    pub fn as_textual_string(&self) -> String {
        format!("{}{}", self.x_as_textual(), self.y_as_textual())
    }

    /// Returns the numeric representation of a `Position` as a tuple in form `(x, y)`
    pub fn as_numeric(&self) -> (u32, u32) {
        (self.x(), self.y())
    }

    /// Returns the numeric representation of the `x` (horizontal) component of a `Position`
    pub fn x(&self) -> u32 {
        self.x.as_numeric()
    }

    /// Returns the textual representation of the `x` (horizontal) component of a `Position`
    pub fn x_as_textual(&self) -> char {
        self.x.as_textual()
    }

    /// Returns the numeric representation of the `y` (vertical) component of a `Position`
    pub fn y(&self) -> u32 {
        self.y.as_numeric()
    }

    /// Returns the textual representation of the `y` (vertical) component of a `Position`
    pub fn y_as_textual(&self) -> char {
        self.y.as_textual()
    }
}
