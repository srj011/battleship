use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn offset(self, dr: isize, dc: isize) -> Option<Self> {
        let new_row = self.row as isize + dr;
        let new_col = self.col as isize + dc;

        if new_row >= 0 && new_col >= 0 {
            Some(Self {
                row: new_row as usize,
                col: new_col as usize,
            })
        } else {
            None
        }
    }
}
