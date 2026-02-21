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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::coord::Coord;
    use crate::game::ship::Direction;

    #[test]
    fn place_ship_within_bounds() {
        let mut board = Board::new();

        let result = board.place_ship(Coord { row: 0, col: 0 }, 3, Direction::Horizontal, 0);

        assert!(result.is_ok());
    }

    #[test]
    fn place_ship_out_of_bounds() {
        let mut board = Board::new();

        let result = board.place_ship(
            Coord {
                row: 0,
                col: BOARD_SIZE - 1,
            },
            3,
            Direction::Horizontal,
            0,
        );

        assert!(result.is_err());
    }

    #[test]
    fn overlapping_ships_fail() {
        let mut board = Board::new();

        board
            .place_ship(Coord { row: 0, col: 0 }, 3, Direction::Horizontal, 0)
            .unwrap();

        let result = board.place_ship(Coord { row: 0, col: 1 }, 3, Direction::Vertical, 1);

        assert!(result.is_err());
    }

    #[test]
    fn fire_at_marks_hit_and_miss() {
        let mut board = Board::new();

        board
            .place_ship(Coord { row: 0, col: 0 }, 2, Direction::Horizontal, 0)
            .unwrap();

        assert!(matches!(
            board.fire_at(Coord { row: 0, col: 0 }),
            FireOutcome::Hit(_)
        ));

        assert!(matches!(
            board.fire_at(Coord { row: 5, col: 5 }),
            FireOutcome::Miss
        ));
    }

    #[test]
    fn firing_twice_returns_already_shot() {
        let mut board = Board::new();

        board.fire_at(Coord { row: 5, col: 5 });

        assert!(matches!(
            board.fire_at(Coord { row: 5, col: 5 }),
            FireOutcome::AlreadyShot
        ));
    }
}
