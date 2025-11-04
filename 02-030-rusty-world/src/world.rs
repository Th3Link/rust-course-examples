use crate::position::Position;
use crate::robot::Robot;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Tile {
    Empty,
    Wall,
}

#[derive(Debug)]
pub struct World {
    height: u32,
    width: u32,
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
