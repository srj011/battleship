use std::collections::HashSet;
use rand::{prelude::*, rng};

use super::ship::{ShipPlacement, ShipType, FLEET};
use super::board::BOARD_SIZE;
use super::coord::Coord;
use super::player::{ShotOutcome, ShotResult};

#[derive(Debug, Clone, Copy)]
pub enum AiCell {
    Unknown,
    Hit,
    Miss,
    Blocked,
}

#[derive(Debug)]
struct KnowledgeState {
    active_hits: HashSet<Coord>,
    sunk_cells: HashSet<Coord>,
    misses: HashSet<Coord>,
    blocked: HashSet<Coord>,
    remaining_ships: HashSet<ShipType>
}

impl KnowledgeState {
    fn new() -> Self {
        Self {
            active_hits: HashSet::new(),
            sunk_cells: HashSet::new(),
            misses: HashSet::new(),
            blocked: HashSet::new(),
            remaining_ships: FLEET.into_iter().collect()
        }
    }
}

pub struct AiPlayer {
    knowledge: KnowledgeState
}

impl AiPlayer {
    pub fn new() -> Self {
        Self {
            knowledge: KnowledgeState::new()
        }
    }

    pub fn next_shot(&mut self) -> Coord {
        let placements = self.filter_placements(self.generate_placements());
        let heatmap = self.probability_map(&placements);

        let max = *heatmap
            .iter()
            .flatten()
            .max()
            .expect("heatmap empty");
        let mut candidates = Vec::new();

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if heatmap[row][col] == max {
                    candidates.push(Coord::new(row, col));
                }
            }
        }

        *candidates.choose(&mut rng()).expect("ai next_shot fn failed")
    }

    pub fn process_result(&mut self, coord: Coord, outcome: &ShotOutcome) {
        match outcome.result {
            ShotResult::Hit => {
                self.knowledge.active_hits.insert(coord);
            }
            ShotResult::Miss => {
                self.knowledge.misses.insert(coord);
            }
            ShotResult::Sunk => {
                self.knowledge.active_hits.insert(coord);
                self.knowledge.sunk_cells.extend(self.knowledge.active_hits.drain());

                if let Some(sunk_ship) = outcome.sunk_ship {
                    self.knowledge.remaining_ships.remove(&sunk_ship);
                }
            }
            ShotResult::AlreadyShot => {}
        }

        self.knowledge.blocked.extend(outcome.blocked.iter().copied());
    }

    fn generate_placements(&self) -> Vec<ShipPlacement> {
        let mut placements = Vec::new();

        for &ship in &self.knowledge.remaining_ships {
            for row in 0..BOARD_SIZE {
                for col in 0..BOARD_SIZE {
                    for &(dr, dc) in &[(1,0), (0,1)] {
                        if dr == 1 && row + ship.length() as usize > BOARD_SIZE {
                            continue;
                        }

                        if dc == 1 && col + ship.length() as usize > BOARD_SIZE {
                            continue;
                        }

                        placements.push(ShipPlacement {
                            ship_type: ship,
                            start: Coord::new(row, col),
                            direction: if dr == 1 {
                                super::ship::Direction::Vertical
                            } else {
                                super::ship::Direction::Horizontal
                            }
                        });
                    }
                }
            }
        }
        placements
    }

    fn filter_placements(&self, placements: Vec<ShipPlacement>) -> Vec<ShipPlacement> {
        placements.into_iter().filter(|p| {
            let coords = p.coords();

            !coords.iter().any(|c|
                self.knowledge.misses.contains(c)
                || self.knowledge.blocked.contains(c)
                || self.knowledge.sunk_cells.contains(c)
            )
            &&
            self.knowledge.active_hits.iter().all(|h|
             coords.contains(h)
            )
        }).collect()
    }

    fn probability_map(&self, placements: &[ShipPlacement]) -> [[u32; BOARD_SIZE]; BOARD_SIZE] {
        let mut map = [[0; BOARD_SIZE]; BOARD_SIZE];

        for p in placements {
            for coord in &p.coords() {
                if self.knowledge.active_hits.contains(&coord)
                || self.knowledge.sunk_cells.contains(&coord)
                || self.knowledge.misses.contains(&coord)
                || self.knowledge.blocked.contains(&coord)
                {
                    continue;
                }
                map[coord.row()][coord.col()] += 1;
            }
        }
        map
    }
}
