use crate::position::Position;
use crate::robot::Robot;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum Tile {
    Empty,
    Wall,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    height: u32,
    width: u32,
    #[serde_as(as = "Vec<(_, _)>")]
    tiles: HashMap<Position, Tile>,
    robots: Vec<Robot>,
}

impl World {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            height,
            width,
            tiles: HashMap::new(),
            robots: Vec::new(),
        }
    }

    pub fn add_tile(&mut self, position: Position, tile: Tile) {
        self.tiles.insert(position, tile);
    }
    pub fn add_robot_new(&mut self, name: String) {
        self.robots.push(Robot::new(name));
    }
    pub fn add_robot_existing(&mut self, robot: Robot) {
        self.robots.push(robot);
    }
}
