use std::fmt::Display;

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
