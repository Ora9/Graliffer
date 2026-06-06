use crate::{Action, Direction, Position, PositionError, State};

#[derive(Debug)]
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

    fn act(&mut self, action: &HeadAction) {
        match action {
            HeadAction::Step => {
                self.step();
            }
            HeadAction::MoveTo(position) => {
                self.move_to(*position);
            }
            HeadAction::DirectTo(direction) => {
                self.direct_to(*direction);
            }
        };
    }
}
