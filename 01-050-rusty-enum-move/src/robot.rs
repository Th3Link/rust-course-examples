pub struct Robot {
    name: String,
    x: i32,
    y: i32,
}

pub enum Direction {
    Forward { step: i32 },
    Backwards,
    Left,
    Right,
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
    }

    pub fn move_robot(&mut self, direction: Direction) {
        match direction {
            Direction::Forward { step } => {
                self.y += step;
            }
            Direction::Backwards => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}
