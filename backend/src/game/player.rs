use super::board::Board;
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

    pub fn has_lost(&self) -> bool {
        self.ships.iter().all(|ship| ship.is_sunk())
    }
}
