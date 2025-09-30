use std::fmt::Debug;

use anyhow::{Context, bail};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

use crate::utils::Direction;

/// `PositionAxis` represents a coordinate axis in the [Grid](crate::grid::Grid). A combination of two `PositionAxis` makes a [`Position`]
///
/// # Representation
/// A `PositionAxis` have two representation :
/// - Numeric : any number in range `[0-63]`
/// - Textual : using the same character to number correspondence as [base64](https://en.wikipedia.org/wiki/Base64)
///
/// # Examples
///
/// ```
/// # use graliffer::grid::PositionAxis;
/// let pos = PositionAxis::from_textual('A').unwrap();
/// assert_eq!(pos.as_numeric(), 0);
///
/// let pos = PositionAxis::from_numeric(51).unwrap();
/// assert_eq!(pos.as_textual(), 'z');
/// ```
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PositionAxis(u8);

impl PositionAxis {
    /// Minimum numeric value that can take a `PositionAxis`
    pub const MIN_NUMERIC: u32 = 0;
    /// Maximum numeric value that can take a `PositionAxis`. Anything higher is invalid
    pub const MAX_NUMERIC: u32 = 63;

    /// PositionAxis placed at origin (0)
    pub const ORIGIN: PositionAxis = PositionAxis(0);

    /// Return `true` if the given `u32` is considered a valid numeric representation of [`PositionAxis`]
    ///
    /// Any number within the inclusive range `[0-63]` is valid
    ///
    /// # Example
    ///
    /// ```
    /// # use graliffer::grid::PositionAxis;
    /// assert!( PositionAxis::is_valid_numeric(0));
    /// assert!( PositionAxis::is_valid_numeric(63));
    /// assert!( PositionAxis::is_valid_numeric(42));
    /// assert!(!PositionAxis::is_valid_numeric(64));
    /// ```
    pub fn is_valid_numeric(value: u32) -> bool {
        (Self::MIN_NUMERIC..=Self::MAX_NUMERIC).contains(&value)
    }

    /// Restrict a value to the limit of the numeric representation of `PositionAxis`.
    ///
    /// # Examples
    /// ```
    /// # use graliffer::grid::PositionAxis;
    /// assert_eq!(PositionAxis::clamp_numeric(0), 0);
    /// assert_eq!(PositionAxis::clamp_numeric(25), 25);
    /// assert_eq!(PositionAxis::clamp_numeric(63), 63);
    /// assert_eq!(PositionAxis::clamp_numeric(64), 63);
    /// ```
    pub fn clamp_numeric(value: u32) -> u32 {
        value.clamp(Self::MIN_NUMERIC, Self::MAX_NUMERIC)
    }

    /// Return `true` if the given `char` is considered a valid textual representation of [`PositionAxis`]
    ///
    /// Any char in set `[A-Za-z0-9+/]` is valid. See [base64](https://en.wikipedia.org/wiki/Base64) for more infos
    ///
    /// # Example
    /// ```
    /// # use graliffer::grid::PositionAxis;
    /// assert!( PositionAxis::is_valid_textual('A'));
    /// assert!( PositionAxis::is_valid_textual('/'));
    /// assert!( PositionAxis::is_valid_textual('q'));
    /// assert!(!PositionAxis::is_valid_textual('-'));
    /// ```
    pub fn is_valid_textual(value: char) -> bool {
        matches!(value, 'A'..='Z' | 'a'..='z' | '0'..='9' | '+' | '/')
    }

    /// Return a textual representation from a given numeric representation
    ///
    /// Error :
    /// Returns an error if the given numeric representation is invalid
    /// See [`PositionAxis`](PositionAxis#representation).
    ///
    /// # Example
    /// ```
    /// # use graliffer::grid::PositionAxis;
    /// assert_eq!(PositionAxis::numeric_to_textual(5).unwrap(), 'F');
    /// assert_eq!(PositionAxis::numeric_to_textual(52).unwrap(), '0');
    /// assert_eq!(PositionAxis::numeric_to_textual(26).unwrap(), 'a');
    /// assert_ne!(PositionAxis::numeric_to_textual(0).unwrap(), 'a');
    /// ```
    pub fn numeric_to_textual(coordinate: u32) -> Result<char, anyhow::Error> {
        if !PositionAxis::is_valid_numeric(coordinate) {
            bail!(format!(
                "The given coordinate is out of bound, expected to be in range [0-63], found `{}`",
                coordinate
            ));
        }

        let coordinate = u8::try_from(coordinate).unwrap();

        match coordinate {
            0..=25 => Ok((coordinate + 65) as char),       // A-Z
            26..=51 => Ok((coordinate - 26 + 97) as char), // a-z
            52..=61 => Ok((coordinate - 52 + 48) as char), // 0-9
            62 => Ok(43 as char),                          // +
            63 => Ok(47 as char),                          // /
            _ => bail!(format!(
                "The given coordinate is out of bound, expected to be in range [0-63], found `{}`",
                coordinate
            )),
        }
    }

    /// Return a numeric representation from a given textual representation
    ///
    /// Error :
    /// Returns an error if the given textual representation is invalid
    /// See [`PositionAxis`](PositionAxis#representation).
    ///
    /// # Example
    /// ```
    /// # use graliffer::grid::PositionAxis;
    /// assert_eq!(PositionAxis::textual_to_numeric('Y').unwrap(), 24);
    /// assert_eq!(PositionAxis::textual_to_numeric('5').unwrap(), 57);
    /// assert_eq!(PositionAxis::textual_to_numeric('+').unwrap(), 62);
    /// assert_ne!(PositionAxis::textual_to_numeric('R').unwrap(), 34);
    /// ```
    pub fn textual_to_numeric(coordinate: char) -> Result<u32, anyhow::Error> {
        let coordinate_u32 = coordinate as u32;

        match coordinate_u32 {
            65..=90 => Ok(coordinate_u32 - 65),       // A-Z
            97..=122 => Ok(coordinate_u32 - 97 + 26), // a-z
            48..=57 => Ok(coordinate_u32 - 48 + 52),  // 0-9
            43 => Ok(62),                             // +
            47 => Ok(63),                             // /
            _ => bail!(format!(
                "The given coordinate is out of bound, expected to be in character set [A-Za-z0-9/+], found `{}`",
                coordinate
            )),
        }
    }

    /// Get a [`PositionAxis`] given a valid numeric representation
    ///
    /// # Error
    /// Returns an error if the given numeric representation is invalid.
    /// See [`PositionAxis`](PositionAxis#representation).
    ///
    /// # Example
    /// ```
    /// # use graliffer::grid::PositionAxis;
    /// let pos = PositionAxis::from_numeric(0).unwrap();
    /// assert_eq!(pos.as_numeric(), 0);
    ///
    /// let pos = PositionAxis::from_numeric(26).unwrap();
    /// assert_eq!(pos.as_numeric(), 26);
    ///
    /// assert!(PositionAxis::from_numeric(63).is_ok());
    /// assert!(PositionAxis::from_numeric(64).is_err());
    /// ```
    pub fn from_numeric(coordinate: u32) -> Result<Self, anyhow::Error> {
        if !PositionAxis::is_valid_numeric(coordinate) {
            bail!(format!(
                "The given coordinate is out of bound, expected to be in range [0-63], found `{}`",
                coordinate
            ))
        } else {
            Ok(Self(u8::try_from(coordinate).unwrap()))
        }
    }

    /// Returns a [`PositionAxis`] given a valid textual representation
    ///
    /// # Errors
    /// Returns an error if the given textual representation is invalid. See [`PositionAxis`](PositionAxis#representation).
    ///
    /// # Examples
    /// ```
    /// # use graliffer::grid::PositionAxis;
    /// let pos = PositionAxis::from_textual('A').unwrap();
    /// assert_eq!(pos.as_numeric(), 0);
    ///
    /// let pos = PositionAxis::from_textual('a').unwrap();
    /// assert_eq!(pos.as_numeric(), 26);
    ///
    /// assert!(PositionAxis::from_textual('+').is_ok());
    /// assert!(PositionAxis::from_textual('-').is_err());
    /// ```
    pub fn from_textual(coordinate: char) -> Result<Self, anyhow::Error> {
        Self::from_numeric(Self::textual_to_numeric(coordinate)?)
    }

    /// Returns the numeric representation of a `PositionAxis`
    pub fn as_numeric(self) -> u32 {
        self.0.into()
    }

    /// Returns the textual representation of a `PositionAxis`
    pub fn as_textual(self) -> char {
        Self::numeric_to_textual(self.0 as u32).unwrap()
    }

    /// Performs an addition on two [`PositionAxis`]
    ///
    /// Errors
    /// Returns an error if the addition could not be performed (overflowing the [`Grid`] limits).
    ///
    /// Examples
    /// ```
    /// # use graliffer::grid::PositionAxis;
    /// let zero = PositionAxis::ZERO;
    /// let five = PositionAxis::from_numeric(5).unwrap();
    /// let ten = PositionAxis::from_numeric(10).unwrap();
    /// let fifteen = PositionAxis::from_numeric(15).unwrap();
    /// let too_big = PositionAxis::from_numeric(PositionAxis::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(ten.checked_add(five).unwrap(), fifteen);
    /// assert_eq!(five.checked_add(five).unwrap(), ten);
    /// assert_eq!(fifteen.checked_add(zero).unwrap(), fifteen);
    /// assert!(ten.checked_add(too_big).is_err());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn checked_add(&self, other: Self) -> Result<Self, anyhow::Error> {
        {
            let sum = self
                .0
                .checked_add(other.0)
                .ok_or(anyhow::anyhow!("adding these would overflow"))?;
            Self::from_numeric(sum.into())
        }
        .context("could not add these two `PositionAxis`s")
    }

    /// Performs a substraction between two [`PostionAxis`]
    ///
    /// Errors
    /// Returns an error if the substraction could not be performed (underflowing the [`Grid`] limits).
    ///
    /// Examples
    /// ```
    /// # use graliffer::grid::PositionAxis;
    /// let zero = PositionAxis::ZERO;
    /// let five = PositionAxis::from_numeric(5).unwrap();
    /// let ten = PositionAxis::from_numeric(10).unwrap();
    /// let fifteen = PositionAxis::from_numeric(15).unwrap();
    /// let too_big = PositionAxis::from_numeric(PositionAxis::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(ten.checked_sub(five).unwrap(), five);
    /// assert_eq!(fifteen.checked_sub(ten).unwrap(), five);
    /// assert_eq!(ten.checked_sub(zero).unwrap(), ten);
    /// assert!(ten.checked_sub(fifteen).is_err());
    /// assert!(ten.checked_sub(too_big).is_err());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn checked_sub(&self, other: Self) -> Result<Self, anyhow::Error> {
        {
            let diff = self
                .0
                .checked_sub(other.0)
                .ok_or(anyhow::anyhow!("subtracting these would underflow"))?;
            Self::from_numeric(diff.into())
        }
        .context("could not add these two `PositionAxis`s")
    }

    /// Perform an addition between a [`PositionAxis`] and a `u32`
    ///
    /// # Errors
    /// Returns an error if the addition could not be performed (overflowing the [`Grid`] limits).
    ///
    /// # Examples
    /// ```
    /// # use graliffer::grid::PositionAxis;
    /// let five = PositionAxis::from_numeric(5).unwrap();
    /// let ten = PositionAxis::from_numeric(10).unwrap();
    /// let fifteen = PositionAxis::from_numeric(15).unwrap();
    /// let too_big = PositionAxis::from_numeric(PositionAxis::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(five.checked_increment(10).unwrap(), fifteen);
    /// assert_eq!(ten.checked_increment(5).unwrap(), fifteen);
    /// assert_eq!(ten.checked_increment(0).unwrap(), ten);
    /// assert!(too_big.checked_increment(1).is_err());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn checked_increment(&self, value: u32) -> Result<Self, anyhow::Error> {
        {
            let sum = (self.0 as u32).checked_add(value)
                .ok_or(anyhow::anyhow!("adding these would overflow"))?;
            Self::from_numeric(sum)
        }.context(format!("could not increment further, attempted to increment {:?} by {}, but result must be in range [0-63]", self, value))
    }

    /// Perform a substraction between a [`PositionAxis`] and a `u32`
    ///
    /// # Errors
    /// Returns an error if the substraction could not be performed (underflowing the [`Grid`] limits).
    ///
    /// # Examples
    /// ```
    /// # use graliffer::grid::PositionAxis;
    /// let zero = PositionAxis::ZERO;
    /// let five = PositionAxis::from_numeric(5).unwrap();
    /// let ten = PositionAxis::from_numeric(10).unwrap();
    /// let fifteen = PositionAxis::from_numeric(15).unwrap();
    /// let too_big = PositionAxis::from_numeric(PositionAxis::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(ten.checked_decrement(5).unwrap(), five);
    /// assert_eq!(fifteen.checked_decrement(10).unwrap(), five);
    /// assert_eq!(five.checked_decrement(5).unwrap(), zero);
    /// assert_eq!(too_big.checked_decrement(PositionAxis::MAX_NUMERIC).unwrap(), zero);
    /// assert!(zero.checked_decrement(1).is_err());
    /// ```
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn checked_decrement(&self, value: u32) -> Result<Self, anyhow::Error> {
        {
            let diff = (self.0 as u32).checked_sub(value)
                .ok_or(anyhow::anyhow!("adding these would underflow"))?;
            Self::from_numeric(diff)
        }.context(format!("could not decrement further, attempted to decrement {:?} by {}, but result must be in range [0-63]", self, value))
    }
}

impl From<PositionAxis> for u32 {
    fn from(coordinate: PositionAxis) -> Self {
        coordinate.as_numeric()
    }
}

impl From<PositionAxis> for u8 {
    fn from(coordinate: PositionAxis) -> Self {
        coordinate.0
    }
}

impl From<PositionAxis> for char {
    fn from(coordinate: PositionAxis) -> Self {
        coordinate.as_textual()
    }
}

impl TryFrom<u8> for PositionAxis {
    type Error = anyhow::Error;

    fn try_from(coordinate: u8) -> Result<Self, Self::Error> {
        PositionAxis::from_numeric(coordinate.into())
    }
}

impl TryFrom<u32> for PositionAxis {
    type Error = anyhow::Error;

    fn try_from(coordinate: u32) -> Result<Self, Self::Error> {
        PositionAxis::from_numeric(coordinate)
    }
}

impl TryFrom<char> for PositionAxis {
    type Error = anyhow::Error;

    fn try_from(coordinate: char) -> Result<Self, Self::Error> {
        PositionAxis::from_textual(coordinate)
    }
}

impl Debug for PositionAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PositionAxis (`{}` ({}))",
            self.as_textual(),
            self.as_numeric()
        )
    }
}

/// A `Position` represents a 2d coordinate in the [Grid](crate::grid::Grid). A `Position` is made of two [`PositionAxis`]
///
/// # Representation
/// A `Position` have two representation :
/// - Numeric : any two number in range `[0-63]`
/// - Textual : two chars using the same character to number correspondence as [base64](https://fr.wikipedia.org/wiki/Base64)
///
/// As a convention, any two unlabeled parameters (numeric or textual) are ordered like `xy`, `x` being the horizontal axis of a `Grid`, `y` being vertical
/// As a convention, when refering to a position (either in `Address` or `Pointer` operands)
/// # Examples
///
/// ```
/// # use graliffer::grid::Position;
/// let pos = Position::from_numeric(0, 0).unwrap();
/// assert_eq!(pos.as_textual(), ('A', 'A'));
///
/// let pos = Position::from_textual('a', '/').unwrap();
/// assert_eq!(pos.as_numeric(), (26, 63));
/// ```
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: PositionAxis,
    y: PositionAxis,
}

impl Position {
    /// A Position placed at the origin (0)
    pub const ORIGIN: Position = Position {
        x: PositionAxis::ORIGIN,
        y: PositionAxis::ORIGIN,
    };

    /// Returns a `Position` given two [`PositionAxis`]
    ///
    /// # Example
    /// ```
    /// # use graliffer::grid::{Position, PositionAxis};
    /// let x = PositionAxis::from_numeric(0).unwrap();
    /// let y = PositionAxis::from_numeric(0).unwrap();
    /// let pos = Position::from_position_axis(x, y);
    /// assert_eq!(pos.as_numeric(), (0, 0));
    ///
    /// let x = PositionAxis::from_numeric(5).unwrap();
    /// let y = PositionAxis::from_numeric(10).unwrap();
    /// let pos = Position::from_position_axis(x, y);
    /// assert_eq!(pos.as_numeric(), (5, 10));
    /// ```
    pub fn from_position_axis(x: PositionAxis, y: PositionAxis) -> Self {
        Self { x, y }
    }

    /// Returns a `Position` given two valid numeric representations
    ///
    /// # Errors
    /// Returns an error if one or more the given numeric representation are invalid.
    /// See [`PositionAxis`](PositionAxis#representation).
    ///
    /// # Examples
    /// ```
    /// # use graliffer::grid::Position;
    /// let pos = Position::from_numeric(0, 0).unwrap();
    /// assert_eq!(pos.as_numeric(), (0, 0));
    ///
    /// let pos = Position::from_numeric(16, 32).unwrap();
    /// assert_eq!(pos.as_numeric(), (16, 32));
    ///
    /// assert!(Position::from_numeric(63, 0).is_ok());
    /// assert!(Position::from_numeric(64, 0).is_err());
    /// ```
    pub fn from_numeric(x: u32, y: u32) -> Result<Self, anyhow::Error> {
        let x = PositionAxis::from_numeric(x).context("`x` coordinate is invalid")?;
        let y = PositionAxis::from_numeric(y).context("`y` coordinate is invalid")?;

        Ok(Self::from_position_axis(x, y))
    }

    /// Returns a `Position` given two valid textual representations
    ///
    /// # Errors
    /// Returns an error if one or more the given textual representation are invalid.
    /// See [`PositionAxis`](PositionAxis#representation).
    ///
    /// # Examples
    /// ```
    /// # use graliffer::grid::Position;
    /// let pos = Position::from_textual('A', 'A').unwrap();
    /// assert_eq!(pos.as_numeric(), (0, 0));
    ///
    /// let pos = Position::from_textual('a', '5').unwrap();
    /// assert_eq!(pos.as_numeric(), (26, 57));
    ///
    /// assert!(Position::from_textual('+', 'A').is_ok());
    /// assert!(Position::from_textual('-', 'A').is_err());
    /// ```
    pub fn from_textual(x: char, y: char) -> Result<Self, anyhow::Error> {
        let x = PositionAxis::from_textual(x).context("`x` coordinate is invalid")?;
        let y = PositionAxis::from_textual(y).context("`y` coordinate is invalid")?;

        Ok(Self::from_position_axis(x, y))
    }

    /// Returns the textual representation of a `Position` as tuple in form `(x, y)`
    pub fn as_textual(self) -> (char, char) {
        (self.x.as_textual(), self.y.as_textual())
    }

    pub fn as_textual_string(self) -> String {
        format!("{}{}", self.x.as_textual(), self.y.as_textual())
    }

    /// Returns the numeric representation of a `Position` as tuple in form `(x, y)`
    pub fn as_numeric(self) -> (u32, u32) {
        (self.x.as_numeric(), self.y.as_numeric())
    }

    /// Return the numeric representation of the `x` (horizontal) component of a `Position`
    pub fn x(self) -> u32 {
        self.x.as_numeric()
    }

    /// Return the numeric representation of the `y` (vertical) component of a `Position`
    pub fn y(self) -> u32 {
        self.y.as_numeric()
    }

    /// Performs an addition on two [`Position`]
    ///
    /// Errors
    /// Returns an error if the addition could not be performed (overflowing the [`Grid`] limits).
    ///
    /// Examples
    /// ```
    /// # use graliffer::grid::{PositionAxis, Position};
    /// let zero = Position::ZERO;
    /// let five_ten = Position::from_numeric(5, 10).unwrap();
    /// let ten_twenty = Position::from_numeric(10, 20).unwrap();
    /// let too_big = Position::from_numeric(PositionAxis::MAX_NUMERIC, PositionAxis::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(five_ten.checked_add(five_ten).unwrap(), ten_twenty);
    /// assert_eq!(zero.checked_add(five_ten).unwrap(), five_ten);
    /// assert!(ten_twenty.checked_add(too_big).is_err());
    pub fn checked_add(&self, other: Self) -> Result<Self, anyhow::Error> {
        let x = self
            .x
            .checked_add(other.x)
            .context("`x` coordinate is invalid")?;
        let y = self
            .y
            .checked_add(other.y)
            .context("`y` coordinate is invalid")?;

        Ok(Self::from_position_axis(x, y))
    }

    /// Performs a substraction on two [`Position`]
    ///
    /// Errors
    /// Returns an error if the substracton could not be performed (underflowing the [`Grid`] limits).
    ///
    /// Examples
    /// ```
    /// # use graliffer::grid::{PositionAxis, Position};
    /// let zero = Position::ZERO;
    /// let five_ten = Position::from_numeric(5, 10).unwrap();
    /// let ten_twenty = Position::from_numeric(10, 20).unwrap();
    /// let too_big = Position::from_numeric(PositionAxis::MAX_NUMERIC, PositionAxis::MAX_NUMERIC).unwrap();
    ///
    /// assert_eq!(five_ten.checked_sub(five_ten).unwrap(), zero);
    /// assert_eq!(ten_twenty.checked_sub(zero).unwrap(), ten_twenty);
    /// assert_eq!(ten_twenty.checked_sub(five_ten).unwrap(), five_ten);
    /// assert!(five_ten.checked_sub(ten_twenty).is_err());
    /// assert!(ten_twenty.checked_sub(too_big).is_err());
    pub fn checked_sub(&self, other: Self) -> Result<Self, anyhow::Error> {
        let x = self
            .x
            .checked_sub(other.x)
            .context("`x` coordinate is invalid")?;
        let y = self
            .y
            .checked_sub(other.y)
            .context("`y` coordinate is invalid")?;

        Ok(Self::from_position_axis(x, y))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn checked_increment_x(&self, value: u32) -> Result<Self, anyhow::Error> {
        Ok(Self::from_position_axis(
            self.x.checked_increment(value)?,
            self.y,
        ))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn checked_increment_y(&self, value: u32) -> Result<Self, anyhow::Error> {
        Ok(Self::from_position_axis(
            self.x,
            self.y.checked_increment(value)?,
        ))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn checked_decrement_x(&self, value: u32) -> Result<Self, anyhow::Error> {
        Ok(Self::from_position_axis(
            self.x.checked_decrement(value)?,
            self.y,
        ))
    }

    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn checked_decrement_y(&self, value: u32) -> Result<Self, anyhow::Error> {
        Ok(Self::from_position_axis(
            self.x,
            self.y.checked_decrement(value)?,
        ))
    }

    pub fn checked_step(&self, direction: Direction, offset: u32) -> Result<Self, anyhow::Error> {
        match direction {
            Direction::Up => self.checked_decrement_y(offset),
            Direction::Right => self.checked_increment_x(offset),
            Direction::Down => self.checked_increment_y(offset),
            Direction::Left => self.checked_decrement_x(offset),
        }.context("could not step out of the grid")
    }
}

impl TryFrom<&str> for Position {
    type Error = anyhow::Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let mut chars = string.chars();
        let x = chars.next();
        let y = chars.next();

        if let (Some(x), Some(y)) = (x, y) {
            Position::from_textual(x, y)
        } else {
            bail!(format!(
                "The string given does not respect the format, expected to be in format 'XX' with 'X' being a base64 character, found `{}`",
                string
            ))
        }
    }
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pos = self.as_textual();
        serializer.serialize_str(format!("{}{}", pos.0, pos.1).as_str())
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PositionVisitor;

        impl<'de> Visitor<'de> for PositionVisitor {
            type Value = Position;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("valid position (`A-Za-z+/`)")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Position::try_from(v).map_err(|error| serde::de::Error::custom(error))
            }
        }

        deserializer.deserialize_str(PositionVisitor)
    }
}
// pub trait Deserialize<'de>: Sized {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>;
// }

/// ```
/// use graliffer::grid::Position;
/// dbg!(Position::from_numeric(5, 8).unwrap());
/// ```
impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position (`{}` ({}, {}))",
            self.as_textual_string(),
            self.x(),
            self.y()
        )
    }
}
