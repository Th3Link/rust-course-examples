use crate::robot::{Direction, Robot};

pub fn run() {
    let mut robot = Robot::new("wall-e".to_string()); // changed to mut to use move_robot
    let mut count = 0i32;
    robot.hello(&mut count);
    robot.hello(&mut count);
    robot.move_robot(Direction::Forward { step: 4 });
    robot.hello(&mut count);
    robot.move_robot(Direction::Left);
    robot.hello(&mut count);
    robot.move_robot(Direction::Backwards);
    robot.hello(&mut count);
    robot.move_robot(Direction::Right);
    robot.move_robot(Direction::Right);
    robot.hello(&mut count);
}
