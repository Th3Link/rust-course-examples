use crate::position::Position;
use crate::world::{Tile, World};
use std::sync::Arc;
use tokio::sync::Mutex;
use zbus::interface;
use zbus::object_server::SignalEmitter;

pub struct WorldDbus {
    world: Arc<Mutex<World>>,
}

impl WorldDbus {
    pub fn new(world: Arc<Mutex<World>>) -> Self {
        Self { world }
    }
}

#[interface(name = "org.example.something")]
impl WorldDbus {
    async fn add_robot(&self, robot_name: &str) {
        self.world
            .lock()
            .await
            .add_robot_new(robot_name.to_string());
    }

    async fn add_tile(&self, tile_name: &str, x: i32, y: i32) -> zbus::fdo::Result<()> {
        let position = Position::new(x, y);
        let tile: Tile = serde_json::from_str(tile_name).map_err(|e| {
            zbus::fdo::Error::UnknownObject(format!("Tile name {tile_name} is invalid ({e})"))
        })?;
        self.world.lock().await.add_tile(position, tile);
        Ok(())
    }

    async fn get_robot(&self, robot_name: &str) -> zbus::fdo::Result<(i32, i32)> {
        let pos = self
            .world
            .lock()
            .await
            .get_robot_position(robot_name)
            .ok_or(zbus::fdo::Error::Failed(format!(
                "Robot {robot_name} not found"
            )))?;
        Ok((pos.x, pos.y))
    }

    #[zbus(property)]
    async fn height(&self) -> u32 {
        self.world.lock().await.height
    }

    #[zbus(property)]
    async fn width(&self) -> u32 {
        self.world.lock().await.width
    }
    #[zbus(signal)]
    async fn robot(emitter: &SignalEmitter<'_>, name: String, x: i32, y: i32) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn tile(emitter: &SignalEmitter<'_>, name: String, x: i32, y: i32) -> zbus::Result<()>;
}
