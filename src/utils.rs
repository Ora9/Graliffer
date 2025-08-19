use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use strum_macros::AsRefStr;

#[derive(Serialize, Deserialize, AsRefStr, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Direction::{}", self.as_ref())
    }
}
