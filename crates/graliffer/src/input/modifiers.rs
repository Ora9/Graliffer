use std::fmt::{Display, Formatter, Write};

use crate::KeystrokeParseError;

/// Key modifiers for keystrokes
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub control: bool,
    pub alt: bool,
    pub shift: bool,
}

impl Modifiers {
    pub const NONE: Self = Self {
        control: false,
        alt: false,
        shift: false,
    };

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
    /// Add two modifiers
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
    pub fn or(self, rhs: Self) -> Self {
        Self {
            control: self.control | rhs.control,
            alt: self.alt | rhs.alt,
            shift: self.shift | rhs.shift,
        }
    }

    /// Test if none of the modifiers keys are pressed
    /// ```
    /// # use graliffer::Modifiers;
    /// assert!(Modifiers::default().is_none());
    /// ```
    pub fn is_none(&self) -> bool {
        self == &Self::NONE
    }

    /// Test if any of the modifiers keys are pressed
    /// ```
    /// # use graliffer::Modifiers;
    /// assert!(Modifiers::CONTROL.is_any());
    /// ```
    pub fn is_any(&self) -> bool {
        !self.is_none()
    }

    /// Test if all modifiers keys are pressed
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

impl TryFrom<&str> for Modifiers {
    type Error = KeystrokeParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
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

        /// Fuck meta
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
    fn parse() -> Result<(), KeystrokeParseError> {
        assert_eq!(Modifiers::try_from("ctrl")?, Modifiers::CONTROL);
        assert_eq!(Modifiers::try_from("alt")?, Modifiers::ALT);
        assert_eq!(Modifiers::try_from("shift")?, Modifiers::SHIFT);

        assert_eq!(Modifiers::try_from("ctrl-alt-shift")?, Modifiers::ALL);

        assert_eq!(
            Modifiers::try_from("ctrl-shift")?,
            Modifiers::CONTROL | Modifiers::SHIFT
        );

        Ok(())
    }

    #[test]
    fn parse_order() -> Result<(), KeystrokeParseError> {
        assert_eq!(Modifiers::try_from("shift-alt-ctrl")?, Modifiers::ALL);
        assert_eq!(
            Modifiers::try_from("alt-ctrl")?,
            Modifiers::try_from("ctrl-alt")?
        );
        Ok(())
    }

    #[test]
    fn parse_dashes() -> Result<(), KeystrokeParseError> {
        assert_eq!(
            Modifiers::try_from("ctrl--alt")?,
            Modifiers::CONTROL | Modifiers::ALT
        );
        assert_eq!(Modifiers::try_from("-")?, Modifiers::NONE);
        Ok(())
    }

    #[test]
    fn parse_duplicate() -> Result<(), KeystrokeParseError> {
        assert_eq!(Modifiers::try_from("ctrl-ctrl")?, Modifiers::CONTROL);
        assert_eq!(
            Modifiers::try_from("ctrl-alt-ctrl")?,
            Modifiers::CONTROL | Modifiers::ALT
        );
        Ok(())
    }

    #[test]
    fn parse_invalid() -> Result<(), KeystrokeParseError> {
        assert_eq!(
            Modifiers::try_from("oops"),
            Err(KeystrokeParseError::InvalidModifiers("oops".to_string()))
        );
        Ok(())
    }

    #[test]
    fn parse_empty() -> Result<(), KeystrokeParseError> {
        assert_eq!(Modifiers::try_from("")?, Modifiers::NONE);
        assert_eq!(
            Modifiers::try_from(" "),
            Err(KeystrokeParseError::InvalidModifiers(" ".to_string()))
        );
        assert_eq!(
            Modifiers::try_from("ctrl- -alt"),
            Err(KeystrokeParseError::InvalidModifiers(" ".to_string()))
        );
        Ok(())
    }
}
