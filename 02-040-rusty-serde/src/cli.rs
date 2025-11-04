//! Command line arguments and commands to start and control the robot and world.
use clap::Parser;

/// Command-line interface for the Rusty Robot world builder.
///
/// This program lets you create or configure a simulated robot world.
/// It demonstrates how to use [`clap`](https://docs.rs/clap/latest/clap/)
/// for structured command-line argument parsing.
///
/// # Example
///
/// ```bash
/// # Create a world of 40x20 tiles with the robot "Rusty"
/// cargo run -- --name rusty --width 40 --height 20
///
/// # Use short flag for name
/// cargo run -- -n boris
/// ```
///
/// # Fields
///
/// - `name`: The name of the robot to be added to the world.
/// - `height`: The vertical size (in tiles) of the generated world.
/// - `width`: The horizontal size (in tiles) of the generated world.
///
#[derive(Parser, Debug)]
#[command(
    version,
    about = "Create and manage the Rusty Robot simulation world.",
    long_about = "The Rusty Robot CLI allows you to configure and launch a simulated world \
                  containing one or more robots. You can set world dimensions and add robots by name."
)]
pub struct Cli {
    /// Name of the robot to be added to the world.
    ///
    /// Defaults to `"rusty"` if not specified.
    ///
    /// # Example
    /// ```bash
    /// cargo run -- --name rusty
    /// ```
    #[arg(short, long, default_value = "rusty")]
    pub name: String,

    /// Height (number of tiles) of the world.
    ///
    /// Defaults to `20`.
    ///
    /// # Example
    /// ```bash
    /// cargo run -- --height 25
    /// ```
    #[arg(long, default_value_t = 20)]
    pub height: u32,

    /// Width (number of tiles) of the world.
    ///
    /// Defaults to `40`.
    ///
    /// # Example
    /// ```bash
    /// cargo run -- --width 50
    /// ```
    #[arg(long, default_value_t = 40)]
    pub width: u32,
}
