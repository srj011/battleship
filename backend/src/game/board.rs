use super::coord::Coord;
use super::ship::Direction;

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

    pub fn get_cell(&self, coord: Coord) -> Cell {
        self.grid[coord.row][coord.col]
    }

    pub fn place_ship(
        &mut self,
        start: Coord,
        length: usize,
        direction: Direction,
        ship_index: usize,
    ) -> Result<Vec<Coord>, String> {
        let mut positions: Vec<Coord> = Vec::new();

        for i in 0..length {
            // Calculate coordinate for each iteration
            let coord = match direction {
                Direction::Horizontal => start.offset(0, i as isize),
                Direction::Vertical => start.offset(i as isize, 0),
            }
            .ok_or_else(|| "Ship out of bounds!".to_string())?;

            // Bounds check
            if !self.within_bounds(coord) {
                return Err("Ship out of bounds!".into());
            }

            // Overlap check
            if self.grid[coord.row][coord.col] != Cell::Empty {
                return Err("Ship overlaps another ship!".into());
            }

            positions.push(coord);
        }

        for coord in &positions {
            self.grid[coord.row][coord.col] = Cell::Ship(ship_index);
        }

        Ok(positions)
    }

    pub fn fire_at(&mut self, coord: Coord) -> FireOutcome {
        match self.grid[coord.row][coord.col] {
            Cell::Empty => {
                self.grid[coord.row][coord.col] = Cell::Miss;
                FireOutcome::Miss
            }

            Cell::Ship(index) => {
                self.grid[coord.row][coord.col] = Cell::Hit;
                FireOutcome::Hit(index)
            }

            Cell::Hit | Cell::Miss => FireOutcome::AlreadyShot,
        }
    }

    pub fn within_bounds(&self, coord: Coord) -> bool {
        coord.row < BOARD_SIZE && coord.col < BOARD_SIZE
    }
}

pub enum FireOutcome {
    Hit(usize),
    Miss,
    AlreadyShot,
}
