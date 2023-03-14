use std::fmt;

pub struct Row<'a> {
    cells: &'a [Cell],
}

impl<'a> Row<'a> {
    pub fn new(cells: &'a [Cell]) -> Row<'a> {
        Self { cells }
    }
}

impl<'a> fmt::Display for Row<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in self.cells {
            let symbol = if cell.is_alive() { '#' } else { ' ' };
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
    pub fn is_alive(&self) -> bool {
        match self {
            Cell::Alive => true,
            Cell::Dead => false,
        }
    }

    pub fn set_state(&self, n: u8) -> Self {
        match (self, n) {
            (Self::Alive, 3) => Self::Alive,
            (Self::Alive, 2) => Self::Alive,
            (Self::Dead, 3) => Self::Alive,
            (_, _) => Self::Dead,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cell::{Alive, Dead};

    #[test]
    fn test_cell_is_alive() {
        let a = Alive;
        assert_eq!(a.is_alive(), true);

        let b = Dead;
        assert_eq!(b.is_alive(), false);
    }
}
