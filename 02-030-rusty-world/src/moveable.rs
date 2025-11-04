//! Movement abstraction traits and supporting types.
//!
//! This module defines the [`crate::moveable::Moveable`] trait along with
//! supporting types [`crate::moveable::Direction`] and [`crate::moveable::MovementError`].
//! It provides the foundation for controlling objects that
//! can change their position or state in a directional manner.

/// Defines the direction in which a [`crate::moveable::Moveable`] entity can move.
///
/// Each variant represents a logical movement or orientation change.
///
/// # Examples
/// ```
/// use robot_control::moveable::Direction;
///
/// let forward = Direction::Forward { step: 2 };
/// ```
pub enum Direction {
    /// Move forward by a specific number of steps.
    Forward { step: i32 },

    /// Move one unit backward.
    Backwards,

    /// Move one unit to the left.
    Left,

    /// Move one unit to the right.
    Right,
}

/// Represents potential errors during movement operations.
///
/// Returned when movement cannot be completed successfully.
#[derive(Debug, PartialEq)]
pub enum MovementError {
    // pub as its return type of move_robot which is also public
    TooFar,
}

/// Trait defining directional movement behavior.
///
/// Implementors can define how a type responds to movement commands.
///
/// # Example
/// ```
/// use robot_control::moveable::{Direction, Moveable, MovementError};
///
/// struct Dummy;
///
/// impl Moveable for Dummy {
///     fn move_robot(&mut self, direction: Direction) -> Result<(), MovementError> {
///         println!("Moving {:?}", direction);
///         Ok(())
///     }
/// }
/// ```
pub trait Moveable {
    fn move_robot(&mut self, direction: Direction) -> Result<(), MovementError>;
}
