use std::fmt::Debug;
use std::io::Write;

use crate::game::Grid;

#[derive(Debug)]
pub struct Display<T: Write + Debug> {
    size_screen: (u16, u16),
    size_grid: (u16, u16),
    handler: T,
}

impl<T> Display<T>
where
    T: Write + Debug,
{
    pub fn new() -> Self {
        unimplemented!()
    }

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
