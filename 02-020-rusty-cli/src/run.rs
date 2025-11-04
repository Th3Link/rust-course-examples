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
use crate::cli::Cli;
use crate::moveable::{Direction, Moveable};
use crate::robot::Robot;
use clap::Parser;
use env_logger::{self, Env};
use log::{debug, error, info, trace};

/// Runs a sample sequence of robot movements.
///
/// This is the main demonstration function of the crate.
/// It constructs a new [`Robot`], executes a few movement commands,
/// and prints results to the console.
///
/// # Panics
/// Panics if any `unwrap()` operation fails (used for demonstration purposes only).
pub fn run() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let cli = Cli::parse();
    let mut robot = Robot::new(cli.name); // changed to mut to use move_robot
    info!("Display output {robot}");
    info!("Debug output {robot:?}\n");

    let r = robot.move_robot(Direction::Forward { step: 4 });
    // use if-let to print the error
    if let Err(e) = r {
        error!("Movement Error: {e:?}");
    }
    debug!("{robot}");
    // use inspect_err to print the error but ignore it
    let _ = robot
        .move_robot(Direction::Forward { step: 5 })
        .inspect_err(|e| error!("Movement Error {e:?}"));

    debug!("{robot}");
    let _ = robot.move_robot(Direction::Forward { step: 2 });
    let _ = robot.move_robot(Direction::Left);
    trace!("{robot}");
    let _ = robot.move_robot(Direction::Backwards);
    info!("{robot}");
    let _ = robot.move_robot(Direction::Right);
    let _ = robot.move_robot(Direction::Right);
    info!("{robot}");
}
