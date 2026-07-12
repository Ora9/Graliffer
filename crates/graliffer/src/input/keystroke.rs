use std::{
    error,
    fmt::{Display, Formatter},
};

use crate::{Key, KeyFromCrosstermError, KeyParseError, Modifiers, ModifiersParseError};

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

#[derive(Debug, thiserror::Error)]
pub enum KeystrokeParseError {
    #[error("could not parse the key part of the keystroke")]
    KeyParseError(#[source] KeyParseError),

    #[error("could not parse the modifiers part of the keystroke")]
    ModifiersParseError(#[source] ModifiersParseError),
}

impl From<KeyParseError> for KeystrokeParseError {
    fn from(value: KeyParseError) -> Self {
        KeystrokeParseError::KeyParseError(value)
    }
}

impl From<ModifiersParseError> for KeystrokeParseError {
    fn from(value: ModifiersParseError) -> Self {
        KeystrokeParseError::ModifiersParseError(value)
    }
}

impl TryFrom<&str> for Keystroke {
    type Error = KeystrokeParseError;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        if let Some((source_modifiers, source_key)) = source.rsplit_once("-") {
            Ok(Self {
                modifiers: source_modifiers
                    .try_into()
                    .map_err(|err| KeystrokeParseError::ModifiersParseError(err))?,
                key: source_key
                    .try_into()
                    .map_err(|err| KeystrokeParseError::KeyParseError(err))?,
            })
        } else {
            Ok(Self {
                modifiers: Modifiers::NONE,
                key: source
                    .try_into()
                    .map_err(|err| KeystrokeParseError::KeyParseError(err))?,
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
    fn display() {}
}
