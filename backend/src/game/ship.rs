use serde::{Deserialize, Serialize};

use crate::game::coord::Coord;

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

impl ShipType {
    pub fn length(&self) -> usize {
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
    hits: usize,
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

    pub fn register_hit(&mut self) {
        if self.hits < self.ship_type.length() {
            self.hits += 1;
        }
    }

    pub fn is_sunk(&self) -> bool {
        self.hits == self.ship_type.length()
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Horizontal,
    Vertical,
}
