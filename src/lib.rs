use std::error::Error;
use std::ops::Index;

type MyResult<T> = Result<T, Box<dyn Error>>;

const WIDTH: usize = 60;
const HEIGHT: usize = 20;
const LIVING_CELL: u8 = b'#';

#[derive(Debug)]
enum Cell {
    Living(u8),
    Dead
}

impl Cell {
    fn is_alive(&self) -> bool {
        match self {
            Cell::Living(_) => true,
            Cell::Dead => false
        }
    }

    fn get_neighbours(&self, grid: &Grid) -> Vec<&Cell> {
        unimplemented!()
    }
}

struct Grid {
    data: Vec<Vec<Cell>>
}

impl Grid {
    fn from(prev: Grid) -> Self {
        unimplemented!()
    }
}

impl Default for Grid {
    fn default() -> Self {
        let mut grid = Vec::with_capacity(WIDTH);
    
        for _ in 0..WIDTH + 1 {
            let mut column = Vec::with_capacity(HEIGHT);

            for _ in 0..HEIGHT + 1 {
                if rand::random() {
                    column.push(Cell::Living(LIVING_CELL));
                } else {
                    column.push(Cell::Dead);
                }
            }

            grid.push(column);
        }

        Self { data: grid }
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
    println!("{:?}", grid[0][0]);

    Ok(())
}

fn num_living_neighbours(neighbours: &[Cell]) -> usize {
    neighbours.into_iter().filter(|x| x.is_alive()).count()
}