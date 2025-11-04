mod robot;

use crate::robot::Robot;

fn main() {
    let robot = Robot::new("wall-e".to_string());
    let mut count = 0i32;
    robot.hello(&mut count);
    robot.hello(&mut count);
}
