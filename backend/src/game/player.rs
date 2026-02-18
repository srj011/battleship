use super::board::{BOARD_SIZE, Board, Cell};
use super::ship::{Direction, Ship};
use crate::game::board::FireOutcome;

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

    pub fn place_random_ships(&mut self, ship_lengths: &[usize]) {
        let mut rng = rand::rng();

        for &length in ship_lengths {
            loop {
                let row = rng.random_range(0..BOARD_SIZE);
                let col = rng.random_range(0..BOARD_SIZE);

                let direction = if rng.random_bool(0.5) {
                    Direction::Horizontal
                } else {
                    Direction::Vertical
                };

                if self.place_ship(row, col, length, direction).is_ok() {
                    break;
                }
            }
        }
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

    pub fn random_shot(&self) -> (usize, usize) {
        let mut rng = rand::rng();

        loop {
            let row = rng.random_range(0..BOARD_SIZE);
            let col = rng.random_range(0..BOARD_SIZE);

            match self.board.get_cell(row, col) {
                Cell::Hit | Cell::Miss => continue,
                _ => return (row, col),
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
