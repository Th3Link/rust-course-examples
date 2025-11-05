use crate::position::Position;

pub struct Robot {
    pub name: String,
    pub position: Position,
    pub state_of_charge: u8,
}
