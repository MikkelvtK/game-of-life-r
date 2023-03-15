use std::{
    error::Error,
    io::{self, BufWriter, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo},
    queue,
    style::Print,
    terminal,
    terminal::{Clear, ClearType},
};
use game::World;

use crate::game::world_parts::Row;

// TODO: Add documentation
// TODO: Refactor game module
// TODO: Add Crossterm for terminal access
// - Give user to stop simulation gracefully
// - Center grid
// - Clear screen after printing
// TODO: Add user options

mod game;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    let mut world = World::new(60, 20);
    let screen = terminal::size()?;
    let cursor = (
        screen.0 / 2 - (world.width / 2) as u16,
        screen.1 / 2 - (world.height / 2) as u16,
    );

    let mut n = 0;
    queue!(stdout, Hide)?;
    loop {
        queue!(stdout, Clear(ClearType::All))?;

        for (i, row) in world.grid.chunks(world.width as usize).enumerate() {
            let cells = Row::new(row);
            queue!(stdout, MoveTo(cursor.0, cursor.1 + i as u16), Print(&cells))?;
        }

        stdout.flush()?;
        world.evolve();
        thread::sleep(Duration::from_secs(1));

        n += 1;
        if n == 60 {
            break;
        }
    }

    Ok(())
}
