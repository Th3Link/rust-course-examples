pub mod dbus;

#[cfg(test)]
mod tests;

use crate::moveable::{Direction, Moveable};
use crate::robot::Robot;
use crate::{moveable::MovementError, position::Position};
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
    pub fn get_robot_position(&self, name: &str) -> Option<Position> {
        self.robots
            .iter()
            .find(|robot| robot.name == name)
            .and_then(|robot| Some(robot.position.clone()))
    }

    pub fn move_robot(&mut self, name: &str, direction: Direction) -> Result<(), MovementError> {
        self.robots
            .iter_mut()
            .find(|robot| robot.name == name)
            .and_then(|robot| Some(robot.move_robot(direction).ok()?));
        Ok(())
    }
}
