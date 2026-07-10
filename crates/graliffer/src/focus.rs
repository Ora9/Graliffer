use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, Hash)]
pub enum PopupId {
    About,
    CommandPicker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, Hash)]
pub enum PaneId {
    Grid,
    Console,
    Stack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FocusId {
    Pane(PaneId),
    Popup(PopupId),
}

impl FocusId {
    pub fn is_pane(&self) -> bool {
        matches!(self, FocusId::Pane(_))
    }

    pub fn is_popup(&self) -> bool {
        matches!(self, FocusId::Popup(_))
    }
}

impl Display for FocusId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pane(pane) => f.write_str(&pane.to_string()),
            FocusId::Popup(popup) => f.write_str(&popup.to_string()),
        }
    }
}

impl From<PaneId> for FocusId {
    fn from(value: PaneId) -> Self {
        Self::Pane(value)
    }
}

impl From<PopupId> for FocusId {
    fn from(value: PopupId) -> Self {
        Self::Popup(value)
    }
}
