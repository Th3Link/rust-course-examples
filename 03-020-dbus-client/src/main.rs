mod position;
mod robot;
mod tui;
mod world;

use position::Position;
use robot::Robot;
use std::sync::Arc;
use tokio::sync::Mutex;
use world::{Tile, World};
const H: i32 = 20;
const W: i32 = 40;

pub async fn add_outer_wall(world: Arc<Mutex<World>>, width: i32, height: i32, wall_tile: Tile) {
    let mut world = world.lock().await;
    for x in 0..width {
        world.add_tile(Position { x, y: 0 }, wall_tile.clone()); // top
        world.add_tile(Position { x, y: height - 1 }, wall_tile.clone()); // bottom
    }

    for y in 1..(height - 1) {
        world.add_tile(Position { x: 0, y }, wall_tile.clone()); // left
        world.add_tile(Position { x: width - 1, y }, wall_tile.clone()); // right
    }
}

pub async fn handle_movement(
    command: tui::Command,
    rusty: Arc<Mutex<Robot>>,
    world: Arc<Mutex<World>>,
) {
    let mut rusty = rusty.lock().await;
    let pos = &mut rusty.position;
    let x = &mut pos.x;
    let y = &mut pos.y;

    match command {
        tui::Command::MoveUp => {
            if *y > 0 {
                *y -= 1
            }
        }
        tui::Command::MoveDown => {
            if *y < H - 1 {
                *y += 1
            }
        }
        tui::Command::MoveLeft => {
            if *x > 0 {
                *x -= 1
            }
        }
        tui::Command::MoveRight => {
            if *x < W - 1 {
                *x += 1
            }
        }
    }

    world
        .lock()
        .await
        .update_robot("Rusty", rusty.position.clone(), rusty.state_of_charge);
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let rusty = Arc::new(Mutex::new(Robot {
        name: "Rusty".to_string(),
        position: Position { x: 5i32, y: 5i32 },
        state_of_charge: 255,
    }));
    let rusty_clone = rusty.clone();
    let world = Arc::new(Mutex::new(World::default()));
    let world_clone = world.clone();
    let movement = {
        move |command: tui::Command| {
            let rusty_clone = rusty_clone.clone();
            let world_clone = world_clone.clone();
            handle_movement(command, rusty_clone, world_clone)
        }
    };
    let rusty = rusty.lock().await;
    world.lock().await.add_robot(
        rusty.name.clone(),
        rusty.position.clone(),
        rusty.state_of_charge,
    );
    drop(rusty);

    add_outer_wall(world.clone(), W, H, Tile::Wall).await;

    let _ = tokio::spawn(tui::tui(movement, 40, 20, world)).await?;
    Ok(())
}
