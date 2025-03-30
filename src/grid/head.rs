use anyhow::Context;

use crate::{artifact::{Action, Artifact}, grid::Position, Frame};

/// A [`Head`] always has a direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// An head travels in a [`Grid`] reading [`Operand`] and [`Opcodes`]
///
/// an `Head` has :
/// - A [`Position`] in a [`Grid`]
/// - A [`Direction`],
///
/// # Example
/// ```
/// # use graliffer::grid::{Head, Direction, Position};
/// let pos1 = Position::from_numeric(25, 25).unwrap();
/// let pos2 = Position::from_numeric(26, 24).unwrap();
/// let direction = Direction::Down;
///
/// let mut head = Head::new(pos1, Direction::Right);
///
/// head.take_step();
/// head.direct_to(Direction::Up);
/// head.take_step();
/// assert_eq!(head.position, pos2);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct Head {
    pub position: Position,
    pub direction: Direction,
}

impl Head {
    /// Return an `Head` given a [`Position`] and a [`Direction`]
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
    /// # use graliffer::grid::{Head, Direction, Position};
    /// let mut head = Head::default();
    /// assert_eq!(head.position.as_numeric(), (0, 0));
    ///
    /// let pos = Position::from_numeric(25, 25).unwrap();
    /// head.move_to(pos);
    /// assert_eq!(head.position, pos);
    /// ```
    fn move_to(&mut self, position: Position) {
        self.position = position;
    }

    /// Change the [`Direction`], without changing the [`Position`], next "step", will be in that set direction
    ///
    /// # Examples
    /// ```
    /// # use graliffer::grid::{Head, Direction};
    /// let mut head = Head::default();
    /// assert_eq!(head.direction, Direction::Right);
    ///
    /// head.direct_to(Direction::Up);
    /// assert_eq!(head.direction, Direction::Up);
    /// ```
    fn direct_to(&mut self, direction: Direction) {
        self.direction = direction;
    }

    /// Take one step in the [`Head`]'s [`Direction`]
    ///
    /// # Errors
    /// Returns an error if [`Head`] could not step further in that direction,
    /// because it could not go outside of the [`Grid`]'s limits
    ///
    /// # Examples
    /// ```
    /// # use graliffer::grid::{Head, Direction, Position};
    /// let pos = Position::from_numeric(25, 25).unwrap();
    /// let mut head = Head::new(pos, Direction::Right);
    ///
    /// head.take_step();
    /// head.direct_to(Direction::Down);
    /// head.take_step();
    /// head.direct_to(Direction::Left);
    /// head.take_step();
    /// head.direct_to(Direction::Up);
    /// head.take_step();
    /// assert_eq!(head.position, pos);
    /// ```
    fn take_step(&mut self) -> Result<(), anyhow::Error> {
        use self::Direction::*;
        self.position = match self.direction {
            Up => self.position.checked_decrement_y(1),
            Right => self.position.checked_increment_x(1),
            Down => self.position.checked_increment_y(1),
            Left => self.position.checked_decrement_x(1),
        }.context("could not step into darkness, the position is invalid")?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum HeadAction {
    MoveTo(Position),
    DirectTo(Direction),
    TakeStep()
}

impl Action for HeadAction {
    fn act(&self, frame: &mut Frame) -> Artifact {
        match self {
            Self::MoveTo(position) => {
                let old_position = frame.head.position;

                frame.head.move_to(*position);

                Artifact::from_redo_undo(
                    Box::new(self.to_owned()),
                    Box::new(Self::MoveTo(old_position))
                )
            }
            Self::DirectTo(direction) => {
                let old_direction = frame.head.direction;

                frame.head.direct_to(*direction);

                Artifact::from_redo_undo(
                    Box::new(self.to_owned()),
                    Box::new(Self::DirectTo(old_direction))
                )
            }
            Self::TakeStep() => {
                let old_position = frame.head.position;

                frame.head.take_step();

                Artifact::from_redo_undo(
                    Box::new(self.to_owned()),
                    Box::new(Self::MoveTo(old_position))
                )
            }
        }
    }
}
