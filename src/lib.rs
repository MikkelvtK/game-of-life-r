use std::error::Error;
use std::io;
use std::io::{BufWriter, Write};
use std::thread;
use std::time::Duration;

use crate::game::Grid;

// TODO: Follow project naming conventions
// TODO: Look up usefull derive traits for Grid struct
// TODO: Add documentation
// TODO: Split project up in modules
// TODO: Add Crossterm for terminal access
// - Give user to stop simulation gracefully
// - Center grid
// - Clear screen after printing
// TODO: Add user options

mod game;

type MyResult<T> = Result<T, Box<dyn Error>>;

const SEPARATOR: [u8; 5] = [b'\n'; 5];

pub fn run() -> MyResult<()> {
    let mut grid = Grid::default();
    let stdout = io::stdout().lock();
    let mut handler = BufWriter::new(stdout);

    print_grid(&grid, &mut handler)?;

    let mut n = 0;
    loop {
        grid = Grid::from(&grid)?;

        print_grid(&grid, &mut handler)?;

        n += 1;
        if n == 60 {
            break;
        }
    }

    Ok(())
}

fn print_grid(grid: &Grid, mut handler: impl Write) -> MyResult<()> {
    let bytes = grid.to_bytes();
    let mut line_start = 0;

    handler.write(&SEPARATOR)?;
    while line_start != grid.width() * grid.height() {
        let next_line = line_start..line_start + grid.width();
        let bytes_written = handler.write(&bytes[next_line])?;
        handler.write(b"\n")?;
        line_start += bytes_written;
    }

    handler.flush()?;
    thread::sleep(Duration::from_secs(1));

    Ok(())
}
