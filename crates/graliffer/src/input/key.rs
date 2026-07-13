use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crossterm::event;

use crate::Modifiers;

/// Keyboard key of a keystroke
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
    /// Format key
    ///
    /// ```
    /// # use graliffer::Key;
    /// assert_eq!(Key::Enter.to_string(), "enter");
    /// assert_eq!(Key::F5.to_string(), "f5");
    /// assert_eq!(Key::Char('h').to_string(), "h");
    /// assert_eq!(Key::Char('H').to_string(), "h");
    /// ```
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
    #[error("expected a valid key, got an empty string")]
    EmptyKey,

    #[error("expected a valid key, got unknown key: `{got}`")]
    UnknownKey { got: String },
}

impl FromStr for Key {
    type Err = KeyParseError;

    /// Parse from `&str`
    ///
    /// ```
    /// # use graliffer::{Key, KeyParseError};
    /// # use std::str::FromStr;
    /// assert_eq!(Key::from_str("pageup")?, Key::PageUp);
    /// assert_eq!(Key::from_str("f9")?, Key::F9);
    /// assert_eq!(Key::from_str("ß")?, Key::Char('ß'));
    /// # Ok::<(), KeyParseError>(())
    /// ```
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
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
                    (None, _) => Err(KeyParseError::EmptyKey),
                    (Some(c), None) => Ok(Key::Char(c)),
                    _ => Err(KeyParseError::UnknownKey {
                        got: value.to_string(),
                    }),
                }
            }
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
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
    use crossterm::event::KeyCode;

    use super::*;

    fn assert_display(key: Key, string: &str) {
        assert_eq!(key.to_string(), string);
    }

    #[test]
    fn display_char() {
        assert_display(Key::Char('a'), "a");
        assert_display(Key::Char('Ä'), "ä");
    }

    #[test]
    fn display_special() {
        assert_display(Key::Backspace, "backspace");
        assert_display(Key::Enter, "enter");
        assert_display(Key::Left, "left");
        assert_display(Key::Right, "right");
        assert_display(Key::Up, "up");
        assert_display(Key::Down, "down");
        assert_display(Key::Home, "home");
        assert_display(Key::End, "end");
        assert_display(Key::PageUp, "pageup");
        assert_display(Key::PageDown, "pagedown");
        assert_display(Key::Tab, "tab");
        assert_display(Key::BackTab, "backtab");
        assert_display(Key::Delete, "delete");
        assert_display(Key::Insert, "insert");
        assert_display(Key::Escape, "escape");
    }

    #[test]
    fn display_fn() {
        assert_display(Key::F1, "f1");
        assert_display(Key::F2, "f2");
        assert_display(Key::F3, "f3");
        assert_display(Key::F4, "f4");
        assert_display(Key::F5, "f5");
        assert_display(Key::F6, "f6");
        assert_display(Key::F7, "f7");
        assert_display(Key::F8, "f8");
        assert_display(Key::F9, "f9");
        assert_display(Key::F10, "f10");
        assert_display(Key::F11, "f11");
        assert_display(Key::F12, "f12");
    }

    fn assert_parse(string: &str, key: Key) {
        assert_eq!(
            Key::from_str(string).expect("[test]: should have parsed a valid key"),
            key
        );
    }

    #[test]
    fn parse_char() {
        assert_parse("a", Key::Char('a'));
        assert_parse("æ", Key::Char('æ'));

        assert_parse("-", Key::Char('-'));
    }

    #[test]
    fn parse_special() {
        assert_parse("backspace", Key::Backspace);
        assert_parse("enter", Key::Enter);
        assert_parse("left", Key::Left);
        assert_parse("right", Key::Right);
        assert_parse("up", Key::Up);
        assert_parse("down", Key::Down);
        assert_parse("home", Key::Home);
        assert_parse("end", Key::End);
        assert_parse("pageup", Key::PageUp);
        assert_parse("pagedown", Key::PageDown);
        assert_parse("tab", Key::Tab);
        assert_parse("backtab", Key::BackTab);
        assert_parse("delete", Key::Delete);
        assert_parse("insert", Key::Insert);
        assert_parse("escape", Key::Escape);
    }

    #[test]
    fn parse_fn() {
        assert_parse("f1", Key::F1);
        assert_parse("f2", Key::F2);
        assert_parse("f3", Key::F3);
        assert_parse("f4", Key::F4);
        assert_parse("f5", Key::F5);
        assert_parse("f6", Key::F6);
        assert_parse("f7", Key::F7);
        assert_parse("f8", Key::F8);
        assert_parse("f9", Key::F9);
        assert_parse("f10", Key::F10);
        assert_parse("f11", Key::F11);
        assert_parse("f12", Key::F12);
    }

    #[test]
    fn parse_ignore_case() {
        assert_parse("BaCkSpAcE", Key::Backspace);
    }

    #[test]
    fn parse_empty() {
        assert_eq!(Key::from_str(""), Err(KeyParseError::EmptyKey));
    }

    #[test]
    fn parse_invalid() {
        assert_eq!(
            Key::from_str("invalid"),
            Err(KeyParseError::UnknownKey {
                got: "invalid".to_string()
            })
        );
    }

    fn assert_from_ct(ct_key: KeyCode, key: Key) {
        assert_eq!(
            Key::try_from(ct_key)
                .expect("[test]: should have been able to try_from a valid crossterm key"),
            key
        );
    }

    #[test]
    fn from_crossterm_unrepresentable() {
        assert_eq!(
            Key::try_from(KeyCode::Null),
            Err(KeyFromCrosstermError::UnrepresentableKey(KeyCode::Null))
        );
    }

    #[test]
    fn from_crossterm_char() {
        assert_from_ct(KeyCode::Char('a'), Key::Char('a'));
        assert_from_ct(KeyCode::Char('ß'), Key::Char('ß'));
    }

    #[test]
    fn from_crossterm_special() {
        assert_from_ct(KeyCode::Backspace, Key::Backspace);
        assert_from_ct(KeyCode::Enter, Key::Enter);
        assert_from_ct(KeyCode::Left, Key::Left);
        assert_from_ct(KeyCode::Right, Key::Right);
        assert_from_ct(KeyCode::Up, Key::Up);
        assert_from_ct(KeyCode::Down, Key::Down);
        assert_from_ct(KeyCode::Home, Key::Home);
        assert_from_ct(KeyCode::End, Key::End);
        assert_from_ct(KeyCode::PageUp, Key::PageUp);
        assert_from_ct(KeyCode::PageDown, Key::PageDown);
        assert_from_ct(KeyCode::Tab, Key::Tab);
        assert_from_ct(KeyCode::BackTab, Key::BackTab);
        assert_from_ct(KeyCode::Delete, Key::Delete);
        assert_from_ct(KeyCode::Insert, Key::Insert);
        assert_from_ct(KeyCode::Esc, Key::Escape);
    }

    #[test]
    fn from_crossterm_fn() {
        assert_from_ct(KeyCode::F(1), Key::F1);
        assert_from_ct(KeyCode::F(2), Key::F2);
        assert_from_ct(KeyCode::F(3), Key::F3);
        assert_from_ct(KeyCode::F(4), Key::F4);
        assert_from_ct(KeyCode::F(5), Key::F5);
        assert_from_ct(KeyCode::F(6), Key::F6);
        assert_from_ct(KeyCode::F(7), Key::F7);
        assert_from_ct(KeyCode::F(8), Key::F8);
        assert_from_ct(KeyCode::F(9), Key::F9);
        assert_from_ct(KeyCode::F(10), Key::F10);
        assert_from_ct(KeyCode::F(11), Key::F11);
        assert_from_ct(KeyCode::F(12), Key::F12);

        assert_eq!(
            Key::try_from(KeyCode::F(13)),
            Err(KeyFromCrosstermError::InvalidFnKey(13))
        );

        assert_eq!(
            Key::try_from(KeyCode::F(u8::MAX)),
            Err(KeyFromCrosstermError::InvalidFnKey(u8::MAX))
        );
    }
}
