// note: You need to use the Moveable trait here, otherwise no access to the trait fn
use crate::moveable::{Direction, Moveable};
use crate::robot::Robot;

pub fn run() {
    let mut robot = Robot::new("wall-e".to_string()); // changed to mut to use move_robot
    let mut count = 0i32;
    robot.hello(&mut count);
    robot.hello(&mut count);
    let r = robot.move_robot(Direction::Forward { step: 4 });
    // use if-let to print the error
    if let Err(e) = r {
        println!("Movement Error: {e:?}");
    }
    robot.hello(&mut count);
    // use inspect_err to print the error but ignore it
    let _ = robot
        .move_robot(Direction::Forward { step: 5 })
        .inspect_err(|e| println!("Movement Error {e:?}"));

    robot.hello(&mut count);
    // hard panic on error, not recommended
    robot.move_robot(Direction::Forward { step: 2 }).unwrap();
    let _ = robot.move_robot(Direction::Left);
    robot.hello(&mut count);
    let _ = robot.move_robot(Direction::Backwards);
    robot.hello(&mut count);
    let _ = robot.move_robot(Direction::Right);
    let _ = robot.move_robot(Direction::Right);
    robot.hello(&mut count);
}
