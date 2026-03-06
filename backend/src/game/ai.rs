use rand::{prelude::*, rng};
use std::collections::HashSet;

use super::board::{BOARD_SIZE, within_bounds};
use super::coord::Coord;
use super::player::ShotResult;

pub struct AiPlayer {
    shots_taken: HashSet<Coord>,
    adjacent_targets: Vec<Coord>,
    current_hits: Vec<Coord>,
}

impl AiPlayer {
    pub fn new() -> Self {
        Self {
            shots_taken: HashSet::new(),
            adjacent_targets: Vec::new(),
            current_hits: Vec::new(),
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
        let mut rng = rng();
        loop {
            let row = rng.random_range(0..BOARD_SIZE);
            let col = rng.random_range(0..BOARD_SIZE);

            // Parity check
            if (row + col) % 2 != 0 {
                continue;
            }

            let coord = Coord::new(row, col);

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
        self.current_hits.push(coord);

        if self.current_hits.len() == 1 {
            // First hit
            self.add_adjacent_targets(coord);
        } else {
            // Subsequent hits => Orientation discovered
            self.adjacent_targets.clear();
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

    fn target_mode_shot(&self) -> Option<Coord> {
        if self.current_hits.len() < 2 {
            return None;
        }

        let first = self.current_hits[0];
        let second = self.current_hits[1];

        if first.row == second.row {
            // Horizontal
            let min_col = self.current_hits.iter().map(|c| c.col).min().unwrap();
            let max_col = self.current_hits.iter().map(|c| c.col).max().unwrap();

            if let Some(left) = Coord::new(first.row, min_col).offset(0, -1) {
                if self.is_valid_target(left) {
                    return Some(left);
                }
            }

            if let Some(right) = Coord::new(first.row, max_col).offset(0, 1) {
                if self.is_valid_target(right) {
                    return Some(right);
                }
            }
        } else {
            // Vertical
            let min_row = self.current_hits.iter().map(|c| c.row).min().unwrap();
            let max_row = self.current_hits.iter().map(|c| c.row).max().unwrap();

            if let Some(up) = Coord::new(min_row, first.col).offset(-1, 0) {
                if self.is_valid_target(up) {
                    return Some(up);
                }
            }

            if let Some(down) = Coord::new(max_row, first.col).offset(1, 0) {
                if self.is_valid_target(down) {
                    return Some(down);
                }
            }
        }

        None
    }

    // Utility methods
    fn is_valid_target(&self, coord: Coord) -> bool {
        within_bounds(coord) && !self.shots_taken.contains(&coord)
    }

    fn reset_targets(&mut self) {
        self.adjacent_targets.clear();
        self.current_hits.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_never_repeats_shots() {
        let mut ai = AiPlayer::new();
        let mut shots = HashSet::new();

        for _ in 0..50 {
            let shot = ai.next_shot();
            assert!(!shots.contains(&shot));
            shots.insert(shot);
        }
    }
}
