pub struct Robot {
    name: String,
    x: i32,
    y: i32,
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
}

fn main() {
    let robot = Robot::new("wall-e".to_string());
    let mut count = 0i32;
    robot.hello(&mut count);
}
