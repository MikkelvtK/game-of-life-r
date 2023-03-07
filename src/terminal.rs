use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;

use crossterm::terminal;
use crossterm::terminal::SetSize;
use crossterm::ExecutableCommand;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct Display {
    size_screen: (u16, u16),
    size_grid: (u16, u16),
    handler: Box<dyn Write>,
}

pub struct DisplayBuilder {
    screen_size: (u16, u16),
    grid_width: Option<u16>,
    grid_height: Option<u16>,
    handler: Box<dyn Write>,
}

impl Display {
    pub fn reset_cursor_position(self) -> Self {
        unimplemented!()
    }

    pub fn print_border(&self) {
        unimplemented!()
    }

    pub fn print_grid(self, grid: &[u8]) -> Self {
        unimplemented!()
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

    pub fn grid_width(mut self, width: u16) -> MyResult<()> {
        if width > self.screen_size.0 {
            self.screen_size.0 = width + 10;
            self.handler
                .execute(SetSize(self.screen_size.0, self.screen_size.1))?;
        }

        self.grid_width = Some(width);
        Ok(())
    }

    pub fn grid_height(mut self, height: u16) -> MyResult<()> {
        if height > self.screen_size.1 {
            self.screen_size.1 = height + 10;
            self.handler
                .execute(SetSize(self.screen_size.0, self.screen_size.1))?;
        }

        self.grid_height = Some(height);
        Ok(())
    }

    pub fn build(self) -> Display {
        let size_grid = (
            self.grid_width.expect("Please set a grid_width"),
            self.grid_height.expect("Please set a grid_height"),
        );

        Display {
            size_screen: self.screen_size,
            size_grid,
            handler: self.handler,
        }
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
