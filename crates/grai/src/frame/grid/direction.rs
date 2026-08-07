use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HorizontalDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerticalDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn axis(&self) -> Axis {
        match self {
            Self::Up | Self::Down => Axis::Vertical,
            Self::Right | Self::Left => Axis::Horizontal,
        }
    }
}

impl From<VerticalDirection> for Direction {
    fn from(value: VerticalDirection) -> Self {
        match value {
            VerticalDirection::Up => Direction::Up,
            VerticalDirection::Down => Direction::Down,
        }
    }
}

impl From<HorizontalDirection> for Direction {
    fn from(value: HorizontalDirection) -> Self {
        match value {
            HorizontalDirection::Left => Direction::Left,
            HorizontalDirection::Right => Direction::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Horizontal => write!(f, "horizontal"),
            Self::Vertical => write!(f, "vertical"),
        }
    }
}
