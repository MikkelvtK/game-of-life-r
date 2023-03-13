use std::error::Error;
use std::fmt;
use std::io;
use std::io::{BufWriter, Write};
use std::thread;
use std::time::Duration;

use crossterm::cursor;
use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, SetSize};
use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;

use crate::game::Row;
use crate::game::World;

// TODO: Implement the border
// TODO: Queue commands
// TODO: Fix remaining bugs

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    col: u32,
    row: u32,
}

impl Point {
    fn new_line(&mut self) {
        self.row += 1;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

pub struct Display {
    screen: Size,
    grid: Size,
    cursor: Point,
    handler: Box<dyn Write>,
}

#[derive(Debug)]
pub struct DisplayBuilder {
    screen: Option<Size>,
    grid: Option<Size>,
}

impl Display {
    pub fn builder() -> DisplayBuilder {
        DisplayBuilder::default()
    }

    pub fn clear(&mut self) -> MyResult<()> {
        self.handler.queue(Clear(ClearType::All))?;

        Ok(())
    }

    fn reset_cursor(&mut self) {
        self.cursor = DisplayBuilder::calc_default_cursor(self.grid, self.screen);
    }

    fn move_cursor(&mut self) -> MyResult<()> {
        self.handler.queue(cursor::MoveTo(
            self.cursor.col as u16,
            self.cursor.row as u16,
        ))?;

        self.handler.queue(cursor::Hide)?;

        Ok(())
    }

    pub fn print_border(&mut self) -> MyResult<()> {
        unimplemented!()
    }

    pub fn print_grid(&mut self, world: &World) -> MyResult<()> {
        self.move_cursor()?;

        for row in 0..world.height {
            let cells = world.get_row(row);
            let cells = Row::new(cells);
            queue!(
                self.handler,
                MoveTo(self.cursor.col as u16, self.cursor.row as u16),
                Print(&cells)
            )?;
            self.cursor.new_line();
        }

        self.handler.flush()?;
        thread::sleep(Duration::from_secs(1));
        self.reset_cursor();

        Ok(())
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Display")
            .field("screen", &self.screen)
            .field("grid", &self.grid)
            .field("cursor", &self.cursor)
            .field("handler", &"Box<dyn Write>")
            .finish()
    }
}

impl DisplayBuilder {
    pub fn default() -> Self {
        Self {
            screen: None,
            grid: None,
        }
    }

    fn calc_default_cursor(grid: Size, screen: Size) -> Point {
        Point {
            col: screen.width / 2 - grid.width / 2,
            row: screen.height / 2 - grid.height / 2,
        }
    }

    pub fn screen(mut self, width: u32, height: u32) -> Self {
        self.screen = Some(Size::new(width, height));
        self
    }

    pub fn grid(mut self, width: u32, height: u32) -> Self {
        self.grid = Some(Size::new(width, height));
        self
    }

    pub fn build(self) -> MyResult<Display> {
        let mut stdout = BufWriter::new(io::stdout().lock());

        let mut screen = self.screen.expect("Please set the screen size");
        let grid = self.grid.expect("Please set the grid size");

        if grid.width > screen.width {
            screen.width = grid.width + 10;
        }

        if grid.height > screen.height {
            screen.height = grid.height + 10;
        }

        if screen != self.screen.unwrap() {
            stdout.execute(SetSize(screen.width as u16, screen.height as u16))?;
        }

        Ok(Display {
            screen,
            grid,
            cursor: Self::calc_default_cursor(grid, screen),
            handler: Box::new(stdout),
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Display, Size};
    use crossterm::{cursor, terminal};

    #[test]
    fn test_display_reset_cursor() {
        println!("Screen size: {:?}", terminal::size().unwrap());
        let display = Display::builder().screen(12, 12).grid(10, 10).build();
        assert!(display.is_ok());

        assert_eq!(cursor::position().unwrap(), (1, 1));
    }

    #[test]
    fn test_display_builder() {
        // Happy flow
        let display = Display::builder().screen(40, 40).grid(50, 50).build();

        assert!(display.is_ok());
        assert_eq!(
            display.unwrap().grid,
            Size {
                width: 50,
                height: 50
            }
        );

        // Resize screen
        let display = Display::builder().screen(10, 10).grid(20, 20).build();
        assert!(display.is_ok());

        assert_eq!(
            display.unwrap().screen,
            Size {
                width: 30,
                height: 30
            }
        );
    }
}
