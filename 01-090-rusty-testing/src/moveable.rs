pub enum Direction {
    // public enum, needs to be accessed when using move_robot
    Forward { step: i32 },
    Backwards,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum MovementError {
    // pub as its return type of move_robot which is also public
    TooFar,
}

pub trait Moveable {
    fn move_robot(&mut self, direction: Direction) -> Result<(), MovementError>;
}
