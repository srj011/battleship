use rand::prelude::*;
use serde::Serialize;
use std::collections::HashSet;

use super::board::{BOARD_SIZE, Board, Cell, FireOutcome, within_bounds};
use super::coord::Coord;
use super::errors::PlacementError;
use super::ship::{Direction, FLEET, Ship, ShipPlacement, ShipType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ShotResult {
    Hit,
    Miss,
    Sunk,
    AlreadyShot,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct DamageInfo {
    pub ship_type: ShipType,
    pub total: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShotOutcome {
    pub result: ShotResult,
    pub blocked: Vec<Coord>,
    pub sunk_ship: Option<ShipType>,
    pub damage: Option<DamageInfo>,
}

impl ShotOutcome {
    pub fn new(
        result: ShotResult,
        blocked: Vec<Coord>,
        sunk_ship: Option<ShipType>,
        damage: Option<DamageInfo>,
    ) -> Self {
        Self {
            result,
            blocked,
            sunk_ship,
            damage,
        }
    }
}

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

    pub fn ships(&self) -> &[Ship] {
        &self.ships
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
            let length = ship_type.length() as usize;
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

    pub fn fire_at(&mut self, coord: Coord) -> ShotOutcome {
        match self.board.fire_at(coord) {
            FireOutcome::Miss => ShotOutcome::new(ShotResult::Miss, Vec::new(), None, None),
            FireOutcome::AlreadyShot => {
                ShotOutcome::new(ShotResult::AlreadyShot, Vec::new(), None, None)
            }
            FireOutcome::Hit(ship_type) => {
                let (sunk_positions, hits) = {
                    let ship = self.get_ship_mut(ship_type);
                    ship.register_hit();
                    let hits = ship.hits();

                    if ship.is_sunk() {
                        (Some(ship.positions().clone()), hits)
                    } else {
                        (None, hits)
                    }
                };

                let damage = Some(DamageInfo {
                    ship_type,
                    total: hits,
                });

                if let Some(positions) = sunk_positions {
                    let blocked = self.mark_adjacent_as_blocked(&positions);
                    ShotOutcome::new(ShotResult::Sunk, blocked, Some(ship_type), damage)
                } else {
                    ShotOutcome::new(ShotResult::Hit, Vec::new(), None, damage)
                }
            }
        }
    }

    pub fn has_lost(&self) -> bool {
        self.ships.iter().all(|ship| ship.is_sunk())
    }

    pub fn mark_adjacent_as_blocked(&mut self, positions: &[Coord]) -> Vec<Coord> {
        let mut blocked: Vec<Coord> = Vec::new();
        for &coord in positions {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if let Some(adj) = coord.offset(dr, dc) {
                        if !within_bounds(adj) {
                            continue;
                        }
                        if let Cell::Empty = self.board.get_cell(adj) {
                            self.board.mark_blocked(adj);
                            blocked.push(adj);
                        }
                    }
                }
            }
        }
        blocked
    }
}
