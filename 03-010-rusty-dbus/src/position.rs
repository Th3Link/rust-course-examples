//! Position wtih x and y.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Hash, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(({}/{}))", self.x, self.y)
    }
}
