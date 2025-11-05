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
use crate::position::Position;
use crate::robot::Robot;
use crate::world::dbus::WorldDbus;
use crate::world::{Tile, World};
use clap::Parser;
use env_logger::{self, Env};
use log::{debug, error, info, trace, warn};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Runs a sample sequence of robot movements.
///
/// This is the main demonstration function of the crate.
/// It constructs a new [`Robot`], executes a few movement commands,
/// and prints results to the console.
///
/// # Panics
/// Panics if any `unwrap()` operation fails (used for demonstration purposes only).
pub async fn run() {
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

    let path = "world.json";
    let world_opt: Option<World> = std::fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok());

    let mut world = match world_opt {
        Some(world) => {
            info!("Loaded world from json");
            world
        }
        None => {
            warn!("Created new world");
            let mut world = World::new(cli.height, cli.width);
            world.add_tile(Position { x: 0, y: 0 }, Tile::Wall);
            world.add_robot_existing(robot);
            world.add_robot_new(String::from("karl"));
            world
        }
    };
    let _ = world.move_robot("karl", Direction::Forward { step: 2 });

    let world = Arc::new(Mutex::new(world));
    let world_iface = WorldDbus::new(world.clone());
    let connection = zbus::Connection::session().await.unwrap();
    connection
        .object_server()
        .at("/", world_iface)
        .await
        .unwrap();
    connection.request_name("de.marc.rusty").await.unwrap();

    loop {}

    info!("{world:?}");
    let world = world.lock().await;
    let data = serde_json::to_string_pretty(&*world).unwrap();
    std::fs::write(path, data).unwrap();
}
