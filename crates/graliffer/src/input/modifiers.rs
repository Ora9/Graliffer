use std::{
    fmt::{Display, Formatter, Write},
    str::FromStr,
};

use crate::KeystrokeParseError;

/// Key modifiers for keystrokes
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub control: bool,
    pub alt: bool,
    pub shift: bool,
}

impl Modifiers {
    /// None of the modifiers pressed
    pub const NONE: Self = Self {
        control: false,
        alt: false,
        shift: false,
    };

    /// All modifiers pressed
    pub const ALL: Self = Self {
        control: true,
        alt: true,
        shift: true,
    };

    pub const CONTROL: Self = Self {
        control: true,
        alt: false,
        shift: false,
    };

    pub const SHIFT: Self = Self {
        control: false,
        alt: false,
        shift: true,
    };

    pub const ALT: Self = Self {
        control: false,
        alt: true,
        shift: false,
    };
}

impl Modifiers {
    /// Add (or) two modifiers
    ///
    /// ```
    /// # use graliffer::Modifiers;
    /// assert_eq!(
    ///     Modifiers::CONTROL | Modifiers::ALT,
    ///     Modifiers {
    ///         control: true,
    ///         alt: true,
    ///         shift: false
    ///     }
    /// );
    /// ```
    #[must_use]
    pub fn or(self, rhs: Self) -> Self {
        Self {
            control: self.control | rhs.control,
            alt: self.alt | rhs.alt,
            shift: self.shift | rhs.shift,
        }
    }

    /// Test if none of the modifiers keys are pressed
    ///
    /// ```
    /// # use graliffer::Modifiers;
    /// assert!(Modifiers::default().is_none());
    /// ```
    pub fn is_none(&self) -> bool {
        self == &Self::NONE
    }

    /// Test if any of the modifiers keys are pressed
    ///
    /// ```
    /// # use graliffer::Modifiers;
    /// assert!(Modifiers::CONTROL.is_any());
    /// ```
    pub fn is_any(&self) -> bool {
        !self.is_none()
    }

    /// Test if all modifiers keys are pressed
    ///
    /// ```
    /// # use graliffer::Modifiers;
    /// assert!(Modifiers::ALL.is_all());
    /// ```
    pub fn is_all(&self) -> bool {
        self.control && self.alt && self.shift
    }
}

impl std::ops::BitOr for Modifiers {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self.or(rhs)
    }
}

impl Display for Modifiers {
    /// Format modifiers
    ///
    /// ```
    /// # use graliffer::Modifiers;
    /// assert_eq!(Modifiers::CONTROL.to_string(), "ctrl");
    /// assert_eq!(Modifiers::ALL.to_string(), "ctrl-alt-shift");
    /// assert_eq!((Modifiers::ALT | Modifiers::SHIFT).to_string(), "alt-shift");
    /// assert_eq!(Modifiers::NONE.to_string(), "");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ctrl = ("ctrl", self.control);
        let alt = ("alt", self.alt);
        let shift = ("shift", self.shift);

        let mut first = true;

        for (name, is_pressed) in [ctrl, alt, shift] {
            if is_pressed {
                if !first {
                    f.write_char('-')?;
                }

                first = false;

                f.write_str(name)?;
            }
        }

        std::fmt::Result::Ok(())
    }
}

impl FromStr for Modifiers {
    type Err = KeystrokeParseError;

    /// Parse from `&str`
    ///
    /// ```
    /// # use graliffer::{Modifiers, KeystrokeParseError};
    /// # use std::str::FromStr;
    /// assert_eq!(Modifiers::from_str("ctrl")?, Modifiers::CONTROL);
    /// assert_eq!(Modifiers::from_str("alt-shift")?, Modifiers::ALT | Modifiers::SHIFT);
    /// assert_eq!(Modifiers::from_str("ctrl-alt-shift")?, Modifiers::ALL);
    /// # Ok::<(), KeystrokeParseError>(())
    /// ```
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut modifiers = Modifiers::NONE;

        let mut parts = value.split('-');
        while let Some(part) = parts.next() {
            match part.to_ascii_lowercase().as_str() {
                "ctrl" => modifiers.control = true,
                "alt" => modifiers.alt = true,
                "shift" => modifiers.shift = true,
                "" => {}
                _ => {
                    return Err(KeystrokeParseError::InvalidModifiers(String::from(part)));
                }
            }
        }

        Ok(modifiers)
    }
}

// impl TryFrom<&str> for Modifiers {
//     type Error = KeystrokeParseError;
// }

impl From<crossterm::event::KeyModifiers> for Modifiers {
    fn from(modifiers: crossterm::event::KeyModifiers) -> Self {
        Self {
            control: modifiers.intersects(crossterm::event::KeyModifiers::CONTROL),
            shift: modifiers.intersects(crossterm::event::KeyModifiers::SHIFT),
            alt: modifiers.intersects(crossterm::event::KeyModifiers::ALT),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_crossterm() {
        use crossterm::event::KeyModifiers;

        assert_eq!(Modifiers::from(KeyModifiers::all()), Modifiers::ALL);
        assert_eq!(Modifiers::from(KeyModifiers::CONTROL), Modifiers::CONTROL);
        assert_eq!(Modifiers::from(KeyModifiers::ALT), Modifiers::ALT);
        assert_eq!(Modifiers::from(KeyModifiers::SHIFT), Modifiers::SHIFT);

        // Casse toi meta
        assert_eq!(Modifiers::from(KeyModifiers::META), Modifiers::NONE);
    }

    #[test]
    fn or() {
        assert_eq!(
            Modifiers::CONTROL.or(Modifiers::ALT),
            Modifiers::CONTROL | Modifiers::ALT,
        );

        assert_eq!(
            Modifiers::CONTROL | Modifiers::ALT,
            Modifiers {
                control: true,
                alt: true,
                shift: false
            }
        );

        assert_eq!(
            Modifiers::ALT | Modifiers::CONTROL,
            Modifiers::CONTROL | Modifiers::ALT,
        );
    }

    #[test]
    fn none() {
        assert_eq!(Modifiers::NONE, Modifiers::default());
        assert!(Modifiers::NONE.is_none());
    }

    #[test]
    fn any() {
        assert!(Modifiers::CONTROL.is_any());
        assert!(Modifiers::ALL.is_any());
        assert!(!Modifiers::NONE.is_any());
    }

    #[test]
    fn all() {
        assert!(Modifiers::ALL.is_all());
        assert!(!Modifiers::NONE.is_all());
        assert!(!Modifiers::CONTROL.is_all());
    }

    #[test]
    fn display() {
        assert_eq!(Modifiers::CONTROL.to_string(), "ctrl");
        assert_eq!(Modifiers::SHIFT.to_string(), "shift");
        assert_eq!(Modifiers::ALT.to_string(), "alt");
    }

    #[test]
    fn display_order() -> Result<(), KeystrokeParseError> {
        assert_eq!(Modifiers::ALL.to_string(), "ctrl-alt-shift");
        assert_eq!(
            (Modifiers::CONTROL | Modifiers::SHIFT).to_string(),
            "ctrl-shift"
        );
        Ok(())
    }

    #[test]
    fn parse_display() -> Result<(), KeystrokeParseError> {
        assert_eq!(
            Modifiers::from_str(&Modifiers::ALL.to_string())?,
            Modifiers::ALL
        );

        Ok(())
    }

    #[test]
    fn parse() -> Result<(), KeystrokeParseError> {
        assert_eq!(Modifiers::from_str("ctrl")?, Modifiers::CONTROL);
        assert_eq!(Modifiers::from_str("alt")?, Modifiers::ALT);
        assert_eq!(Modifiers::from_str("shift")?, Modifiers::SHIFT);

        assert_eq!(Modifiers::from_str("ctrl-alt-shift")?, Modifiers::ALL);

        assert_eq!(
            Modifiers::from_str("ctrl-shift")?,
            Modifiers::CONTROL | Modifiers::SHIFT
        );

        Ok(())
    }

    #[test]
    fn parse_order() -> Result<(), KeystrokeParseError> {
        assert_eq!(Modifiers::from_str("shift-alt-ctrl")?, Modifiers::ALL);
        assert_eq!(
            Modifiers::from_str("alt-ctrl")?,
            Modifiers::from_str("ctrl-alt")?
        );
        Ok(())
    }

    #[test]
    fn parse_dashes() -> Result<(), KeystrokeParseError> {
        assert_eq!(
            Modifiers::from_str("ctrl--alt")?,
            Modifiers::CONTROL | Modifiers::ALT
        );
        assert_eq!(Modifiers::from_str("-")?, Modifiers::NONE);
        Ok(())
    }

    #[test]
    fn parse_duplicate() -> Result<(), KeystrokeParseError> {
        assert_eq!(Modifiers::from_str("ctrl-ctrl")?, Modifiers::CONTROL);
        assert_eq!(
            Modifiers::from_str("ctrl-alt-ctrl")?,
            Modifiers::CONTROL | Modifiers::ALT
        );
        Ok(())
    }

    #[test]
    fn parse_invalid() -> Result<(), KeystrokeParseError> {
        assert_eq!(
            Modifiers::from_str("oops"),
            Err(KeystrokeParseError::InvalidModifiers("oops".to_string()))
        );
        Ok(())
    }

    #[test]
    fn parse_empty() -> Result<(), KeystrokeParseError> {
        assert_eq!(Modifiers::from_str("")?, Modifiers::NONE);
        assert_eq!(
            Modifiers::from_str(" "),
            Err(KeystrokeParseError::InvalidModifiers(" ".to_string()))
        );
        assert_eq!(
            Modifiers::from_str("ctrl- -alt"),
            Err(KeystrokeParseError::InvalidModifiers(" ".to_string()))
        );
        Ok(())
    }
}
