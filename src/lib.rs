use std::error::Error;
use std::io::{Write, BufWriter};
use std::io;
use std::ops::{Index, Range};
use std::thread;
use std::time::Duration;

// TODO: Follow project naming conventions
// TODO: Look up usefull derive traits for Grid struct
// TODO: Add documentation
// TODO: Split project up in modules
// TODO: Add Crossterm for terminal access
    // - Give user to stop simulation gracefully
    // - Center grid
    // - Clear screen after printing 
// TODO: Add user options

type MyResult<T> = Result<T, Box<dyn Error>>;
type Matrix = Vec<Vec<Cell>>;

const WIDTH: usize = 60;
const HEIGHT: usize = 20;
const LIVING_CELL: u8 = b'#';
const DEAD_CELL: u8 = b' ';
const SEPARATOR: [u8; 5] = [b'\n'; 5];

#[derive(Debug, PartialEq)]
enum Cell {
    Living(u8),
    Dead(u8)
}

impl Cell {
    fn is_alive(&self) -> bool {
        match self {
            Cell::Living(_) => true,
            Cell::Dead(_) => false
        }
    }

    fn set_state(&self, n: usize) -> Self {
        match self {
            Cell::Living(_) if n == 3 || n ==2 => Self::Living(LIVING_CELL),
            Cell::Dead(_) if n == 3 => Self::Living(LIVING_CELL),
            _ => Self::Dead(DEAD_CELL)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos(usize, usize);

#[derive(Debug, PartialEq)]
struct Grid {
    data: Matrix,
    width: usize,
    height: usize
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let i_range = 0..width;
        let j_range = 0..height;

        let data = i_range
            .map(|_| j_range.clone()
                .map(|_| {
                    if rand::random() {
                        Cell::Living(LIVING_CELL)
                    } else {
                        Cell::Dead(DEAD_CELL)
                    }
                }).collect()
            ).collect::<Matrix>();

        Self { data, width, height }
    }

    fn from(prev: &Grid) -> MyResult<Self> {
        let i_range = 0..prev.width;
        let j_range = 0..prev.height;

        let data = i_range
            .map(|i| j_range.clone()
                .map(|j| {
                    let num_living_neighbours = num_living_neighbours(Pos(i, j), &prev)?;

                    Ok(prev[i][j].set_state(num_living_neighbours))
                }).collect()
            ).collect::<MyResult<Matrix>>();

        Ok(Self { data: data?, width: prev.width, height: prev.height })
    }

    fn get_neighbours(&self, p: Pos) -> Vec<&Cell>{
        let m_range = get_neighbours_range(p.0, self.width);
        let n_range = get_neighbours_range(p.1, self.height);

        m_range
            .flat_map(|m| n_range.clone().map(move |n| Pos(m, n)))
            .filter(|&q| q != p)
            .map(|q| &self[q.0][q.1])
            .collect()
    }

    fn contains(&self, cell_pos: Pos) -> bool {
        cell_pos.0 < self.width && cell_pos.1 < self.height
    }

    fn to_bytes(&self) -> Vec<u8> {
        let height_range = 0..self.height;
        let width_range = 0..self.width;

        height_range
            .flat_map(|n| width_range.clone().map(move |m| {
                match &self[m][n] {
                    Cell::Living(b) => *b,
                    Cell::Dead(b) => *b
                }
            }))
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
            break
        }
    }

    Ok(())
}

fn print_grid(grid: &Grid, mut handler: impl Write) -> MyResult<()> {
    let bytes = grid.to_bytes();
    let mut line_start = 0;

    handler.write(&SEPARATOR)?;
    while line_start != grid.width * grid.height {
        let next_line = line_start..line_start + grid.width;
        let bytes_written = handler.write(&bytes[next_line])?;
        handler.write(b"\n")?;
        line_start += bytes_written;
    }

    handler.flush()?;
    thread::sleep(Duration::from_secs(1));

    Ok(())
}

fn num_living_neighbours(cell_pos: Pos, grid: &Grid) -> MyResult<usize> {
    if grid.contains(cell_pos) {
        let neighbours = grid.get_neighbours(cell_pos);

        return Ok(neighbours.into_iter().filter(|x| x.is_alive()).count());
    }

    Err(From::from("illegal cell position accessed"))
}

fn get_neighbours_range(n: usize, limit: usize) -> Range<usize> {
    let lower = if n > 0 { n - 1 } else { 0 };
    let upper = if n < limit - 1 { n + 2 } else { limit };

    lower..upper
}

#[cfg(test)]
mod test {
    use crate::{LIVING_CELL, DEAD_CELL};

    use super::num_living_neighbours;
    use super::{Grid, Pos};
    use super::Cell::*;

    #[test]
    fn test_num_living_neighbours() {
        let grid = Grid {
            data: vec![
                vec![Living(0), Dead(0), Dead(0)],
                vec![Dead(0), Living(0), Dead(0)],
                vec![Living(0), Dead(0), Dead(0)]
            ],
            width: 3,
            height: 3
        };

        let n = num_living_neighbours(Pos(1, 1), &grid);
        assert!(n.is_ok());
        assert_eq!(n.unwrap(), 2);

        let n = num_living_neighbours(Pos(2, 2), &grid);
        assert!(n.is_ok());
        assert_eq!(n.unwrap(), 1);

        let n = num_living_neighbours(Pos(0, 0), &grid);
        assert!(n.is_ok());
        assert_eq!(n.unwrap(), 1);

        let n = num_living_neighbours(Pos(3, 0), &grid);
        assert!(n.is_err());
    }

    #[test]
    fn test_grid_from() {
        let grid = Grid {
            data: vec![
                vec![Living(LIVING_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
                vec![Dead(DEAD_CELL), Living(LIVING_CELL), Dead(DEAD_CELL)],
                vec![Living(LIVING_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)]
            ],
            width: 4,
            height: 3
        };

        let should_be = Grid {
            data: vec![
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
                vec![Living(LIVING_CELL), Living(LIVING_CELL), Dead(DEAD_CELL)],
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)]
            ],
            width: 4,
            height: 3
        };

        let next = Grid::from(&grid);
        assert!(next.is_ok());
        assert_eq!(next.unwrap(), should_be);

        let grid = Grid {
            data: vec![
                vec![Living(LIVING_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
                vec![Dead(DEAD_CELL), Living(LIVING_CELL), Dead(DEAD_CELL)],
                vec![Living(LIVING_CELL), Dead(DEAD_CELL), Living(LIVING_CELL)],
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Living(LIVING_CELL)]
            ],
            width: 4,
            height: 3
        };

        let should_be = Grid {
            data: vec![
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL), Dead(DEAD_CELL)],
                vec![Living(LIVING_CELL), Living(LIVING_CELL), Dead(DEAD_CELL)],
                vec![Dead(DEAD_CELL), Dead(DEAD_CELL),Living(LIVING_CELL)],
                vec![Dead(DEAD_CELL), Living(LIVING_CELL), Dead(DEAD_CELL)]
            ],
            width: 4,
            height: 3
        };

        let next = Grid::from(&grid);
        assert!(next.is_ok());
        assert_eq!(next.unwrap(), should_be);
    }
}