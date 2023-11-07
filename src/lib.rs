#![cfg_attr(feature = "strict", deny(warnings))]

use std::{fmt, vec};

pub fn next_state() -> bool {
    true
}

pub struct Board {
    pub cells: Vec<(i32, i32)>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: & mut fmt::Formatter) -> fmt::Result {
        let height = 5;
        let width = 10;
        for y in 0..height {
            for x in 0..width {
                let value: &str = if self.is_alive((x, y)) { "*" } else { "." };
                write!(f, "{}", value)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new() -> Board {
        Board { cells: Vec::new() }
    }
    pub fn with_alive_cells_at(alive_cells: Vec<(i32, i32)>) -> Board {
        Board { cells: alive_cells }
    }
    pub fn is_alive(&self, cell: (i32, i32)) -> bool {
        self.cells.contains(&cell)
    }

    pub fn next_state(&mut self) -> Board {
        let mut new_board = Board::new();
        // iterate through all cells that are alive + their neighbors
        for cell in &self.cells {
            for c in Self::get_neighborhood(cell) {
                let neighbors = self.count_neighbors(&c);
                if neighbors == 3 || (neighbors == 2 && self.is_alive(c)) {
                    if !new_board.cells.contains(&c) {
                        new_board.cells.push(c);
                    }
                }
            }
        }
        new_board
    }

    fn count_neighbors(&self, cell: &(i32, i32)) -> i32 {
        let mut neighbors = 0;

        for c in Self::get_neighborhood(cell) {
            if c == *cell {
                continue;
            }
            if self.is_alive(c) {
                neighbors += 1;
            }
        }

        neighbors
    }

    fn get_neighborhood(cell: &(i32, i32)) -> Vec<(i32, i32)> {
        let mut neighbors2: Vec<(i32, i32)> = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                neighbors2.push((cell.0 + x, cell.1 + y));
            }
        }
        neighbors2
    }
}
