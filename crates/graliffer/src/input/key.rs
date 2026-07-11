use std::fmt::{Display, Formatter};

use crossterm::event;

use crate::Modifiers;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Escape,
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
            Key::Escape => "escape",
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

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum KeyParseError {
    #[error("empty string")]
    EmptyString,

    #[error("invalid key, got `{0}`")]
    InvalidKey(String),
}

impl TryFrom<&str> for Key {
    type Error = KeyParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "backspace" => Ok(Key::Backspace),
            "enter" => Ok(Key::Enter),
            "left" => Ok(Key::Left),
            "right" => Ok(Key::Right),
            "up" => Ok(Key::Up),
            "down" => Ok(Key::Down),
            "home" => Ok(Key::Home),
            "end" => Ok(Key::End),
            "pageup" => Ok(Key::PageUp),
            "pagedown" => Ok(Key::PageDown),
            "tab" => Ok(Key::Tab),
            "backtab" => Ok(Key::BackTab),
            "delete" => Ok(Key::Delete),
            "insert" => Ok(Key::Insert),
            "escape" => Ok(Key::Escape),
            "f1" => Ok(Key::F1),
            "f2" => Ok(Key::F2),
            "f3" => Ok(Key::F3),
            "f4" => Ok(Key::F4),
            "f5" => Ok(Key::F5),
            "f6" => Ok(Key::F6),
            "f7" => Ok(Key::F7),
            "f8" => Ok(Key::F8),
            "f9" => Ok(Key::F9),
            "f10" => Ok(Key::F10),
            "f11" => Ok(Key::F11),
            "f12" => Ok(Key::F12),
            _ => {
                let mut chars = value.chars();
                match (chars.next(), chars.next()) {
                    (None, _) => Err(KeyParseError::EmptyString),
                    (Some(c), None) => Ok(Key::Char(c)),
                    _ => Err(KeyParseError::InvalidKey(value.to_string())),
                }
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum KeyFromCrosstermError {
    #[error("invalid function key, must be inbetween 1 and 12, got {0}")]
    InvalidFnKey(u8),

    #[error("unrepresentable key, got {0:?}")]
    UnrepresentableKey(crossterm::event::KeyCode),
}

impl TryFrom<crossterm::event::KeyCode> for Key {
    type Error = KeyFromCrosstermError;

    fn try_from(value: crossterm::event::KeyCode) -> Result<Key, Self::Error> {
        use crossterm::event::KeyCode;
        match value {
            KeyCode::Char(char) => Ok(Key::Char(char.to_ascii_lowercase())),
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
            KeyCode::Esc => Ok(Key::Escape),
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
                _ => Err(KeyFromCrosstermError::InvalidFnKey(f)),
            },
            _ => Err(KeyFromCrosstermError::UnrepresentableKey(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_char() {
        assert_eq!(Key::Char('a').to_string(), "a");
        assert_eq!(Key::Char('Ä').to_string(), "ä");
    }

    #[test]
    fn display_special() {
        assert_eq!(Key::Backspace.to_string(), "backspace");
        assert_eq!(Key::Enter.to_string(), "enter");
        assert_eq!(Key::Left.to_string(), "left");
        assert_eq!(Key::Right.to_string(), "right");
        assert_eq!(Key::Up.to_string(), "up");
        assert_eq!(Key::Down.to_string(), "down");
        assert_eq!(Key::Home.to_string(), "home");
        assert_eq!(Key::End.to_string(), "end");
        assert_eq!(Key::PageUp.to_string(), "pageup");
        assert_eq!(Key::PageDown.to_string(), "pagedown");
        assert_eq!(Key::Tab.to_string(), "tab");
        assert_eq!(Key::BackTab.to_string(), "backtab");
        assert_eq!(Key::Delete.to_string(), "delete");
        assert_eq!(Key::Insert.to_string(), "insert");
        assert_eq!(Key::Escape.to_string(), "escape");
    }

    #[test]
    fn display_fn() {
        assert_eq!(Key::F1.to_string(), "f1");
        assert_eq!(Key::F2.to_string(), "f2");
        assert_eq!(Key::F3.to_string(), "f3");
        assert_eq!(Key::F4.to_string(), "f4");
        assert_eq!(Key::F5.to_string(), "f5");
        assert_eq!(Key::F6.to_string(), "f6");
        assert_eq!(Key::F7.to_string(), "f7");
        assert_eq!(Key::F8.to_string(), "f8");
        assert_eq!(Key::F9.to_string(), "f9");
        assert_eq!(Key::F10.to_string(), "f10");
        assert_eq!(Key::F11.to_string(), "f11");
        assert_eq!(Key::F12.to_string(), "f12");
    }

    #[test]
    fn parse_char() -> Result<(), KeyParseError> {
        assert_eq!(Key::try_from("a")?, Key::Char('a'));
        assert_eq!(Key::try_from("æ")?, Key::Char('æ'));

        Ok(())
    }

    #[test]
    fn parse_error() {
        assert_eq!(
            Key::try_from("invalid"),
            Err(KeyParseError::InvalidKey(String::from("invalid")))
        );

        assert_eq!(Key::try_from(""), Err(KeyParseError::EmptyString));
    }

    #[test]
    fn parse_special() -> Result<(), KeyParseError> {
        assert_eq!(Key::try_from("backspace")?, Key::Backspace);
        assert_eq!(Key::try_from("enter")?, Key::Enter);
        assert_eq!(Key::try_from("left")?, Key::Left);
        assert_eq!(Key::try_from("right")?, Key::Right);
        assert_eq!(Key::try_from("up")?, Key::Up);
        assert_eq!(Key::try_from("down")?, Key::Down);
        assert_eq!(Key::try_from("home")?, Key::Home);
        assert_eq!(Key::try_from("end")?, Key::End);
        assert_eq!(Key::try_from("pageup")?, Key::PageUp);
        assert_eq!(Key::try_from("pagedown")?, Key::PageDown);
        assert_eq!(Key::try_from("tab")?, Key::Tab);
        assert_eq!(Key::try_from("backtab")?, Key::BackTab);
        assert_eq!(Key::try_from("delete")?, Key::Delete);
        assert_eq!(Key::try_from("insert")?, Key::Insert);
        assert_eq!(Key::try_from("escape")?, Key::Escape);

        Ok(())
    }

    #[test]
    fn parse_fn() -> Result<(), KeyParseError> {
        assert_eq!(Key::try_from("f1")?, Key::F1);
        assert_eq!(Key::try_from("f2")?, Key::F2);
        assert_eq!(Key::try_from("f3")?, Key::F3);
        assert_eq!(Key::try_from("f4")?, Key::F4);
        assert_eq!(Key::try_from("f5")?, Key::F5);
        assert_eq!(Key::try_from("f6")?, Key::F6);
        assert_eq!(Key::try_from("f7")?, Key::F7);
        assert_eq!(Key::try_from("f8")?, Key::F8);
        assert_eq!(Key::try_from("f9")?, Key::F9);
        assert_eq!(Key::try_from("f10")?, Key::F10);
        assert_eq!(Key::try_from("f11")?, Key::F11);
        assert_eq!(Key::try_from("f12")?, Key::F12);

        Ok(())
    }
}
