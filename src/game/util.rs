use std::ops::Range;

use super::Grid;
use crate::MyResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    i: usize,
    j: usize,
}

impl Pos {
    pub fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    pub fn i(&self) -> usize {
        self.i
    }

    pub fn j(&self) -> usize {
        self.j
    }
}

pub fn num_living_neighbours(cell_pos: Pos, grid: &Grid) -> MyResult<usize> {
    if grid.contains(cell_pos) {
        let neighbours = grid.get_neighbours(cell_pos);

        return Ok(neighbours.into_iter().filter(|x| x.is_alive()).count());
    }

    Err(From::from("illegal cell position accessed"))
}

pub fn get_neighbours_range(n: usize, limit: usize) -> Range<usize> {
    let lower = if n > 0 { n - 1 } else { 0 };
    let upper = if n < limit - 1 { n + 2 } else { limit };

    lower..upper
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::Cell::*;

    #[test]
    fn test_num_living_neighbours() {
        let grid = Grid {
            data: vec![
                vec![Living(0), Dead(0), Dead(0)],
                vec![Dead(0), Living(0), Dead(0)],
                vec![Living(0), Dead(0), Dead(0)],
            ],
            width: 3,
            height: 3,
        };

        let n = num_living_neighbours(Pos::new(1, 1), &grid);
        assert!(n.is_ok());
        assert_eq!(n.unwrap(), 2);

        let n = num_living_neighbours(Pos::new(2, 2), &grid);
        assert!(n.is_ok());
        assert_eq!(n.unwrap(), 1);

        let n = num_living_neighbours(Pos::new(0, 0), &grid);
        assert!(n.is_ok());
        assert_eq!(n.unwrap(), 1);

        let n = num_living_neighbours(Pos::new(3, 0), &grid);
        assert!(n.is_err());
    }
}
