use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::{Key, KeyFromCrosstermError, KeyParseError, Modifiers, ModifiersParseError};

// A single keystroke, with a key press, and currently pressed modifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keystroke {
    pub modifiers: Modifiers,
    pub key: Key,
}

impl Keystroke {
    pub fn new(key: Key, modifiers: Modifiers) -> Self {
        Self { modifiers, key }
    }

    /// Get keystroke from key, with default modifiers (none pressed)
    pub fn from_key(key: Key) -> Self {
        Self {
            key,
            modifiers: Modifiers::NONE,
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum KeystrokeParseError {
    #[error("error while parsing the modifiers part, expected valid keystroke, got `{got}`")]
    ModifiersParseError {
        got: String,
        source_error: ModifiersParseError,
    },

    #[error("error while parsing the key part, expected valid keystroke, got `{got}`")]
    KeyParseError {
        got: String,
        source_error: KeyParseError,
    },
}

impl FromStr for Keystroke {
    type Err = KeystrokeParseError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let parse_modifiers = |modifiers: &str| {
            modifiers
                .parse()
                .map_err(|err| KeystrokeParseError::ModifiersParseError {
                    got: source.to_string(),
                    source_error: err,
                })
        };

        let parse_key = |key: &str| {
            key.parse()
                .map_err(|err| KeystrokeParseError::KeyParseError {
                    got: source.to_string(),
                    source_error: err,
                })
        };

        if source.ends_with("-") && source.len() == 1 {
            // Dash key "-"
            Ok(Self {
                modifiers: Modifiers::NONE,
                key: parse_key(source)?,
            })
        } else if source.ends_with("--") {
            // Dash key with modifiers "..--"
            Ok(Self {
                modifiers: parse_modifiers(source.trim_end_matches("--"))?,
                key: parse_key("-")?,
            })
        } else if let Some((modifiers, key)) = source.rsplit_once("-") {
            // Key with modifiers "..-.."
            Ok(Self {
                modifiers: parse_modifiers(modifiers)?,
                key: parse_key(key)?,
            })
        } else {
            // Key without modifiers ".."
            Ok(Self {
                modifiers: Modifiers::NONE,
                key: parse_key(source)?,
            })
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum KeystrokeFromCrosstermError {
    #[error("invalid crossterm key for keystroke")]
    KeyFromCrosstermError(#[source] KeyFromCrosstermError),
}

impl TryFrom<crossterm::event::KeyEvent> for Keystroke {
    type Error = KeystrokeFromCrosstermError;

    fn try_from(event: crossterm::event::KeyEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            key: event
                .code
                .try_into()
                .map_err(|err| KeystrokeFromCrosstermError::KeyFromCrosstermError(err))?,
            modifiers: event.modifiers.into(),
        })
    }
}

impl Display for Keystroke {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let modifiers = self.modifiers.to_string();
        let key = self.key.to_string();

        if modifiers.len() == 0 {
            write!(f, "{key}")?;
        } else {
            write!(f, "{modifiers}-{key}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::UnexpectedDashPlacement;

    use super::*;

    #[test]
    fn from_key() {
        assert_eq!(
            Keystroke::from_key(Key::Char('a')),
            Keystroke {
                modifiers: Modifiers::NONE,
                key: Key::Char('a')
            }
        );
        assert!(Keystroke::from_key(Key::Up).modifiers.is_none());
    }

    #[test]
    fn display() {
        assert_eq!(Keystroke::from_key(Key::Escape).to_string(), "escape");
        assert_eq!(Keystroke::from_key(Key::Char('a')).to_string(), "a");
    }

    #[test]
    fn display_modifiers() {
        assert_eq!(
            Keystroke {
                key: Key::Char('a'),
                modifiers: Modifiers::CONTROL
            }
            .to_string(),
            "ctrl-a"
        );

        assert_eq!(
            Keystroke {
                key: Key::Delete,
                modifiers: Modifiers::ALL
            }
            .to_string(),
            "ctrl-alt-shift-delete"
        );
    }

    #[test]
    fn parse() -> Result<(), KeystrokeParseError> {
        assert_eq!(
            Keystroke::from_str("enter")?,
            Keystroke::from_key(Key::Enter)
        );

        assert_eq!(
            Keystroke::from_str("s")?,
            Keystroke::from_key(Key::Char('s'))
        );

        Ok(())
    }

    #[test]
    fn parse_modifiers() -> Result<(), KeystrokeParseError> {
        assert_eq!(
            Keystroke::from_str("ctrl-pageup")?,
            Keystroke::new(Key::PageUp, Modifiers::CONTROL),
        );

        assert_eq!(
            Keystroke::from_str("alt-shift-a")?,
            Keystroke::new(Key::Char('a'), Modifiers::ALT | Modifiers::SHIFT),
        );

        Ok(())
    }

    #[test]
    fn parse_empty() -> Result<(), KeystrokeParseError> {
        assert_eq!(
            Keystroke::from_str(""),
            Err(KeystrokeParseError::KeyParseError {
                got: "".to_string(),
                source_error: KeyParseError::EmptyKey
            })
        );

        assert_eq!(
            Keystroke::from_str("alt-"),
            Err(KeystrokeParseError::KeyParseError {
                got: "alt-".to_string(),
                source_error: KeyParseError::EmptyKey
            })
        );

        Ok(())
    }

    #[test]
    fn parse_dash_as_key() -> Result<(), KeystrokeParseError> {
        assert_eq!(
            Keystroke::from_str("-")?,
            Keystroke::new(Key::Char('-'), Modifiers::NONE),
        );

        assert_eq!(
            Keystroke::from_str("alt-shift--")?,
            Keystroke::new(Key::Char('-'), Modifiers::ALT | Modifiers::SHIFT),
        );

        Ok(())
    }

    #[test]
    fn parse_unexpected_dash_error() -> Result<(), KeystrokeParseError> {
        assert_eq!(
            Keystroke::from_str("alt--ctrl-a"),
            Err(KeystrokeParseError::ModifiersParseError {
                got: "alt--ctrl-a".to_string(),
                source_error: ModifiersParseError::UnexpectedDash {
                    dash_placement: UnexpectedDashPlacement::Double
                }
            })
        );

        assert_eq!(
            Keystroke::from_str("alt-ctrl--a"),
            Err(KeystrokeParseError::ModifiersParseError {
                got: "alt-ctrl--a".to_string(),
                source_error: ModifiersParseError::UnexpectedDash {
                    dash_placement: UnexpectedDashPlacement::Trailing
                }
            })
        );

        Ok(())
    }
}
