use serde::Serialize;

use crate::game::board::{BOARD_SIZE, Board, Cell};
use crate::game::coord::Coord;
use crate::game::ship::ShipType;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum CellView {
    Unknown,
    Empty,
    Ship(ShipType),
    Hit,
    Miss,
}

pub enum BoardPerspective {
    Owner,
    Opponent,
}

#[derive(Debug, Serialize)]
pub struct BoardView {
    pub cells: [[CellView; BOARD_SIZE]; BOARD_SIZE],
}

impl BoardView {
    pub fn new(board: &Board, perspective: BoardPerspective) -> Self {
        let mut cells = [[CellView::Empty; BOARD_SIZE]; BOARD_SIZE];

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let coord = Coord::new(row, col);
                cells[row][col] = match board.get_cell(coord) {
                    Cell::Empty => match perspective {
                        BoardPerspective::Owner => CellView::Empty,
                        BoardPerspective::Opponent => CellView::Unknown,
                    },
                    Cell::Ship(ship_type) => match perspective {
                        BoardPerspective::Owner => CellView::Ship(ship_type),
                        BoardPerspective::Opponent => CellView::Unknown,
                    },
                    Cell::Hit(_) => CellView::Hit,
                    Cell::Miss => CellView::Miss,
                };
            }
        }
        Self { cells }
    }
}
