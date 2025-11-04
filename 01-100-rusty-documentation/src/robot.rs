//! Concrete robot implementation using the [`crate::moveable::Moveable`] trait.
//!
//! The [`crate::robot::Robot`] struct represents a simple 2D-positioned robot
//! that can move in four directions with boundary enforcement.

use crate::moveable::{Direction, Moveable, MovementError};
use std::fmt;

/// A simple movable robot with a name and 2D position.
///
/// The robot starts at coordinates (0, 0) and implements
/// the [`Moveable`] trait to handle directional changes.
///
/// # Example
/// ```rust
/// use robot_control::moveable::{Direction, Moveable};
/// use robot_control::robot::Robot;
///
/// let mut bot = Robot::new("Rusty".to_string());
/// bot.move_robot(Direction::Forward { step: 2 }).unwrap();
/// assert_eq!(format!("{}", bot), "(Robot name: Rusty Position: 0, 2)");
/// ```
#[derive(Debug)]
pub struct Robot {
    name: String,
    x: i32,
    y: i32,
}

impl Robot {
    /// Creates a new [`Robot`] with the given name at position `(0, 0)`.
    ///
    /// # Example
    /// ```rust
    /// let robot = Robot::new("robotname".to_string());
    /// ```
    pub fn new(name: String) -> Self {
        Robot { name, x: 0, y: 0 }
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Robot name: {} Position: {}, {})",
            self.name, self.x, self.y
        )
    }
}

impl Moveable for Robot {
    /// Moves the robot in the specified [`Direction`].
    ///
    /// # Errors
    /// Returns [`MovementError::TooFar`] if the movement exceeds limits.
    ///
    /// # Example
    /// ```rust
    /// let mut robot = Robot::new("robotname".to_string());
    /// robot.move_robot(Direction::Forward { step:3 })?;
    /// ```
    fn move_robot(&mut self, direction: Direction) -> Result<(), MovementError> {
        match direction {
            Direction::Forward { step } => {
                if step > 3 {
                    return Err(MovementError::TooFar);
                }
                self.y += step;
            }
            Direction::Backwards => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*; // imports symbols from parent module

    #[test]
    fn move_robot_too_far() {
        let mut robot = Robot::new("rusty-test".to_string());
        // assert defaults
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 0);
        // make the change
        assert_eq!(
            robot.move_robot(Direction::Forward { step: 5 }),
            Err(MovementError::TooFar)
        );
        // assert the not changed values
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 0);
    }

    #[test]
    fn move_robot_limit() {
        let mut robot = Robot::new("rusty-test".to_string());
        // assert defaults
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 0);
        // make the change
        assert!(robot.move_robot(Direction::Forward { step: 3 }).is_ok());
        // assert the not changed values
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 3);
    }

    #[test]
    fn move_robot_neg() {
        let mut robot = Robot::new("rusty-test".to_string());
        // assert defaults
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 0);
        // make the change
        assert!(robot.move_robot(Direction::Forward { step: -1 }).is_err());
        // assert the not changed values
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 3);
    }

    #[test]
    fn move_robot_neg_far() {
        let mut robot = Robot::new("rusty-test".to_string());
        // assert defaults
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 0);
        // make the change
        assert!(
            robot
                .move_robot(Direction::Forward { step: -1000 })
                .is_err()
        );
        // assert the not changed values
        assert_eq!(robot.x, 0);
        assert_eq!(robot.y, 0);
    }
}
