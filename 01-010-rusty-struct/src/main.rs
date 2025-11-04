pub struct Robot {
    name: String,
    x: i32,
    y: i32,
}

impl Robot {
    pub fn new(name: String) -> Self {
        Robot { name, x: 0, y: 0 }
    }
    pub fn hello(&self) {
        println!("Hello, my name is {}", self.name);
        println!("I am on {}/{}", self.x, self.y);
    }
}

fn main() {
    let robot = Robot::new("wall-e".to_string());
    robot.hello();
}
