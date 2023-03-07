use std::error::Error;
use std::fmt;
use std::io;
use std::io::Write;

use crossterm::cursor;
use crossterm::terminal;
use crossterm::terminal::SetSize;
use crossterm::ExecutableCommand;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    col: u16,
    row: u16,
}

impl Point {
    pub fn new(col: u16, row: u16) -> Self {
        Self { col, row }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    width: u16,
    height: u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
}

pub struct Display {
    screen: Size,
    grid: Size,
    cursor: Point,
    handler: Box<dyn Write>,
}

pub struct DisplayBuilder {
    screen: Size,
    grid_width: Option<u16>,
    grid_height: Option<u16>,
    handler: Box<dyn Write>,
}

impl Display {
    fn reset_cursor_position(&mut self) -> MyResult<()> {
        self.handler
            .execute(cursor::MoveTo(self.cursor.col, self.cursor.row))?;

        Ok(())
    }

    pub fn print_border(&mut self) -> MyResult<()> {
        self.reset_cursor_position()?;

        Ok(())
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
        let screen = terminal::size()?;

        Ok(Self {
            screen: Size::new(screen.0, screen.1),
            grid_width: None,
            grid_height: None,
            handler: Box::new(stdout),
        })
    }

    pub fn grid_width(&mut self, width: u16) -> MyResult<&mut Self> {
        if width > self.screen.width {
            self.screen.width = width + 10;
            self.handler
                .execute(SetSize(self.screen.width, self.screen.height))?;
        }

        self.grid_width = Some(width);
        Ok(self)
    }

    pub fn grid_height(&mut self, height: u16) -> MyResult<&mut Self> {
        if height > self.screen.height {
            self.screen.height = height + 10;
            self.handler
                .execute(SetSize(self.screen.width, self.screen.height))?;
        }

        self.grid_height = Some(height);
        Ok(self)
    }

    pub fn build(&mut self) -> Display {
        let size_grid = (
            self.grid_width.expect("Please set a grid_width"),
            self.grid_height.expect("Please set a grid_height"),
        );

        let col = self.screen.width / 2 - size_grid.0 / 2;
        let row = self.screen.height / 2 - size_grid.1 / 2;

        Display {
            screen: self.screen,
            grid: Size::new(size_grid.0, size_grid.1),
            cursor: Point::new(col, row),
            handler: Box::new(io::stdout()),
        }
    }
}

impl fmt::Debug for DisplayBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DisplayBuilder")
            .field("screen_size", &self.screen)
            .field("grid_width", &self.grid_width)
            .field("grid_height", &self.grid_height)
            .field("handler", &"Box<dyn Write>")
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::DisplayBuilder;
    use crossterm::{cursor, terminal, ExecutableCommand};
    use std::io;

    #[test]
    fn test_display_reset_cursor() {
        println!("Screen size: {:?}", terminal::size().unwrap());
        let mut display = DisplayBuilder::new()
            .unwrap()
            .grid_height(10)
            .unwrap()
            .grid_width(10)
            .unwrap()
            .build();

        let res = display.reset_cursor_position();
        assert!(res.is_ok());
        assert_eq!(cursor::position().unwrap(), (57, 7));
    }

    #[test]
    fn test_display_builder() {
        // Happy flow
        let builder = DisplayBuilder::new();
        assert!(builder.is_ok());
        let mut builder = builder.unwrap();

        let res = builder.grid_width(50);
        assert!(res.is_ok());

        let res = builder.grid_height(50);
        assert!(res.is_ok());

        let display = builder.build();
        assert_eq!(
            display.grid,
            Size {
                width: 50,
                height: 50
            }
        );

        // Resize screen
        let builder = DisplayBuilder::new();
        assert!(builder.is_ok());
        let mut builder = builder.unwrap();

        let res = builder.grid_width(5000);
        assert!(res.is_ok());

        let res = builder.grid_height(5000);
        assert!(res.is_ok());

        let display = builder.build();
        assert_eq!(
            display.screen,
            Size {
                width: 5010,
                height: 5010
            }
        );
    }
}
