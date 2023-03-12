use core::fmt;
use std::{error::Error, ops::Index};

use crate::game::util::Pos;

pub mod util;
mod world_parts;

const WIDTH: usize = 60;
const HEIGHT: usize = 20;
const LIVING_CELL: u8 = b'#';
const DEAD_CELL: u8 = b' ';

type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct Row<'a> {
    cells: Vec<&'a Cell>,
}

impl<'a> fmt::Display for Row<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in self.cells {
            let symbol = if *cell == Cell::Alive { '#' } else { ' ' };
            write!(f, "{}", symbol)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    fn is_alive(&self) -> bool {
        match self {
            Cell::Alive => true,
            Cell::Dead => false,
        }
    }

    fn set_state(self, n: usize) -> Self {
        match self {
            Cell::Alive if n == 3 || n == 2 => Self::Alive,
            Cell::Dead if n == 3 => Self::Dead,
            _ => Self::Dead,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    grid: Vec<Cell>,
    width: u32,
    height: u32,
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        // The new function will create a new grid and set the initial state of
        // all cells to dead. After which it will use an iterator to set random
        // cells to Alive.
        let mut grid = vec![Cell::Dead; (width * height) as usize];

        grid.iter().map(|cell| {
            if rand::random() {
                *cell = Cell::Alive;
            }
        });

        Self {
            grid,
            width,
            height,
        }
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.height + col) as usize
    }

    pub fn evolve(&mut self) {
        // The function creates a clone of the old grid and then sets the state of each
        // new cell based on the circumstances of the old grid. Finally it sets the grid
        // field of World to the new grid.
        let mut new_grid = self.grid.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let num_neighbours = self.get_num_alive_neighbours(idx);
                new_grid[idx] = self.grid[idx].set_state(num_neighbours);
            }
        }

        self.grid = new_grid;
    }

    fn get_num_alive_neighbours(&self, index: usize) -> usize {
        unimplemented!()
    }

    fn contains(&self, cell_pos: Pos) -> bool {
        cell_pos.i < self.width && cell_pos.j < self.height
    }
}

impl Index<usize> for World {
    type Output = Cell;

    fn index(&self, i: usize) -> &Cell {
        &self.grid[i]
    }
}
