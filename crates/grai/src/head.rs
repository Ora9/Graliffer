use serde::{Deserialize, Serialize};

use crate::{Action, Direction, Position, PositionError, Revert, State};

#[derive(Debug, Serialize, Deserialize)]
pub struct Head {
    pub position: Position,
    pub direction: Direction,
}

impl Default for Head {
    fn default() -> Self {
        Self {
            position: Position::ORIGIN,
            direction: Direction::Right,
        }
    }
}

impl Head {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    /// Jump to another [`Position`], without changing the [`Direction`]
    ///
    /// # Examples
    /// ```
    /// # use grai::{Head, Position};
    /// let mut head = Head::default();
    /// assert_eq!(head.position.as_numeric(), (0, 0));
    ///
    /// let pos = Position::from_numeric(25, 25).unwrap();
    /// head.move_to(pos);
    /// assert_eq!(head.position, pos);
    /// ```
    pub fn move_to(&mut self, position: Position) {
        self.position = position;
    }

    /// Change the [`Direction`], without changing the [`Position`], next "step", will be in that direction
    ///
    /// # Examples
    /// ```
    /// # use grai::{Head, Direction};
    /// let mut head = Head::default();
    /// assert_eq!(head.direction, Direction::Right);
    ///
    /// head.direct_to(Direction::Up);
    /// assert_eq!(head.direction, Direction::Up);
    /// ```
    pub fn direct_to(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn step(&mut self) -> Result<(), PositionError> {
        self.position = self.position.checked_step(self.direction, 1)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum HeadAction {
    MoveTo(Position),
    DirectTo(Direction),
    Step,
}

impl Action for HeadAction {}

impl State for Head {
    type Action = HeadAction;
    type Error = ();

    fn act(&mut self, action: &HeadAction) -> Result<Revert, Self::Error> {
        match action {
            HeadAction::Step => {
                let last_pos = self.position;
                self.step();

                Ok(Revert::new(HeadAction::MoveTo(last_pos)))
            }
            HeadAction::MoveTo(position) => {
                let last_pos = self.position;
                self.move_to(*position);

                Ok(Revert::new(HeadAction::MoveTo(last_pos)))
            }
            HeadAction::DirectTo(direction) => {
                let last_dir = self.direction;
                self.direct_to(*direction);

                Ok(Revert::new(HeadAction::DirectTo(last_dir)))
            }
        }
    }
}
