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
    #[error("this operation would overflow the {axis} axis")]
    WouldOverflow {
        axis: Axis,
        #[source]
        granary_error: GranaryError,
    },
    #[error("this operation would underflow the {axis} axis")]
    WouldUnderflow {
        axis: Axis,
        #[source]
        granary_error: GranaryError,
    },
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

impl Position {
    /// Performs an addition on two [`Position`]
    ///
    /// Errors
    /// Returns an error if the addition could not be performed (overflowing the [`Grid`] limits).
    ///
    /// Examples
    /// ```
    /// # use grai::{Position, granary::GranaryDigit};
    /// let zero = Position::ORIGIN;
    /// let five_ten = Position::from_numeric(5, 10).unwrap();
    /// let ten_twenty = Position::from_numeric(10, 20).unwrap();
    /// let max = Position::from_numeric(GranaryDigit::MAX_NUMERIC, GranaryDigit::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(five_ten.checked_add(five_ten).unwrap(), ten_twenty);
    /// assert_eq!(zero.checked_add(five_ten).unwrap(), five_ten);
    /// assert_eq!(max.checked_add(zero).unwrap(), max);
    /// assert!(max.checked_add(ten_twenty).is_err());
    #[must_use = "this returns the result of an operation, without modifying the original"]
    pub fn checked_add(&self, other: Self) -> Result<Self, PositionError> {
        let x = self
            .x
            .checked_add(other.x)
            .map_err(|err| PositionError::WouldOverflow {
                axis: Axis::Horizontal,
                granary_error: err,
            })?;
        let y = self
            .y
            .checked_add(other.y)
            .map_err(|err| PositionError::WouldOverflow {
                axis: Axis::Horizontal,
                granary_error: err,
            })?;

        Ok(Self::from_granary_digits(x, y))
    }

    /// Performs a substraction on two [`Position`]
    ///
    /// Errors
    /// Returns an error if the substraction could not be performed (underflowing the [`Grid`] limits).
    ///
    /// Examples
    /// ```
    /// # use grai::{Position, granary::GranaryDigit};
    /// let zero = Position::ORIGIN;
    /// let five_ten = Position::from_numeric(5, 10).unwrap();
    /// let ten_twenty = Position::from_numeric(10, 20).unwrap();
    /// let max = Position::from_numeric(GranaryDigit::MAX_NUMERIC, GranaryDigit::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(five_ten.checked_sub(five_ten).unwrap(), zero);
    /// assert_eq!(five_ten.checked_sub(zero).unwrap(), five_ten);
    /// assert_eq!(ten_twenty.checked_sub(five_ten).unwrap(), five_ten);
    /// assert!(five_ten.checked_sub(ten_twenty).is_err());
    #[must_use = "this returns the result of an operation, without modifying the original"]
    pub fn checked_sub(&self, other: Self) -> Result<Self, PositionError> {
        let x = self
            .x
            .checked_sub(other.x)
            .map_err(|err| PositionError::WouldUnderflow {
                axis: Axis::Horizontal,
                granary_error: err,
            })?;
        let y = self
            .y
            .checked_sub(other.y)
            .map_err(|err| PositionError::WouldUnderflow {
                axis: Axis::Horizontal,
                granary_error: err,
            })?;

        Ok(Self::from_granary_digits(x, y))
    }

    /// Perform an addition between a the `x` component of a [`Position`] and an `u32`
    ///
    /// # Errors
    /// Returns an error if the addition could not be performed (overflowing one digit).
    ///
    /// # Examples
    /// ```
    /// # use grai::{Position, granary::GranaryDigit};
    /// let five_seven = Position::from_numeric(5, 7).unwrap();
    /// let six_seven = Position::from_numeric(6, 7).unwrap();
    /// let zero_twelve = Position::from_numeric(0, 12).unwrap();
    /// let max_twelve = Position::from_numeric(GranaryDigit::MAX_NUMERIC, 12).unwrap();
    ///
    /// assert_eq!(five_seven.checked_increment_x_by(1).unwrap(), six_seven);
    /// assert_eq!(zero_twelve.checked_increment_x_by(6).unwrap().x(), six_seven.x());
    /// assert_eq!(zero_twelve.checked_increment_x_by(GranaryDigit::MAX_NUMERIC).unwrap(), max_twelve);
    /// assert!(max_twelve.checked_increment_x_by(1).is_err());
    /// ```
    pub fn checked_increment_x_by(&self, value: u32) -> Result<Self, PositionError> {
        Ok(Self::from_granary_digits(
            self.x
                .checked_increment_by(value)
                .map_err(|err| PositionError::WouldOverflow {
                    axis: Axis::Horizontal,
                    granary_error: err,
                })?,
            self.y,
        ))
    }

    /// Perform an addition between a the `y` component of a [`Position`] and an `u32`
    ///
    /// # Errors
    /// Returns an error if the addition could not be performed (overflowing one digit).
    ///
    /// # Examples
    /// ```
    /// # use grai::{Position, granary::GranaryDigit};
    /// let six_six = Position::from_numeric(6, 6).unwrap();
    /// let six_seven = Position::from_numeric(6, 7).unwrap();
    /// let twelve_zero = Position::from_numeric(12, 0).unwrap();
    /// let twelve_max = Position::from_numeric(12, GranaryDigit::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(six_six.checked_increment_y_by(1).unwrap(), six_seven);
    /// assert_eq!(twelve_zero.checked_increment_y_by(7).unwrap().y(), six_seven.y());
    /// assert_eq!(twelve_zero.checked_increment_y_by(GranaryDigit::MAX_NUMERIC).unwrap(), twelve_max);
    /// assert!(twelve_max.checked_increment_y_by(1).is_err());
    /// ```
    pub fn checked_increment_y_by(&self, value: u32) -> Result<Self, PositionError> {
        Ok(Self::from_granary_digits(
            self.x,
            self.y
                .checked_increment_by(value)
                .map_err(|err| PositionError::WouldOverflow {
                    axis: Axis::Vertical,
                    granary_error: err,
                })?,
        ))
    }

    /// Perform a substraction between a the `x` component of a [`Position`] and an `u32`
    ///
    /// # Errors
    /// Returns an error if the substraction could not be performed (underflowing one digit).
    ///
    /// # Examples
    /// ```
    /// # use grai::{Position, granary::GranaryDigit};
    /// let seven_seven = Position::from_numeric(7, 7).unwrap();
    /// let six_seven = Position::from_numeric(6, 7).unwrap();
    /// let zero_twelve = Position::from_numeric(0, 12).unwrap();
    /// let max_twelve = Position::from_numeric(GranaryDigit::MAX_NUMERIC, 12).unwrap();
    ///
    /// assert_eq!(seven_seven.checked_decrement_x_by(1).unwrap(), six_seven);
    /// assert_eq!(max_twelve.checked_decrement_x_by(GranaryDigit::MAX_NUMERIC - 6).unwrap().x(), six_seven.x());
    /// assert_eq!(max_twelve.checked_decrement_x_by(GranaryDigit::MAX_NUMERIC).unwrap(), zero_twelve);
    /// assert!(zero_twelve.checked_decrement_x_by(1).is_err());
    /// ```
    pub fn checked_decrement_x_by(&self, value: u32) -> Result<Self, PositionError> {
        Ok(Self::from_granary_digits(
            self.x
                .checked_decrement(value)
                .map_err(|err| PositionError::WouldUnderflow {
                    axis: Axis::Vertical,
                    granary_error: err,
                })?,
            self.y,
        ))
    }

    /// Perform a substraction between a the `y` component of a [`Position`] and an `u32`
    ///
    /// # Errors
    /// Returns an error if the substraction could not be performed (underflowing one digit).
    ///
    /// # Examples
    /// ```
    /// # use grai::{Position, granary::GranaryDigit};
    /// let six_eight = Position::from_numeric(6, 8).unwrap();
    /// let six_seven = Position::from_numeric(6, 7).unwrap();
    /// let twelve_zero = Position::from_numeric(12, 0).unwrap();
    /// let twelve_max = Position::from_numeric(12, GranaryDigit::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(six_eight.checked_decrement_y_by(1).unwrap(), six_seven);
    /// assert_eq!(twelve_max.checked_decrement_y_by(GranaryDigit::MAX_NUMERIC - 7).unwrap().y(), six_seven.y());
    /// assert_eq!(twelve_max.checked_decrement_y_by(GranaryDigit::MAX_NUMERIC).unwrap(), twelve_zero);
    /// assert!(twelve_zero.checked_decrement_y_by(1).is_err());
    /// ```
    pub fn checked_decrement_y_by(&self, value: u32) -> Result<Self, PositionError> {
        Ok(Self::from_granary_digits(
            self.x,
            self.y
                .checked_decrement(value)
                .map_err(|err| PositionError::WouldUnderflow {
                    axis: Axis::Vertical,
                    granary_error: err,
                })?,
        ))
    }

    /// "Take a step"
    pub fn checked_step(&self, direction: Direction, value: u32) -> Result<Self, PositionError> {
        match direction {
            Direction::Up => self.checked_decrement_y_by(value),
            Direction::Right => self.checked_increment_x_by(value),
            Direction::Down => self.checked_increment_y_by(value),
            Direction::Left => self.checked_decrement_x_by(value),
        }
    }
}
