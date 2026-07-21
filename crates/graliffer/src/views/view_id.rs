use std::{
    fmt::Display,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaneId(String);

impl From<&str> for PaneId {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl Display for PaneId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopupId(String);

impl From<&str> for PopupId {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl Display for PopupId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewId {
    Pane(PaneId),
    Popup(PopupId),
}

impl Display for ViewId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pane(pane) => write!(f, "{pane}"),
            Self::Popup(popup) => write!(f, "{popup}"),
        }
    }
}

impl ViewId {
    pub fn is_pane(&self) -> bool {
        matches!(self, Self::Pane(_))
    }

    pub fn is_popup(&self) -> bool {
        matches!(self, Self::Popup(_))
    }
}

impl From<PaneId> for ViewId {
    fn from(value: PaneId) -> Self {
        Self::Pane(value)
    }
}

impl From<PopupId> for ViewId {
    fn from(value: PopupId) -> Self {
        Self::Popup(value)
    }
}
