use std::fmt::{Display, Formatter, Write};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub control: bool,
    pub shift: bool,
    pub alt: bool,
}

impl Modifiers {
    pub const NONE: Self = Self {
        control: false,
        shift: false,
        alt: false,
    };

    pub const CONTROL: Self = Self {
        control: true,
        shift: false,
        alt: false,
    };

    pub const SHIFT: Self = Self {
        control: false,
        shift: true,
        alt: false,
    };

    pub const ALT: Self = Self {
        control: false,
        shift: false,
        alt: true,
    };
}

impl Display for Modifiers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ctrl = ("ctrl", self.control);
        let shift = ("shift", self.shift);
        let alt = ("alt", self.alt);

        let mut first = true;

        for (name, is_pressed) in [ctrl, shift, alt] {
            if is_pressed {
                if !first {
                    f.write_char('-');
                }

                first = false;

                f.write_str(name);
            }
        }

        std::fmt::Result::Ok(())
    }
}

impl From<&str> for Modifiers {
    fn from(value: &str) -> Self {
        let mut modifiers = Modifiers::NONE;

        let mut parts = value.split('-');
        while let Some(part) = parts.next() {
            match part.to_ascii_lowercase().as_str() {
                "ctrl" => modifiers.control = true,
                "shift" => modifiers.shift = true,
                "alt" => modifiers.alt = true,
                _ => {}
            }
        }

        modifiers
    }
}

impl From<KeyModifiers> for Modifiers {
    fn from(modifiers: KeyModifiers) -> Self {
        Self {
            control: modifiers.intersects(KeyModifiers::CONTROL),
            shift: modifiers.intersects(KeyModifiers::SHIFT),
            alt: modifiers.intersects(KeyModifiers::ALT),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    Esc,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Key::Char(char) => &char.to_string().to_lowercase(),
            Key::Backspace => "backspace",
            Key::Enter => "enter",
            Key::Left => "left",
            Key::Right => "right",
            Key::Up => "up",
            Key::Down => "down",
            Key::Home => "home",
            Key::End => "end",
            Key::PageUp => "pageup",
            Key::PageDown => "pagedown",
            Key::Tab => "tab",
            Key::BackTab => "backtab",
            Key::Delete => "delete",
            Key::Insert => "insert",
            Key::Esc => "esc",
            Key::F1 => "f1",
            Key::F2 => "f2",
            Key::F3 => "f3",
            Key::F4 => "f4",
            Key::F5 => "f5",
            Key::F6 => "f6",
            Key::F7 => "f7",
            Key::F8 => "f8",
            Key::F9 => "f9",
            Key::F10 => "f10",
            Key::F11 => "f11",
            Key::F12 => "f12",
        };

        f.write_str(string)
    }
}

impl TryFrom<KeyCode> for Key {
    type Error = eyre::Error;

    fn try_from(value: KeyCode) -> Result<Key, Self::Error> {
        match value {
            KeyCode::Char(char) => Ok(Key::Char(char)),
            KeyCode::Backspace => Ok(Key::Backspace),
            KeyCode::Enter => Ok(Key::Enter),
            KeyCode::Left => Ok(Key::Left),
            KeyCode::Right => Ok(Key::Right),
            KeyCode::Up => Ok(Key::Up),
            KeyCode::Down => Ok(Key::Down),
            KeyCode::Home => Ok(Key::Home),
            KeyCode::End => Ok(Key::End),
            KeyCode::PageUp => Ok(Key::PageUp),
            KeyCode::PageDown => Ok(Key::PageDown),
            KeyCode::Tab => Ok(Key::Tab),
            KeyCode::BackTab => Ok(Key::BackTab),
            KeyCode::Delete => Ok(Key::Delete),
            KeyCode::Insert => Ok(Key::Insert),
            KeyCode::Esc => Ok(Key::Esc),
            KeyCode::F(f) => match f {
                1 => Ok(Key::F1),
                2 => Ok(Key::F2),
                3 => Ok(Key::F3),
                4 => Ok(Key::F4),
                5 => Ok(Key::F5),
                6 => Ok(Key::F6),
                7 => Ok(Key::F7),
                8 => Ok(Key::F8),
                9 => Ok(Key::F9),
                10 => Ok(Key::F10),
                11 => Ok(Key::F11),
                12 => Ok(Key::F12),
                _ => Err(eyre::anyhow!(
                    "invalid function key, must be inbetween `F1` and `F12`"
                )),
            },
            _ => Err(eyre::anyhow!("unrepresentable key")),
        }
    }
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        match value {
            "backspace" => Key::Backspace,
            "enter" => Key::Enter,
            "left" => Key::Left,
            "right" => Key::Right,
            "up" => Key::Up,
            "down" => Key::Down,
            "home" => Key::Home,
            "end" => Key::End,
            "pageup" => Key::PageUp,
            "pagedown" => Key::PageDown,
            "tab" => Key::Tab,
            "backtab" => Key::BackTab,
            "delete" => Key::Delete,
            "insert" => Key::Insert,
            "esc" => Key::Esc,
            "f1" => Key::F1,
            "f2" => Key::F2,
            "f3" => Key::F3,
            "f4" => Key::F4,
            "f5" => Key::F5,
            "f6" => Key::F6,
            "f7" => Key::F7,
            "f8" => Key::F8,
            "f9" => Key::F9,
            "f10" => Key::F10,
            "f11" => Key::F11,
            "f12" => Key::F12,
            _ => Key::Char(value.chars().next().unwrap_or(' ')),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
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

    fn parse(source: String) -> eyre::Result<Self> {
        if let Some((source_modifiers, source_key)) = source.rsplit_once("-") {
            Ok(Self {
                modifiers: source_modifiers.try_into()?,
                key: source_key.into(),
            })
        } else {
            Ok(Self {
                modifiers: Modifiers::NONE,
                key: source.as_str().into(),
            })
        }
    }
}

impl TryFrom<KeyEvent> for Keystroke {
    type Error = eyre::Error;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
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
            write!(f, "{key}");
        } else {
            write!(f, "{modifiers}-{key}");
        }

        Ok(())
    }
}
