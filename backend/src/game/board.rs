use crate::game::ship::Direction;

pub const BOARD_SIZE: usize = 10;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Ship(usize),
    Hit,
    Miss,
}

pub struct Board {
    grid: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn place_ship(
        &mut self,
        start_row: usize,
        start_col: usize,
        length: usize,
        direction: Direction,
        ship_index: usize,
    ) -> Result<Vec<(usize, usize)>, String> {
        let mut positions: Vec<(usize, usize)> = Vec::new();

        for i in 0..length {
            // Calculate coordinate for each iteration
            let (row, col) = match direction {
                Direction::Horizontal => (start_row, start_col + i),
                Direction::Vertical => (start_row + i, start_col),
            };

            // Bounds check
            if row >= BOARD_SIZE && col >= BOARD_SIZE {
                return Err("Ship out of bounds!".into());
            }

            // Overlap check
            if self.grid[row][col] != Cell::Empty {
                return Err("Ship overlaps another ship!".into());
            }

            positions.push((row, col));
        }

        for (row, col) in &positions {
            self.grid[*row][*col] = Cell::Ship(ship_index);
        }

        Ok(positions)
    }
}
