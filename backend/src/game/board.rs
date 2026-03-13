use super::coord::Coord;
use super::errors::PlacementError;
use super::ship::{Direction, Ship, ShipPlacement, ShipType};

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
        self.grid[coord.row()][coord.col()]
    }

    pub fn ship_positions(
        ship_type: ShipType,
        start: Coord,
        direction: Direction,
    ) -> Result<Vec<Coord>, PlacementError> {
        let length = ship_type.length();
        let mut positions = Vec::with_capacity(length);

        for i in 0..length {
            let coord = match direction {
                Direction::Horizontal => start.offset(0, i as isize),
                Direction::Vertical => start.offset(i as isize, 0),
            }
            .ok_or(PlacementError::ShipOutOfBounds)?;

            positions.push(coord);
        }
        Ok(positions)
    }

    pub fn validate_positions(&self, positions: &[Coord]) -> Result<(), PlacementError> {
        for &coord in positions {
            if !within_bounds(coord) {
                return Err(PlacementError::ShipOutOfBounds);
            }

            if self.get_cell(coord) != Cell::Empty {
                return Err(PlacementError::ShipOverlap);
            }
        }
        Ok(())
    }

    pub fn place_ship(
        &mut self,
        ship_type: ShipType,
        start: Coord,
        direction: Direction,
    ) -> Result<Vec<Coord>, PlacementError> {
        let positions = Self::ship_positions(ship_type, start, direction)?;
        self.validate_positions(&positions)?;
        for &coord in &positions {
            self.grid[coord.row()][coord.col()] = Cell::Ship(ship_type);
        }

        Ok(positions)
    }

    pub fn place_fleet(placements: &[ShipPlacement]) -> Result<(Board, Vec<Ship>), PlacementError> {
        // Returns a new board with ships placed if valid
        let mut board = Board::new();
        let mut ships = Vec::with_capacity(placements.len());

        for placement in placements {
            let positions =
                board.place_ship(placement.ship_type, placement.start, placement.direction)?;
            ships.push(Ship::new(placement.ship_type, positions));
        }
        Ok((board, ships))
    }

    pub fn fire_at(&mut self, coord: Coord) -> FireOutcome {
        match self.get_cell(coord) {
            Cell::Empty => {
                self.grid[coord.row()][coord.col()] = Cell::Miss;
                FireOutcome::Miss
            }

            Cell::Ship(ship_type) => {
                self.grid[coord.row()][coord.col()] = Cell::Hit(ship_type);
                FireOutcome::Hit(ship_type)
            }

            Cell::Hit(_) | Cell::Miss => FireOutcome::AlreadyShot,
        }
    }
}

pub fn within_bounds(coord: Coord) -> bool {
    coord.row() < BOARD_SIZE && coord.col() < BOARD_SIZE
}

pub enum FireOutcome {
    Hit(ShipType),
    Miss,
    AlreadyShot,
}
