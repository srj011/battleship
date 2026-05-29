use serde::{Deserialize, Serialize};

use super::coord::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShipType {
    Carrier,
    Battleship,
    Destroyer,
    Submarine,
    PatrolBoat,
}

pub struct ShipPlacement {
    pub ship_type: ShipType,
    pub start: Coord,
    pub direction: Direction,
}

impl ShipPlacement {
    pub fn coords(&self) -> Vec<Coord> {
        (0..self.ship_type.length()).map(|i| {
            match self.direction {
                Direction::Horizontal => Coord::new(
                    self.start.row(),
                    self.start.col() + i as usize
                ),
                Direction::Vertical => Coord::new(
                    self.start.row() + i as usize,
                    self.start.col()
                )
            }
        }).collect()
    }
}

impl ShipType {
    pub fn length(&self) -> u8 {
        match self {
            Self::Carrier => 5,
            Self::Battleship => 4,
            Self::Destroyer => 3,
            Self::Submarine => 3,
            Self::PatrolBoat => 2,
        }
    }
}

pub const FLEET: [ShipType; 5] = [
    ShipType::Carrier,
    ShipType::Battleship,
    ShipType::Destroyer,
    ShipType::Submarine,
    ShipType::PatrolBoat,
];

pub struct Ship {
    ship_type: ShipType,
    positions: Vec<Coord>,
    hits: u8,
}

impl Ship {
    pub fn new(ship_type: ShipType, positions: Vec<Coord>) -> Self {
        Self {
            ship_type,
            positions,
            hits: 0,
        }
    }

    pub fn positions(&self) -> &Vec<Coord> {
        &self.positions
    }

    pub fn ship_type(&self) -> ShipType {
        self.ship_type
    }

    pub fn hits(&self) -> u8 {
        self.hits
    }

    pub fn register_hit(&mut self) {
        if self.hits < self.ship_type.length() {
            self.hits += 1;
        }
    }

    pub fn is_sunk(&self) -> bool {
        self.hits == self.ship_type.length()
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Horizontal,
    Vertical,
}
