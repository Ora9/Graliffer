use anyhow::Context;

use crate::grid::Position;

/// A [`Head`] always has a direction
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

/// An [`Head`] has :
/// - A position
/// - A direction
#[derive(Debug, Clone, Copy, Default)]
pub struct Head {
    pub position: Position,
    pub direction: Direction,
}

impl Head {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn move_to(mut self, position: Position) {
        self.position = position;
    }

    pub fn direct_to(mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn take_step(&mut self) -> Result<(), anyhow::Error> {
        use self::Direction::*;
        let (x_offset, y_offset) = match self.direction {
            Up => (0, 1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (1, 0),
        };

        let offset = Position::from_numeric(x_offset, y_offset).unwrap();

        self.position = self.position.checked_add(offset).context("could not step into darkness, the position is invalid")?;

        Ok(())
    }
}
