//! Position wtih x and y.

use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
