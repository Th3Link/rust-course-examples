pub struct Robot {
    name: String,
    x: i32,
    y: i32,
}

pub enum Direction {
    // public enum, needs to be accessed when using move_robot
    Forward { step: i32 },
    Backwards,
    Left,
    Right,
}

#[derive(Debug)]
pub enum MovementError {
    // pub as its return type of move_robot which is also public
    TooFar,
}

impl Robot {
    pub fn new(name: String) -> Self {
        Robot { name, x: 0, y: 0 }
    }
    pub fn hello(&self, count: &mut i32) {
        println!("Hello, my name is {}", self.name);
        println!("I am on {}/{}", self.x, self.y);
        *count += 1;
        println!("You called me {count} times");
        println!("-------------------------------");
    }

    pub fn move_robot(&mut self, direction: Direction) -> Result<(), MovementError> {
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
