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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_moves_correctly() {
        let c = Coord::new(5, 5);
        let next = c.offset(1, -1).unwrap();

        assert_eq!(next.row, 6);
        assert_eq!(next.col, 4);
    }

    #[test]
    fn offset_prevents_negative_coordinates() {
        let c = Coord::new(0, 0);
        assert!(c.offset(-1, 0).is_none());
    }
}
