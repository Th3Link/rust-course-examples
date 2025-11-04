# rusty-the-robot - The Adventure of rusty the robot

`rusty-the-robot` is a lightweight crate demonstrating clean modular design and Rust trait-based abstraction.  
It defines a simple **`moveable::Moveable`** trait that allows any type to implement directional movement logic, with an example implementation for a `robot::Robot`.

This makes it easy to:
- Encapsulate motion logic and reuse it across entities
- Enforce limits (e.g., prevent moving too far)
- Write clear unit tests
- Extend with new movement-capable entities beyond robots

---

## Features

- Directional movement via the `moveable::Direction` enum
- Error handling with `moveable::MovementError`
- Pluggable movement logic through the `moveable::Moveable` trait
- Example `robot::Robot` implementation with position tracking and tests

---

## Example

```rust
use robot_control::moveable::{Direction, Moveable};
use robot_control::robot::Robot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut robot = Robot::new("Rusty".to_string());
    robot.move_robot(Direction::Forward { step: 2 })?;
    robot.move_robot(Direction::Right)?;
    println!("{robot}");
    Ok(())
}
