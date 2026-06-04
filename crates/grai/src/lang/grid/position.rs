use std::fmt::Display;

pub enum BaseGError {
    InvalidNumericRepresentation,
    InvalidTextualRepresentation,
}

// impl Display for BaseGError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match BaseGError {
//             BaseGError::InvalidNumericRepresentation => "Out"
//         }
//     }
// }

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BaseG(u8);

impl BaseG {
    pub const ZERO: Self = Self(0);

    pub const MIN_NUMERIC: u32 = 0;
    pub const MAX_NUMERIC: u32 = 63;

    /// Returns `true` if the given number is considered a valid numeric representation
    ///
    /// Currently any number within the inclusive range `[0-63]` is valid
    ///
    /// # Example
    /// ```
    /// # use grai::Based;
    /// assert!( Based::is_valid_numeric(0))
    /// assert!( Based::is_valid_numeric(25))
    /// assert!( Based::is_valid_numeric(63))
    /// assert!(!Based::is_valid_numeric(64))
    /// ```
    pub fn is_valid_numeric(value: u32) -> bool {
        (Self::MIN_NUMERIC..=Self::MAX_NUMERIC).contains(&value)
    }

    /// Restrict a value to the limit of the numeric representation.
    ///
    /// # Examples
    /// ```
    /// # use grai::Based;
    /// assert_eq!(Based::clamp_numeric(0), 0);
    /// assert_eq!(Based::clamp_numeric(25), 25);
    /// assert_eq!(Based::clamp_numeric(63), 63);
    /// assert_eq!(Based::clamp_numeric(64), 63);
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
    /// assert_ne!(PositionAxis::numeric_to_textual(0).unwrap(), 'a');
    /// assert_eq!(PositionAxis::numeric_to_textual(5).unwrap(), 'F');
    /// assert_eq!(PositionAxis::numeric_to_textual(26).unwrap(), 'a');
    /// assert_eq!(PositionAxis::numeric_to_textual(52).unwrap(), '0');
    /// ```
    pub fn numeric_to_textual(value: u32) -> Result<char, BaseGError> {
        let char_index = match value {
            0..=25 => value + 65,       // A-Z
            26..=51 => value - 26 + 97, // a-z
            52..=61 => value - 52 + 48, // 0-9
            62 => 43,                   // +
            63 => 47,                   // /
            _ => return Err(BaseGError::InvalidNumericRepresentation),
        };

        Ok(u8::try_from(char_index).expect("we should already have returned") as char)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PositionAxis(BaseG);

impl PositionAxis {
    pub const ORIGIN: Self = Self(BaseG::ZERO);
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Position {
    x: PositionAxis,
    y: PositionAxis,
}
