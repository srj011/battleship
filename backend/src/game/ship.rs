use crate::game::coord::Coord;

pub struct Ship {
    positions: Vec<Coord>,
    hits: usize,
}

impl Ship {
    pub fn new(positions: Vec<Coord>) -> Self {
        Self { positions, hits: 0 }
    }

    pub fn register_hit(&mut self) {
        if self.hits < self.positions.len() {
            self.hits += 1;
        }
    }

    pub fn is_sunk(&self) -> bool {
        self.hits == self.positions.len()
    }
}

pub enum Direction {
    Horizontal,
    Vertical,
}
