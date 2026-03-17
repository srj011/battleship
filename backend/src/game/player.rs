use rand::prelude::*;
use serde::Serialize;
use std::collections::HashSet;

use super::board::{BOARD_SIZE, Board, Cell, FireOutcome};
use super::coord::Coord;
use super::errors::PlacementError;
use super::ship::{Direction, FLEET, Ship, ShipPlacement, ShipType};

pub struct Player {
    board: Board,
    ships: Vec<Ship>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            ships: Vec::new(),
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn place_fleet(&mut self, placements: Vec<ShipPlacement>) -> Result<(), PlacementError> {
        if placements.len() != FLEET.len() {
            return Err(PlacementError::InvalidFleetSize);
        }

        let mut added_ships = HashSet::new();
        for placement in &placements {
            // Check for duplicate ship types
            if !added_ships.insert(placement.ship_type) {
                return Err(PlacementError::ShipAlreadyPlaced);
            }
        }

        let (board, ships) = Board::place_fleet(&placements)?;
        self.board = board;
        self.ships = ships;

        Ok(())
    }

    pub fn generate_random_fleet() -> Vec<ShipPlacement> {
        let mut temp_board = Board::new();
        let mut placements: Vec<ShipPlacement> = Vec::with_capacity(FLEET.len());
        let mut rng = rand::rng();

        for ship_type in FLEET {
            let length = ship_type.length();
            loop {
                let direction = if rng.random_bool(0.5) {
                    Direction::Horizontal
                } else {
                    Direction::Vertical
                };

                let (row, col) = match direction {
                    Direction::Horizontal => (
                        rng.random_range(0..BOARD_SIZE),
                        rng.random_range(0..BOARD_SIZE - length + 1),
                    ),
                    Direction::Vertical => (
                        rng.random_range(0..BOARD_SIZE - length + 1),
                        rng.random_range(0..BOARD_SIZE),
                    ),
                };
                let start = Coord::new(row, col);

                if temp_board.place_ship(ship_type, start, direction).is_ok() {
                    placements.push(ShipPlacement {
                        ship_type,
                        start,
                        direction,
                    });
                    break;
                }
            }
        }

        placements
    }

    pub fn get_ship_mut(&mut self, ship_type: ShipType) -> &mut Ship {
        self.ships
            .iter_mut()
            .find(|s| s.ship_type() == ship_type)
            .expect("Ship not found")
    }

    pub fn fire_at(&mut self, coord: Coord) -> ShotResult {
        match self.board.fire_at(coord) {
            FireOutcome::Miss => ShotResult::Miss,
            FireOutcome::AlreadyShot => ShotResult::AlreadyShot,

            FireOutcome::Hit(ship_type) => {
                let ship = self.get_ship_mut(ship_type);
                ship.register_hit();

                if ship.is_sunk() {
                    ShotResult::Sunk
                } else {
                    ShotResult::Hit
                }
            }
        }
    }

    pub fn random_shot(&self) -> Coord {
        let mut rng = rand::rng();

        loop {
            let row = rng.random_range(0..BOARD_SIZE);
            let col = rng.random_range(0..BOARD_SIZE);
            let coord = Coord::new(row, col);

            match self.board.get_cell(coord) {
                Cell::Hit(_) | Cell::Miss => continue,
                _ => return coord,
            }
        }
    }

    pub fn has_lost(&self) -> bool {
        self.ships.iter().all(|ship| ship.is_sunk())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ShotResult {
    Hit,
    Miss,
    Sunk,
    AlreadyShot,
}
