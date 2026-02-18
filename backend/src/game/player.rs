use crate::game::board::FireOutcome;

use super::board::{Board, Cell};
use super::ship::{Direction, Ship};

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
        start_row: usize,
        start_col: usize,
        length: usize,
        direction: Direction,
    ) -> Result<(), String> {
        let ship_index = self.ships.len();

        let positions = self
            .board
            .place_ship(start_row, start_col, length, direction, ship_index)?;

        self.ships.push(Ship::new(positions));

        Ok(())
    }

    pub fn fire_at(&mut self, row: usize, col: usize) -> ShotResult {
        match self.board.fire_at(row, col) {
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

    pub fn has_lost(&self) -> bool {
        self.ships.iter().all(|ship| ship.is_sunk())
    }
}

pub enum ShotResult {
    Hit,
    Miss,
    Sunk,
    AlreadyShot,
}
