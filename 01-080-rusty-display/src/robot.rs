use crate::moveable::{Direction, Moveable, MovementError};
use std::fmt;

#[derive(Debug)]
pub struct Robot {
    name: String,
    x: i32,
    y: i32,
}

impl Robot {
    pub fn new(name: String) -> Self {
        Robot { name, x: 0, y: 0 }
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Robot name: {} Position: {}, {})",
            self.name, self.x, self.y
        )
    }
}

impl Moveable for Robot {
    fn move_robot(&mut self, direction: Direction) -> Result<(), MovementError> {
        match direction {
            Direction::Forward { step } => {
                if step > 3 {
                    return Err(MovementError::TooFar);
                }
                self.y += step;
            }
            Direction::Backwards => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
        Ok(())
    }
}
