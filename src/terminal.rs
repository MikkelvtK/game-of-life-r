use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::io;
use std::io::Write;

use crossterm::terminal;

use crate::game::Grid;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Display<T: Write + Debug> {
    size_screen: (u16, u16),
    size_grid: (u16, u16),
    handler: T,
}

pub struct DisplayBuilder {
    screen_size: (u16, u16),
    grid_width: Option<u16>,
    grid_height: Option<u16>,
    handler: Box<dyn Write>,
}

impl<T> Display<T>
where
    T: Write + Debug,
{
    pub fn reset_cursor_position(self) -> Self {
        unimplemented!()
    }

    pub fn print_border(&self) {
        unimplemented!()
    }

    pub fn print_grid(self, grid: &Grid) -> Self {
        unimplemented!()
    }
}

impl DisplayBuilder {
    pub fn new() -> MyResult<Self> {
        let stdout = io::stdout();

        Ok(Self {
            screen_size: terminal::size()?,
            grid_width: None,
            grid_height: None,
            handler: Box::new(stdout),
        })
    }

    pub fn grid_width(self, width: u16) -> Self {
        unimplemented!()
    }
}

impl fmt::Debug for DisplayBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DisplayBuilder")
            .field("screen_size", &self.screen_size)
            .field("grid_width", &self.grid_width)
            .field("grid_height", &self.grid_height)
            .field("handler", &"Box<dyn Write>")
            .finish()
    }
}
