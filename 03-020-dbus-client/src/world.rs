use crate::position::Position;
use crate::robot::Robot;

#[derive(Clone)]
pub enum Tile {
    Empty,
    Wall,
    ChargePad,
}

#[derive(Default)]
pub struct World {
    pub tiles: std::collections::HashMap<Position, Tile>,
    pub robots: Vec<Robot>,
}

impl World {
    pub fn add_tile(&mut self, position: Position, tile: Tile) {
        self.tiles.insert(position, tile);
    }

    pub fn add_robot(&mut self, name: String, position: Position, state_of_charge: u8) {
        self.robots.push(Robot {
            name,
            position,
            state_of_charge,
        });
    }

    pub fn update_robot(&mut self, name: &str, position: Position, state_of_charge: u8) {
        match self.robots.iter_mut().find(|robot| robot.name == name) {
            Some(robot) => {
                robot.position = position;
                robot.state_of_charge = state_of_charge;
            }
            _ => {}
        }
    }
}
