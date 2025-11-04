#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::panic)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::private_intra_doc_links)]
#![allow(rustdoc::private_doc_tests)]
#![warn(rustdoc::invalid_codeblock_attributes)]
#![warn(rustdoc::invalid_html_tags)]
#![warn(rustdoc::invalid_rust_codeblocks)]
#![warn(rustdoc::bare_urls)]
#![warn(rustdoc::unescaped_backticks)]

/// Core movement abstraction (directions, trait, and errors).
mod moveable;

/// Implementation of [`moveable::Moveable`] for a 2D robot.
///
/// This module is crate-private by default, as it is primarily used
/// internally and through the public `run` interface.
mod robot; // private is good enough / only used within the crate

/// Public example entry point demonstrating crate functionality.
///
/// This function is exposed publicly so that a binary crate
/// (e.g., `main.rs`) can call it directly.
pub mod run; // needs to be pub visible because main is in a different crate
