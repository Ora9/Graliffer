#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewId {
    Grid,
    Stack,
    Console,

    Picker,
    About,
}

impl std::fmt::Display for ViewId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Grid => f.write_str("Grid"),
            Self::Stack => f.write_str("Stack"),
            Self::Console => f.write_str("Console"),

            Self::Picker => f.write_str("Picker"),
            Self::About => f.write_str("About"),
        }
    }
}
