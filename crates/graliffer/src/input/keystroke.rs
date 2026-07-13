use std::{
    error,
    fmt::{Display, Formatter},
};

use crate::{Key, KeyFromCrosstermError, Modifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Keystroke {
    pub modifiers: Modifiers,
    pub key: Key,
}

impl Keystroke {
    pub fn from_key(key: Key) -> Self {
        Self {
            key,
            modifiers: Modifiers::NONE,
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum KeystrokeParseError {
    #[error("invalid key, got empty string")]
    EmptyKey,

    #[error("unknown key, got `{0}`")]
    UnknownKey(String),

    #[error("invalid modifier, got `{0}`")]
    InvalidModifiers(String),
}

impl TryFrom<&str> for Keystroke {
    type Error = KeystrokeParseError;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        if let Some((modifiers, key)) = source.rsplit_once("-") {
            Ok(Self {
                modifiers: modifiers.parse()?,
                key: key.parse()?,
            })
        } else {
            Ok(Self {
                modifiers: Modifiers::NONE,
                key: source.parse()?,
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
        assert_eq!(Keystroke::from_str(""), Err(KeystrokeParseError::EmptyKey));
        assert_eq!(
            Keystroke::from_str("alt-"),
            Err(KeystrokeParseError::EmptyKey)
        );

        Ok(())
    }

    #[test]
    fn parse_multi_dashes() -> Result<(), KeystrokeParseError> {
        assert_eq!(
            Keystroke::from_str("alt--ctrl-a")?,
            Keystroke::new(Key::Char('a'), Modifiers::ALT | Modifiers::CONTROL)
        );

        assert_eq!(
            Keystroke::from_str("alt-ctrl--a")?,
            Keystroke::new(Key::Char('a'), Modifiers::ALT | Modifiers::CONTROL)
        );

        Ok(())
    }
}
