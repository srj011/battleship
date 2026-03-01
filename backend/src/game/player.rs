use super::board::{BOARD_SIZE, Board, Cell, FireOutcome};
use super::coord::Coord;
use super::ship::{Direction, Ship};

use rand::prelude::*;

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
        start: Coord,
        length: usize,
        direction: Direction,
    ) -> Result<(), String> {
        let ship_index = self.ships.len();

        let positions = self
            .board
            .place_ship(start, length, direction, ship_index)?;

        self.ships.push(Ship::new(positions));

        Ok(())
    }

    pub fn place_random_ships(&mut self, ship_lengths: &[usize]) {
        let mut rng = rand::rng();

        for &length in ship_lengths {
            loop {
                let row = rng.random_range(0..BOARD_SIZE);
                let col = rng.random_range(0..BOARD_SIZE);
                let coord = Coord { row, col };

                let direction = if rng.random_bool(0.5) {
                    Direction::Horizontal
                } else {
                    Direction::Vertical
                };

                if self.place_ship(coord, length, direction).is_ok() {
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

#[derive(Clone, Copy, PartialEq, Eq)]
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
            .place_ship(Coord { row: 0, col: 0 }, 2, Direction::Horizontal)
            .unwrap();

        player.fire_at(Coord { row: 0, col: 0 });
        player.fire_at(Coord { row: 0, col: 1 });

        assert!(player.has_lost());
    }

    #[test]
    fn fire_at_returns_sunk_when_ship_destroyed() {
        let mut player = Player::new();

        player
            .place_ship(Coord { row: 0, col: 0 }, 2, Direction::Horizontal)
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
