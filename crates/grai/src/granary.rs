//! Graliffer uses the Granary numeral system, positional system
//!
//! It is used to minimize the number of character used to represent a number
//!
//! Currently, Base64 is used, but in the future the standard might better make use of the whole unicode character set
//!
//! # Representation
//! Currently Granary uses the same alphabet as [base64](https://en.wikipedia.org/wiki/Base64)
//!
//! Example : `A = 0`, `z = 51`, `/ = 63`

use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum GranaryError {
    #[error("invalid numeric representation, expected to be in range [0-63], found `{0}`")]
    InvalidNumericRepresentation(u32),
    #[error(
        "invalid textual representation, expected to be in character set [A-Za-z0-9/+], found `{0}`"
    )]
    InvalidTextualRepresentation(String),

    #[error("this operation would overflow")]
    WouldOverflowDigit,
    #[error("this operation would underflow")]
    WouldUnderflowDigit,
}

// impl Display for GranaryError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match GranaryError {
//             GranaryError::InvalidNumericRepresentation => "Out"
//         }
//     }
// }

/// `GranaryDigit` is a single digit in the Granary numeral system
///
/// # Examples
///
/// ```
/// # use grai::GranaryDigit;
/// let pos = GranaryDigit::from_textual('A').unwrap();
/// assert_eq!(pos.as_numeric(), 0);
///
/// let pos = GranaryDigit::from_numeric(51).unwrap();
/// assert_eq!(pos.as_textual(), 'z');
/// ```
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GranaryDigit(u8);

impl GranaryDigit {
    pub const ZERO: Self = Self(0);
    pub const MAX: Self = Self(63);

    pub const MIN_NUMERIC: u32 = 0;
    pub const MAX_NUMERIC: u32 = 63;

    /// Returns `true` if the given number is considered a valid numeric representation
    ///
    /// Currently any number within the inclusive range `[0-63]` is valid
    ///
    /// # Example
    /// ```
    /// # use grai::GranaryDigit;
    /// assert!( GranaryDigit::is_valid_numeric(0));
    /// assert!( GranaryDigit::is_valid_numeric(25));
    /// assert!( GranaryDigit::is_valid_numeric(63));
    /// assert!(!GranaryDigit::is_valid_numeric(64));
    /// ```
    pub fn is_valid_numeric(value: u32) -> bool {
        (Self::MIN_NUMERIC..=Self::MAX_NUMERIC).contains(&value)
    }

    /// Restrict a value to the limit of the numeric representation.
    ///
    /// # Examples
    /// ```
    /// # use grai::GranaryDigit;
    /// assert_eq!(GranaryDigit::clamp_numeric(0), 0);
    /// assert_eq!(GranaryDigit::clamp_numeric(25), 25);
    /// assert_eq!(GranaryDigit::clamp_numeric(63), 63);
    /// assert_eq!(GranaryDigit::clamp_numeric(64), 63);
    /// ```
    pub fn clamp_numeric(value: u32) -> u32 {
        value.clamp(Self::MIN_NUMERIC, Self::MAX_NUMERIC)
    }

    /// Return `true` if the given `char` is considered a valid textual representation.
    ///
    /// Currently, any char in the set `[A-Za-z0-9+/]` is valid. See [base64](https://en.wikipedia.org/wiki/Base64) for more infos
    ///
    /// # Example
    /// ```
    /// # use grai::GranaryDigit;
    /// assert!( GranaryDigit::is_valid_textual('A'));
    /// assert!( GranaryDigit::is_valid_textual('/'));
    /// assert!( GranaryDigit::is_valid_textual('q'));
    /// assert!(!GranaryDigit::is_valid_textual('-'));
    /// ```
    pub fn is_valid_textual(value: char) -> bool {
        matches!(value, 'A'..='Z' | 'a'..='z' | '0'..='9' | '+' | '/')
    }

    /// Return a textual representation from a given numeric representation
    ///
    /// Error :
    /// Returns an error if the given numeric representation is invalid
    /// See the [`granary module`](granary#representation).
    ///
    /// # Example
    /// ```
    /// # use grai::GranaryDigit;
    /// assert_ne!(GranaryDigit::numeric_to_textual(0).unwrap(), 'a');
    /// assert_eq!(GranaryDigit::numeric_to_textual(5).unwrap(), 'F');
    /// assert_eq!(GranaryDigit::numeric_to_textual(26).unwrap(), 'a');
    /// assert_eq!(GranaryDigit::numeric_to_textual(52).unwrap(), '0');
    /// ```
    pub fn numeric_to_textual(value: u32) -> Result<char, GranaryError> {
        let char_index = match value {
            0..=25 => value + 65,       // A-Z
            26..=51 => value - 26 + 97, // a-z
            52..=61 => value - 52 + 48, // 0-9
            62 => 43,                   // +
            63 => 47,                   // /
            _ => {
                return Err(GranaryError::InvalidNumericRepresentation(value));
            }
        };

        Ok(u8::try_from(char_index).expect("we should already have returned") as char)
    }

    /// Return a numeric representation from a given textual representation
    ///
    /// Error :
    /// Returns an error if the given textual representation is invalid
    /// See the [`granary module`](granary#representation).
    ///
    /// # Example
    /// ```
    /// # use grai::GranaryDigit;
    /// assert_eq!(GranaryDigit::textual_to_numeric('Y').unwrap(), 24);
    /// assert_eq!(GranaryDigit::textual_to_numeric('5').unwrap(), 57);
    /// assert_eq!(GranaryDigit::textual_to_numeric('+').unwrap(), 62);
    /// assert_ne!(GranaryDigit::textual_to_numeric('R').unwrap(), 34);
    /// ```
    pub fn textual_to_numeric(value: char) -> Result<u32, GranaryError> {
        let value_u32 = value as u32;

        match value_u32 {
            65..=90 => Ok(value_u32 - 65),       // A-Z
            97..=122 => Ok(value_u32 - 97 + 26), // a-z
            48..=57 => Ok(value_u32 - 48 + 52),  // 0-9
            43 => Ok(62),                        // +
            47 => Ok(63),                        // /
            _ => Err(GranaryError::InvalidTextualRepresentation(value.into())),
        }
    }

    /// Get a `GranaryDigit` given a valid numeric representation
    ///
    /// # Error
    /// Returns an error if the given numeric representation is invalid.
    /// See the [`granary module`](granary#representation).
    ///
    /// # Example
    /// ```
    /// # use grai::GranaryDigit;
    /// let digit = GranaryDigit::from_numeric(0).unwrap();
    /// assert_eq!(digit.as_numeric(), 0);
    ///
    /// let digit = GranaryDigit::from_numeric(26).unwrap();
    /// assert_eq!(digit.as_numeric(), 26);
    ///
    /// assert!(GranaryDigit::from_numeric(63).is_ok());
    /// assert!(GranaryDigit::from_numeric(64).is_err());
    /// ```
    pub fn from_numeric(value: u32) -> Result<Self, GranaryError> {
        if !GranaryDigit::is_valid_numeric(value) {
            Err(GranaryError::InvalidNumericRepresentation(value))
        } else {
            Ok(Self(
                u8::try_from(value).expect("we should have already returned"),
            ))
        }
    }

    /// Returns a `GranaryDigit` given a valid textual representation
    ///
    /// # Errors
    /// Returns an error if the given textual representation is invalid.
    /// See the [`granary module`](granary#representation).
    ///
    /// # Examples
    /// ```
    /// # use grai::GranaryDigit;
    /// let digit = GranaryDigit::from_textual('A').unwrap();
    /// assert_eq!(digit.as_numeric(), 0);
    ///
    /// let digit = GranaryDigit::from_textual('a').unwrap();
    /// assert_eq!(digit.as_numeric(), 26);
    ///
    /// assert!(GranaryDigit::from_textual('+').is_ok());
    /// assert!(GranaryDigit::from_textual('-').is_err());
    /// ```
    pub fn from_textual(coordinate: char) -> Result<Self, GranaryError> {
        Self::from_numeric(Self::textual_to_numeric(coordinate)?)
    }

    /// Returns the numeric representation of a `Granary`
    pub fn as_numeric(self) -> u32 {
        self.0.into()
    }

    /// Returns the textual representation of a `Granary`
    pub fn as_textual(self) -> char {
        Self::numeric_to_textual(self.0 as u32)
            .expect("a valid granary should always have a textual representation")
    }

    /// Performs an addition on two [`GranaryDigit`]
    ///
    /// Errors
    /// Returns an error if the addition could not be performed (overflowing one digit).
    ///
    /// Examples
    /// ```
    /// # use grai::GranaryDigit;
    /// let zero = GranaryDigit::ZERO;
    /// let five = GranaryDigit::from_numeric(5).unwrap();
    /// let ten = GranaryDigit::from_numeric(10).unwrap();
    /// let fifteen = GranaryDigit::from_numeric(15).unwrap();
    /// let max = GranaryDigit::from_numeric(GranaryDigit::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(ten.checked_add(five).unwrap(), fifteen);
    /// assert_eq!(five.checked_add(five).unwrap(), ten);
    /// assert_eq!(fifteen.checked_add(zero).unwrap(), fifteen);
    /// assert!(max.checked_add(ten).is_err());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn checked_add(&self, other: Self) -> Result<Self, GranaryError> {
        let sum = self
            .0
            .checked_add(other.0)
            .ok_or(GranaryError::WouldOverflowDigit)?;

        Self::from_numeric(sum.into())
    }

    /// Performs a substraction on two [`GranaryDigit`]
    ///
    /// Errors
    /// Returns an error if the substraction could not be performed (underflowing one digit).
    ///
    /// Examples
    /// ```
    /// # use grai::GranaryDigit;
    /// let zero = GranaryDigit::ZERO;
    /// let five = GranaryDigit::from_numeric(5).unwrap();
    /// let ten = GranaryDigit::from_numeric(10).unwrap();
    /// let fifteen = GranaryDigit::from_numeric(15).unwrap();
    /// let max = GranaryDigit::from_numeric(GranaryDigit::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(ten.checked_sub(five).unwrap(), five);
    /// assert_eq!(fifteen.checked_sub(ten).unwrap(), five);
    /// assert_eq!(ten.checked_sub(zero).unwrap(), ten);
    /// assert!(ten.checked_sub(fifteen).is_err());
    /// assert!(ten.checked_sub(max).is_err());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn checked_sub(&self, other: Self) -> Result<Self, GranaryError> {
        let diff = self
            .0
            .checked_sub(other.0)
            .ok_or(GranaryError::WouldUnderflowDigit)?;

        Self::from_numeric(diff.into())
    }

    /// Perform an addition between a [`GranaryDigit`] and a `u32`
    ///
    /// # Errors
    /// Returns an error if the addition could not be performed (overflowing one digit).
    ///
    /// # Examples
    /// ```
    /// # use grai::GranaryDigit;
    /// let zero = GranaryDigit::ZERO;
    /// let ten = GranaryDigit::from_numeric(10).unwrap();
    /// let five = GranaryDigit::from_numeric(5).unwrap();
    /// let fifteen = GranaryDigit::from_numeric(15).unwrap();
    /// let max = GranaryDigit::from_numeric(GranaryDigit::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(five.checked_increment_by(10).unwrap(), fifteen);
    /// assert_eq!(ten.checked_increment_by(5).unwrap(), fifteen);
    /// assert_eq!(ten.checked_increment_by(0).unwrap(), ten);
    /// assert_eq!(zero.checked_increment_by(GranaryDigit::MAX_NUMERIC).unwrap(), max);
    /// assert!(max.checked_increment_by(1).is_err());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn checked_increment_by(&self, value: u32) -> Result<Self, GranaryError> {
        let sum = (self.0 as u32)
            .checked_add(value)
            .ok_or(GranaryError::WouldOverflowDigit)?;

        Self::from_numeric(sum)
    }

    /// Perform a substraction between a [`GranaryDigit`] and a `u32`
    ///
    /// # Errors
    /// Returns an error if the substraction could not be performed (underflowing one digit).
    ///
    /// # Examples
    /// ```
    /// # use grai::GranaryDigit;
    /// let zero = GranaryDigit::ZERO;
    /// let five = GranaryDigit::from_numeric(5).unwrap();
    /// let ten = GranaryDigit::from_numeric(10).unwrap();
    /// let fifteen = GranaryDigit::from_numeric(15).unwrap();
    /// let max = GranaryDigit::from_numeric(GranaryDigit::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(ten.checked_decrement(5).unwrap(), five);
    /// assert_eq!(fifteen.checked_decrement(10).unwrap(), five);
    /// assert_eq!(five.checked_decrement(5).unwrap(), zero);
    /// assert_eq!(max.checked_decrement(GranaryDigit::MAX_NUMERIC).unwrap(), zero);
    /// assert!(zero.checked_decrement(1).is_err());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn checked_decrement(&self, value: u32) -> Result<Self, GranaryError> {
        let diff = (self.0 as u32)
            .checked_sub(value)
            .ok_or(GranaryError::WouldUnderflowDigit)?;

        Self::from_numeric(diff)
    }
}

impl From<GranaryDigit> for u32 {
    fn from(value: GranaryDigit) -> Self {
        value.as_numeric()
    }
}

impl From<GranaryDigit> for u8 {
    fn from(value: GranaryDigit) -> Self {
        value.0
    }
}

impl From<GranaryDigit> for char {
    fn from(value: GranaryDigit) -> Self {
        value.as_textual()
    }
}

impl TryFrom<u8> for GranaryDigit {
    type Error = GranaryError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        GranaryDigit::from_numeric(value.into())
    }
}

impl TryFrom<u32> for GranaryDigit {
    type Error = GranaryError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        GranaryDigit::from_numeric(value)
    }
}

impl TryFrom<char> for GranaryDigit {
    type Error = GranaryError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        GranaryDigit::from_textual(value)
    }
}

impl Debug for GranaryDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GranaryDigit({}, {})",
            self.as_numeric(),
            self.as_textual(),
        )
    }
}
