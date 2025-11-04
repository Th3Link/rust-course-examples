//! Main execution unit module for [`rusty-the-robot`].
//!
//! The `run` function shows how to construct and control a [`robot::Robot`].
//!
//! It demonstrates safe error handling, use of `inspect_err`,
//! and the [`moveable::Moveable`] trait in action.
//!
//! # Example
//! ```no_run
//! use robot_control::run::run;
//!
//! fn main() {
//!     run(); // prints robot movements and errors
//! }
//! ```

// note: You need to use the Moveable trait here, otherwise no access to the trait fn
use crate::moveable::{Direction, Moveable};
use crate::robot::Robot;

/// Runs a sample sequence of robot movements.
///
/// This is the main demonstration function of the crate.
/// It constructs a new [`Robot`], executes a few movement commands,
/// and prints results to the console.
///
/// # Panics
/// Panics if any `unwrap()` operation fails (used for demonstration purposes only).
pub fn run() {
    let mut robot = Robot::new("wall-e".to_string()); // changed to mut to use move_robot
    println!("Display output {robot}");
    println!("Debug output {robot:?}\n");

    let r = robot.move_robot(Direction::Forward { step: 4 });
    // use if-let to print the error
    if let Err(e) = r {
        println!("Movement Error: {e:?}");
    }
    println!("{robot}");
    // use inspect_err to print the error but ignore it
    let _ = robot
        .move_robot(Direction::Forward { step: 5 })
        .inspect_err(|e| println!("Movement Error {e:?}"));

    println!("{robot}");
    // hard panic on error, not recommended
    robot.move_robot(Direction::Forward { step: 2 }).unwrap();
    let _ = robot.move_robot(Direction::Left);
    println!("{robot}");
    let _ = robot.move_robot(Direction::Backwards);
    println!("{robot}");
    let _ = robot.move_robot(Direction::Right);
    let _ = robot.move_robot(Direction::Right);
    println!("{robot}");
}
