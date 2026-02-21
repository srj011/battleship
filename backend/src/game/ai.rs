use rand::prelude::*;
use std::collections::HashSet;

use super::board::BOARD_SIZE;
use super::coord::Coord;
use super::player::ShotResult;

pub struct AIPlayer {
    rng: ThreadRng,
    shots_taken: HashSet<Coord>,
    adjacent_targets: Vec<Coord>,
    front: Option<Coord>,
    back: Option<Coord>,
}

impl AIPlayer {
    pub fn new() -> Self {
        Self {
            rng: rand::rng(),
            shots_taken: HashSet::new(),
            adjacent_targets: Vec::new(),
            front: None,
            back: None,
        }
    }

    pub fn next_shot(&mut self) -> Coord {
        // Target mode
        // After second and subsequent hits -> Follow orientation(target mode)
        if let Some(target) = self.target_mode_shot() {
            self.shots_taken.insert(target);
            return target;
        }

        // After first hit -> Find orientation of ship
        while let Some(target) = self.adjacent_targets.pop() {
            if !self.shots_taken.contains(&target) {
                self.shots_taken.insert(target);
                return target;
            }
        }

        // Hunt mode (parity based random)
        loop {
            let row = self.rng.random_range(0..BOARD_SIZE);
            let col = self.rng.random_range(0..BOARD_SIZE);

            // Parity check
            if (row + col) % 2 != 0 {
                continue;
            }

            let coord = Coord { row, col };

            if !self.shots_taken.contains(&coord) {
                self.shots_taken.insert(coord);
                return coord;
            }
        }
    }

    pub fn process_result(&mut self, coord: Coord, result: ShotResult) {
        match result {
            ShotResult::Hit => self.handle_hit(coord),
            ShotResult::Sunk => self.reset_targets(),
            _ => {}
        }
    }

    fn handle_hit(&mut self, coord: Coord) {
        match (self.front, self.back) {
            // First hit
            (None, None) => {
                self.front = Some(coord);
                self.back = Some(coord);
                self.add_adjacent_targets(coord);
            }

            // Second hit
            (Some(front), Some(back)) if front == back => {
                self.back = Some(coord);
                self.adjacent_targets.clear();
            }

            // 2+ hits
            _ => {
                self.update_endpoints(coord);
            }
        }
    }

    fn add_adjacent_targets(&mut self, coord: Coord) {
        let directions = [
            (1, 0),  // Down
            (-1, 0), // Up
            (0, 1),  // Right
            (0, -1), // Left
        ];

        for (dr, dc) in directions {
            if let Some(next) = coord.offset(dr, dc) {
                if self.is_valid_target(next) {
                    self.adjacent_targets.push(next);
                }
            }
        }
    }

    fn update_endpoints(&mut self, coord: Coord) {
        let front = self.front.unwrap();
        let back = self.back.unwrap();

        if front.row == back.row {
            // Horizontal
            if coord.col < front.col {
                self.front = Some(coord);
            } else if coord.col > back.col {
                self.back = Some(coord);
            }
        } else {
            // Vertical
            if coord.row < front.row {
                self.front = Some(coord);
            } else if coord.row > back.row {
                self.back = Some(coord);
            }
        }
    }

    fn target_mode_shot(&self) -> Option<Coord> {
        let (front, back) = match (self.front, self.back) {
            (Some(f), Some(b)) if f != b => (f, b),
            _ => return None,
        };

        if front.row == back.row {
            // Horizontal
            if let Some(target) = back.offset(0, 1) {
                // Extend right
                if self.is_valid_target(target) {
                    return Some(target);
                }
            }
            if let Some(target) = front.offset(0, -1) {
                // Extend left
                if self.is_valid_target(target) {
                    return Some(target);
                }
            }
        } else {
            // Vertical
            if let Some(target) = back.offset(1, 0) {
                // Extend down
                if self.is_valid_target(target) {
                    return Some(target);
                }
            }
            if let Some(target) = front.offset(-1, 0) {
                if self.is_valid_target(target) {
                    return Some(target);
                }
            }
        }
        None
    }

    // Utility methods
    fn is_valid_target(&self, target: Coord) -> bool {
        target.row < BOARD_SIZE && target.col < BOARD_SIZE && !self.shots_taken.contains(&target)
    }

    fn reset_targets(&mut self) {
        self.adjacent_targets.clear();
        self.front = None;
        self.back = None;
    }
}
