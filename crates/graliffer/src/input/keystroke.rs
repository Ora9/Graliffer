use std::fmt::{Display, Formatter};

use crate::{Key, Modifiers};

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

impl TryFrom<&str> for Keystroke {
    type Error = eyre::Error;

    fn try_from(source: &str) -> Result<Self, Self::Error> {
        if let Some((source_modifiers, source_key)) = source.rsplit_once("-") {
            Ok(Self {
                modifiers: source_modifiers.try_into()?,
                key: source_key.try_into()?,
            })
        } else {
            Ok(Self {
                modifiers: Modifiers::NONE,
                key: source.try_into()?,
            })
        }
    }
}

impl TryFrom<crossterm::event::KeyEvent> for Keystroke {
    type Error = eyre::Error;

    fn try_from(event: crossterm::event::KeyEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            key: event.code.try_into()?,
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
