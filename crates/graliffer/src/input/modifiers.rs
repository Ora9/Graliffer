use std::{
    fmt::{Display, Formatter, Write},
    str::FromStr,
};

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

#[derive(Debug, PartialEq, Eq)]
pub enum UnexpectedDashPlacement {
    Leading,
    Trailing,
    Double,
}

impl Display for UnexpectedDashPlacement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Leading => f.write_str("leading"),
            Self::Trailing => f.write_str("trailing"),
            Self::Double => f.write_str("double"),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ModifiersParseError {
    #[error("invalid modifier, got `{got}`")]
    InvalidModifier { got: String },

    #[error(
        "unexpected {dash_placement} dash (`-`), expected only one dash seperating every modifier"
    )]
    UnexpectedDash {
        dash_placement: UnexpectedDashPlacement,
    },
}

impl FromStr for Modifiers {
    type Err = ModifiersParseError;

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

        let mut parts = value.split('-').enumerate().peekable();

        while let Some((i, part)) = parts.next() {
            match part.to_ascii_lowercase().as_str() {
                "ctrl" => modifiers.control = true,
                "alt" => modifiers.alt = true,
                "shift" => modifiers.shift = true,
                "" if value.len() == 0 => {
                    // empty source ""
                    return Ok(Modifiers::NONE);
                }
                "" if i == 0 => {
                    // leading dash "-.."
                    return Err(ModifiersParseError::UnexpectedDash {
                        dash_placement: UnexpectedDashPlacement::Leading,
                    });
                }
                "" if parts.peek().is_none() => {
                    // trailing dash "..-"
                    return Err(ModifiersParseError::UnexpectedDash {
                        dash_placement: UnexpectedDashPlacement::Trailing,
                    });
                }
                "" => {
                    // double dash "..--.."
                    return Err(ModifiersParseError::UnexpectedDash {
                        dash_placement: UnexpectedDashPlacement::Double,
                    });
                }
                _ => {
                    return Err(ModifiersParseError::InvalidModifier {
                        got: part.to_string(),
                    });
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
    fn display_order() -> Result<(), ModifiersParseError> {
        assert_eq!(Modifiers::ALL.to_string(), "ctrl-alt-shift");
        assert_eq!(
            (Modifiers::CONTROL | Modifiers::SHIFT).to_string(),
            "ctrl-shift"
        );
        Ok(())
    }

    #[test]
    fn parse_display() -> Result<(), ModifiersParseError> {
        assert_eq!(
            Modifiers::from_str(&Modifiers::ALL.to_string())?,
            Modifiers::ALL
        );

        Ok(())
    }

    #[test]
    fn parse() -> Result<(), ModifiersParseError> {
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
    fn parse_order() -> Result<(), ModifiersParseError> {
        assert_eq!(Modifiers::from_str("shift-alt-ctrl")?, Modifiers::ALL);
        assert_eq!(
            Modifiers::from_str("alt-ctrl")?,
            Modifiers::from_str("ctrl-alt")?
        );
        Ok(())
    }

    #[test]
    fn parse_ignore_case() -> Result<(), ModifiersParseError> {
        assert_eq!(
            Modifiers::from_str("ctrl-alt")?,
            Modifiers::from_str("AlT-CtRl")?
        );
        Ok(())
    }

    #[test]
    fn parse_double_dashes_error() -> Result<(), ModifiersParseError> {
        assert_eq!(
            Modifiers::from_str("ctrl--alt"),
            Err(ModifiersParseError::UnexpectedDash {
                dash_placement: UnexpectedDashPlacement::Double
            })
        );
        Ok(())
    }

    #[test]
    fn parse_trailing_dashes_error() -> Result<(), ModifiersParseError> {
        assert_eq!(
            Modifiers::from_str("shift-"),
            Err(ModifiersParseError::UnexpectedDash {
                dash_placement: UnexpectedDashPlacement::Trailing
            })
        );
        Ok(())
    }

    #[test]
    fn parse_leading_dashes_error() -> Result<(), ModifiersParseError> {
        assert_eq!(
            Modifiers::from_str("-shift-ctrl"),
            Err(ModifiersParseError::UnexpectedDash {
                dash_placement: UnexpectedDashPlacement::Leading
            })
        );

        Ok(())
    }

    #[test]
    fn parse_only_dashes_error() -> Result<(), ModifiersParseError> {
        assert_eq!(
            Modifiers::from_str("-"),
            Err(ModifiersParseError::UnexpectedDash {
                dash_placement: UnexpectedDashPlacement::Leading
            })
        );

        assert_eq!(
            Modifiers::from_str("------"),
            Err(ModifiersParseError::UnexpectedDash {
                dash_placement: UnexpectedDashPlacement::Leading
            })
        );

        Ok(())
    }

    #[test]
    fn parse_ignore_duplicate() -> Result<(), ModifiersParseError> {
        assert_eq!(Modifiers::from_str("ctrl-ctrl")?, Modifiers::CONTROL);
        assert_eq!(
            Modifiers::from_str("ctrl-alt-ctrl")?,
            Modifiers::CONTROL | Modifiers::ALT
        );
        Ok(())
    }

    #[test]
    fn parse_invalid_modifier_name() -> Result<(), ModifiersParseError> {
        assert_eq!(
            Modifiers::from_str("oops"),
            Err(ModifiersParseError::InvalidModifier {
                got: "oops".to_string()
            })
        );
        assert_eq!(
            Modifiers::from_str("control-alt-shift"),
            Err(ModifiersParseError::InvalidModifier {
                got: "control".to_string()
            })
        );
        Ok(())
    }

    #[test]
    fn parse_whitespace() -> Result<(), ModifiersParseError> {
        assert_eq!(
            Modifiers::from_str(" "),
            Err(ModifiersParseError::InvalidModifier {
                got: " ".to_string()
            })
        );
        assert_eq!(
            Modifiers::from_str("ctrl- -alt"),
            Err(ModifiersParseError::InvalidModifier {
                got: " ".to_string()
            })
        );
        Ok(())
    }

    #[test]
    fn parse_empty() -> Result<(), ModifiersParseError> {
        assert_eq!(Modifiers::from_str("")?, Modifiers::NONE);
        Ok(())
    }
}
