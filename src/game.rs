use std::{error::Error, ops::Index};

use crate::game::util::Pos;

pub mod util;

const WIDTH: usize = 60;
const HEIGHT: usize = 20;
const LIVING_CELL: u8 = b'#';
const DEAD_CELL: u8 = b' ';

type Matrix = Vec<Vec<Cell>>;
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
pub enum Cell {
    Living(u8),
    Dead(u8),
}

impl Cell {
    fn is_alive(&self) -> bool {
        match self {
            Cell::Living(_) => true,
            Cell::Dead(_) => false,
        }
    }

    fn set_state(&self, n: usize) -> Self {
        match self {
            Cell::Living(_) if n == 3 || n == 2 => Self::Living(LIVING_CELL),
            Cell::Dead(_) if n == 3 => Self::Living(LIVING_CELL),
            _ => Self::Dead(DEAD_CELL),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Grid {
    data: Matrix,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let i_range = 0..width;
        let j_range = 0..height;

        let data = i_range
            .map(|_| {
                j_range
                    .clone()
                    .map(|_| {
                        if rand::random() {
                            Cell::Living(LIVING_CELL)
                        } else {
                            Cell::Dead(DEAD_CELL)
                        }
                    })
                    .collect()
            })
            .collect::<Matrix>();

        Self {
            data,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn from(prev: &Grid) -> MyResult<Self> {
        let i_range = 0..prev.width;
        let j_range = 0..prev.height;

        let data = i_range
            .map(|i| {
                j_range
                    .clone()
                    .map(|j| {
                        let num_living_neighbours =
                            util::num_living_neighbours(Pos::new(i, j), &prev)?;

                        Ok(prev[i][j].set_state(num_living_neighbours))
                    })
                    .collect()
            })
            .collect::<MyResult<Matrix>>();

        Ok(Self {
            data: data?,
            width: prev.width,
            height: prev.height,
        })
    }

    fn get_neighbours(&self, p: Pos) -> Vec<&Cell> {
        let i_range = util::get_neighbours_range(p.i(), self.width);
        let j_range = util::get_neighbours_range(p.j(), self.height);

        i_range
            .flat_map(|i| j_range.clone().map(move |j| Pos::new(i, j)))
            .filter(|&q| q != p)
            .map(|q| &self[q.i()][q.j()])
            .collect()
    }

    fn contains(&self, cell_pos: Pos) -> bool {
        cell_pos.i() < self.width && cell_pos.j() < self.height
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let height_range = 0..self.height;
        let width_range = 0..self.width;

        height_range
            .flat_map(|j| {
                width_range.clone().map(move |i| match &self[i][j] {
                    Cell::Living(b) => *b,
                    Cell::Dead(b) => *b,
                })
            })
            .collect()
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(WIDTH, HEIGHT)
    }
}

impl Index<usize> for Grid {
    type Output = Vec<Cell>;

    fn index(&self, i: usize) -> &Vec<Cell> {
        &self.data[i]
    }
}

#[cfg(test)]
mod test {
    use super::Cell::*;
    use super::{Grid, DEAD_CELL, LIVING_CELL};

    #[test]
    fn test_grid_from() {
        let grid = Grid {
            data: vec![
                vec![Living(LIVING_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
                vec![Dead(DEAD_CELL), Living(LIVING_CELL), Dead(DEAD_CELL)],
                vec![Living(LIVING_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
            ],
            width: 4,
            height: 3,
        };

        let should_be = Grid {
            data: vec![
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
                vec![Living(LIVING_CELL), Living(LIVING_CELL), Dead(DEAD_CELL)],
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
            ],
            width: 4,
            height: 3,
        };

        let next = Grid::from(&grid);
        assert!(next.is_ok());
        assert_eq!(next.unwrap(), should_be);

        let grid = Grid {
            data: vec![
                vec![Living(LIVING_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
                vec![Dead(DEAD_CELL), Living(LIVING_CELL), Dead(DEAD_CELL)],
                vec![Living(LIVING_CELL), Dead(DEAD_CELL), Living(LIVING_CELL)],
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Living(LIVING_CELL)],
            ],
            width: 4,
            height: 3,
        };

        let should_be = Grid {
            data: vec![
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
                vec![Living(LIVING_CELL), Living(LIVING_CELL), Dead(DEAD_CELL)],
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Living(LIVING_CELL)],
                vec![Dead(DEAD_CELL), Living(LIVING_CELL), Dead(DEAD_CELL)],
            ],
            width: 4,
            height: 3,
        };

        let next = Grid::from(&grid);
        assert!(next.is_ok());
        assert_eq!(next.unwrap(), should_be);
    }
}
