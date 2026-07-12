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
    #[error("empty string")]
    EmptyString,

    #[error("invalid key, got `{0}`")]
    InvalidKey(String),

    #[error("invalid modifier, got `{0}`")]
    InvalidModifiers(String),
}

impl TryFrom<&str> for Keystroke {
    type Error = KeystrokeParseError;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        if let Some((modifiers, key)) = source.rsplit_once("-") {
            Ok(Self {
                modifiers: modifiers.parse()?,
                key: key.try_into()?,
            })
        } else {
            Ok(Self {
                modifiers: Modifiers::NONE,
                key: source.try_into()?,
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
