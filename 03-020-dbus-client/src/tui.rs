use crate::position::Position;
use crate::world::{Tile, World};
use crossterm::{
    event::{Event, EventStream, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures_util::StreamExt;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::{select, time};

#[derive(Debug)]
pub enum Command {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

pub async fn tui<F, Fut>(
    mut movement: F,
    width: i32,
    height: i32,
    world: Arc<Mutex<World>>,
) -> io::Result<()>
where
    F: FnMut(Command) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut reader = EventStream::new();

    'draw: loop {
        let world = world.lock().await;
        terminal.draw(|f| {
            let area = f.area();
            let mut buf = String::new();
            for y in 0..height {
                for x in 0..width {
                    if world
                        .robots
                        .iter()
                        .find(|robot| robot.position == Position { x, y })
                        .is_some()
                    {
                        buf.push('R');
                        continue;
                    }
                    let tile = match world.tiles.get(&Position { x, y }) {
                        Some(Tile::Wall) => 'W',
                        Some(Tile::ChargePad) => 'C',
                        _ => ' ',
                    };
                    buf.push(tile);
                }
                buf.push('\n');
            }
            drop(world); // give back the lock asap
            let block = Block::default()
                .title("Rusty World (press q to exit)")
                .borders(Borders::ALL);
            f.render_widget(Paragraph::new(buf).block(block), area);
        })?;

        select! {
            maybe_event = reader.next() => {
                if let Some(Ok(Event::Key(key))) = maybe_event {
                    match key.code {
                        KeyCode::Up => movement(Command::MoveUp).await,
                        KeyCode::Down => movement(Command::MoveDown).await,
                        KeyCode::Left => movement(Command::MoveLeft).await,
                        KeyCode::Right => movement(Command::MoveRight).await,
                        KeyCode::Char('q') | KeyCode::Esc => break 'draw,
                        _ => {}
                    }
                }
            }
            _ = time::sleep(Duration::from_millis(50)) => {}
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
