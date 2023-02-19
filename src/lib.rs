use std::error::Error;
use std::ops::{Index, Range};

// TODO: Follow project naming conventions
// TODO: Look up usefull derive traits for Grid struct

type MyResult<T> = Result<T, Box<dyn Error>>;

const WIDTH: usize = 60;
const HEIGHT: usize = 20;
const LIVING_CELL: u8 = b'#';
const DEAD_CELL: u8 = b' ';

#[derive(Debug)]
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos(usize, usize);

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<Cell>>,
    width: usize,
    height: usize
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let mut grid = Vec::with_capacity(width);
        
        for _ in 0..width + 1 {
            let mut column = Vec::with_capacity(height);

            for _ in 0..height + 1 {
                if rand::random() {
                    column.push(Cell::Living(LIVING_CELL));
                } else {
                    column.push(Cell::Dead(DEAD_CELL));
                }
            }

            grid.push(column);
        }

        Self { data: grid, width, height }
    }

    fn from(prev: Grid) -> MyResult<Self> {
        let i_range = 0..prev.width;
        let j_range = 0..prev.height;

        let data = i_range
            .map(|i| j_range.clone().map(|j| {
                let num_living = num_living_neighbours(Pos(i, j), &prev)?;

                match &prev[i][j] {
                    Cell::Living(b) => {
                        if num_living == 2 || num_living == 3 {
                            Ok(Cell::Living(*b))
                        } else {
                            Ok(Cell::Dead(DEAD_CELL))
                        }
                    },
                    Cell::Dead(b) => {
                        if num_living == 3 {
                            Ok(Cell::Living(*b))
                        } else {
                            Ok(Cell::Dead(*b))
                        }
                    }
                }
            })
            .collect::<MyResult<Vec<Cell>>>()
        ).collect::<MyResult<Vec<Vec<Cell>>>>();

        Ok(Grid { data: data?, width: prev.width, height: prev.height })
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

    // TODO: Move to separate module
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
    
    let grid = Grid::default();
    println!("First: {:?}", grid[0][0]);

    let second = Grid::from(grid)?;
    println!("Second: {:?}", second[0][0]);


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
}