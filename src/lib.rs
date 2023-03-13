use std::error::Error;

use game::World;

use crate::terminal::Display;

// TODO: Add documentation
// TODO: Refactor game module
// TODO: Add Crossterm for terminal access
// - Give user to stop simulation gracefully
// - Center grid
// - Clear screen after printing
// TODO: Add user options

mod game;
mod terminal;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let world = World::new(60, 20);
    let screen = crossterm::terminal::size()?;

    let mut display = Display::builder()
        .screen(screen.0 as u32, screen.1 as u32)
        .grid(world.width, world.height)
        .build()?;

    display.clear()?;

    let mut n = 0;
    loop {
        display.clear()?;

        n += 1;
        if n == 60 {
            break;
        }
    }

    Ok(())
}
