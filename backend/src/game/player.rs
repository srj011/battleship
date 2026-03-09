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

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ShotResult {
    Hit,
    Miss,
    Sunk,
    AlreadyShot,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::coord::Coord;
    use crate::game::ship::Direction;

    #[test]
    fn player_loses_when_all_ships_sunk() {
        let mut player = Player::new();

        player
            .place_ship(
                ShipType::Submarine,
                Coord { row: 0, col: 0 },
                Direction::Horizontal,
            )
            .unwrap();

        player.fire_at(Coord { row: 0, col: 0 });
        player.fire_at(Coord { row: 0, col: 1 });
        player.fire_at(Coord { row: 0, col: 2 });

        assert!(player.has_lost());
    }

    #[test]
    fn fire_at_returns_sunk_when_ship_destroyed() {
        let mut player = Player::new();

        player
            .place_ship(
                ShipType::PatrolBoat,
                Coord { row: 0, col: 0 },
                Direction::Horizontal,
            )
            .unwrap();

        assert!(matches!(
            player.fire_at(Coord { row: 0, col: 0 }),
            ShotResult::Hit
        ));

        assert!(matches!(
            player.fire_at(Coord { row: 0, col: 1 }),
            ShotResult::Sunk
        ));
    }
}
