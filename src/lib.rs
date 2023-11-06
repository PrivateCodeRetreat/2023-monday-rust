#![cfg_attr(feature = "strict", deny(warnings))]

pub fn next_state() -> bool {
    true
}

pub struct Board {
    pub cells: Vec<(i32, i32)>,
}

impl Board {
    pub fn new() -> Board {
        Board { cells: Vec::new() }
    }
    pub fn is_alive(&self, cell: (i32, i32)) -> bool {
        self.cells.contains(&cell)
    }

    pub fn next_state(&mut self) -> Board {
        let mut new_board = Board::new();
        for cell in &self.cells {
            let mut neighbors = 0;
            for x in -1..2 {
                for y in -1..2 {
                    if x == 0 && y == 0 {
                        continue;
                    }
                    if self.is_alive((cell.0 + x, cell.1 + y)) {
                        neighbors += 1;
                    }
                }
            }
            if neighbors == 2 || neighbors == 3 {
                new_board.cells.push(*cell);
            }
        }
        new_board
    }
}