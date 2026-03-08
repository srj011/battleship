use super::coord::Coord;
use super::errors::PlacementError;
use super::ship::{Direction, ShipType};

pub const BOARD_SIZE: usize = 10;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Ship(ShipType),
    Hit(ShipType),
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
        direction: Direction,
        ship_type: ShipType,
    ) -> Result<Vec<Coord>, PlacementError> {
        let mut positions: Vec<Coord> = Vec::new();
        let length = ship_type.length();

        for i in 0..length {
            // Calculate coordinate for each iteration
            let coord = match direction {
                Direction::Horizontal => start.offset(0, i as isize),
                Direction::Vertical => start.offset(i as isize, 0),
            }
            .ok_or_else(|| PlacementError::ShipOutOfBounds)?;

            // Bounds check
            if !within_bounds(coord) {
                return Err(PlacementError::ShipOutOfBounds);
            }

            // Overlap check
            if self.grid[coord.row][coord.col] != Cell::Empty {
                return Err(PlacementError::ShipOverlap);
            }

            positions.push(coord);
        }

        for coord in &positions {
            self.grid[coord.row][coord.col] = Cell::Ship(ship_type);
        }

        Ok(positions)
    }

    pub fn fire_at(&mut self, coord: Coord) -> FireOutcome {
        match self.grid[coord.row][coord.col] {
            Cell::Empty => {
                self.grid[coord.row][coord.col] = Cell::Miss;
                FireOutcome::Miss
            }

            Cell::Ship(ship_type) => {
                self.grid[coord.row][coord.col] = Cell::Hit(ship_type);
                FireOutcome::Hit(ship_type)
            }

            Cell::Hit(_) | Cell::Miss => FireOutcome::AlreadyShot,
        }
    }
}

pub fn within_bounds(coord: Coord) -> bool {
    coord.row < BOARD_SIZE && coord.col < BOARD_SIZE
}

pub enum FireOutcome {
    Hit(ShipType),
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

        let result = board.place_ship(
            Coord { row: 0, col: 0 },
            Direction::Horizontal,
            ShipType::Destroyer,
        );

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
            Direction::Horizontal,
            ShipType::Submarine,
        );

        assert!(result.is_err());
    }

    #[test]
    fn overlapping_ships_fail() {
        let mut board = Board::new();

        board
            .place_ship(
                Coord { row: 0, col: 0 },
                Direction::Horizontal,
                ShipType::Destroyer,
            )
            .unwrap();

        let result = board.place_ship(
            Coord { row: 0, col: 1 },
            Direction::Vertical,
            ShipType::Submarine,
        );

        assert!(result.is_err());
    }

    #[test]
    fn fire_at_marks_hit_and_miss() {
        let mut board = Board::new();

        board
            .place_ship(
                Coord { row: 0, col: 0 },
                Direction::Horizontal,
                ShipType::PatrolBoat,
            )
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

#[test]
fn within_bounds_detects_invalid_coordinates() {
    assert!(within_bounds(Coord { row: 0, col: 0 }));
    assert!(within_bounds(Coord { row: 9, col: 9 }));

    assert!(!within_bounds(Coord { row: 10, col: 0 }));
    assert!(!within_bounds(Coord { row: 0, col: 10 }));
}
