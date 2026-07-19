use std::fmt::Display;

use ratatui::{
    style::Stylize,
    text::{Span, ToSpan},
};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum InputMode {
    #[default]
    Insert,
    Command,
}

impl Display for InputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Insert => f.write_str("insert"),
            Self::Command => f.write_str("command"),
        }
    }
}

impl InputMode {
    pub fn formated<'a>(&self) -> Span<'a> {
        use InputMode::*;
        match self {
            Command => "COMMAND".red(),
            Insert => "INSERT".to_span(),
        }
    }
}
