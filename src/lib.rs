use std::error::Error;
use std::ops::Index;

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

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<Cell>>,
    width: usize,
    height: usize
}

impl Grid {
    fn from(prev: Grid) -> Self {
        unimplemented!()
    }

    fn get_neighbours(&self, coord: (usize, usize)) -> Vec<&Cell> {
        unimplemented!()
    }

    // TODO: Create contains individual cell function

    // TODO: Move to separate module
}

impl Default for Grid {

    // TODO: Create new function, move this algo and call new from here
    fn default() -> Self {
        let mut grid = Vec::with_capacity(WIDTH);
    
        for _ in 0..WIDTH + 1 {
            let mut column = Vec::with_capacity(HEIGHT);

            for _ in 0..HEIGHT + 1 {
                if rand::random() {
                    column.push(Cell::Living(LIVING_CELL));
                } else {
                    column.push(Cell::Dead(DEAD_CELL));
                }
            }

            grid.push(column);
        }

        Self { data: grid, width: WIDTH, height: HEIGHT }
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

// TODO: Expand to retrieve neighbours and then count them
fn num_living_neighbours(neighbours: &[Cell]) -> MyResult<usize> {
    if neighbours.is_empty() {
        return Err(From::from("no neighbouring cells were found"));
    }

    Ok(neighbours.into_iter().filter(|x| x.is_alive()).count())
}

#[cfg(test)]
mod test {
    use super::num_living_neighbours;
    use super::Cell::*;

    #[test]
    fn test_num_living_neighbours() {
        let neighbours = vec![Living(35), Living(35), Dead(32)];
        assert_eq!(num_living_neighbours(&neighbours).unwrap(), 2);

        let neighbours = vec![Dead(35), Dead(35), Dead(32)];
        assert_eq!(num_living_neighbours(&neighbours).unwrap(), 0);

        let neighbours = vec![];
        assert!(num_living_neighbours(&neighbours).is_err());
    }
}