use rand::prelude::*;
use serde::Serialize;

use super::board::{BOARD_SIZE, Board, Cell, FireOutcome};
use super::coord::Coord;
use super::ship::{Direction, FLEET, Ship, ShipType};

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

    pub fn place_ship(
        &mut self,
        ship_type: ShipType,
        start: Coord,
        direction: Direction,
    ) -> Result<(), String> {
        if self.ships.iter().any(|ship| ship.ship_type() == ship_type) {
            return Err("Ship already placed".to_string());
        }

        let ship_index = self.ships.len();
        let length = ship_type.length();

        let positions = self
            .board
            .place_ship(start, length, direction, ship_index)?;

        self.ships.push(Ship::new(ship_type, positions));

        Ok(())
    }

    pub fn place_random_ships(&mut self) {
        let mut rng = rand::rng();

        for ship in FLEET {
            loop {
                let row = rng.random_range(0..BOARD_SIZE);
                let col = rng.random_range(0..BOARD_SIZE);
                let coord = Coord { row, col };

                let direction = if rng.random_bool(0.5) {
                    Direction::Horizontal
                } else {
                    Direction::Vertical
                };

                if self.place_ship(ship, coord, direction).is_ok() {
                    break;
                }
            }
        }
    }

    pub fn fire_at(&mut self, coord: Coord) -> ShotResult {
        match self.board.fire_at(coord) {
            FireOutcome::Miss => ShotResult::Miss,
            FireOutcome::AlreadyShot => ShotResult::AlreadyShot,

            FireOutcome::Hit(index) => {
                let ship = &mut self.ships[index];
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
            let coord = Coord { row, col };

            match self.board.get_cell(coord) {
                Cell::Hit | Cell::Miss => continue,
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
