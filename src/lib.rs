use std::error::Error;

use crate::game::Grid;
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
    let mut grid = Grid::default();
    let screen = crossterm::terminal::size()?;

    let mut display = Display::builder()
        .screen(screen.0 as usize, screen.1 as usize)
        .grid(grid.width(), grid.height())
        .build()?;

    display.clear()?;
    display.print_grid(&grid.to_bytes())?;

    let mut n = 0;
    loop {
        grid = Grid::from(&grid)?;

        display.clear()?;
        display.print_grid(&grid.to_bytes())?;

        n += 1;
        if n == 60 {
            break;
        }
    }

    Ok(())
}
