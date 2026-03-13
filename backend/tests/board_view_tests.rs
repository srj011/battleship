use battleship::app::board_view::{BoardPerspective, BoardView, CellView};
use battleship::game::board::Board;
use battleship::game::coord::Coord;
use battleship::game::ship::{Direction, ShipType};

#[test]
fn owner_view_shows_ships() {
    let mut board = Board::new();

    board
        .place_ship(
            ShipType::PatrolBoat,
            Coord::new(0, 0),
            Direction::Horizontal,
        )
        .unwrap();

    let view = BoardView::new(&board, BoardPerspective::Owner);

    assert!(matches!(view.cells[0][0], CellView::Ship(_)));
}

#[test]
fn opponent_view_hides_ships() {
    let mut board = Board::new();

    board
        .place_ship(
            ShipType::PatrolBoat,
            Coord::new(0, 0),
            Direction::Horizontal,
        )
        .unwrap();

    let view = BoardView::new(&board, BoardPerspective::Opponent);

    assert!(matches!(view.cells[0][0], CellView::Unknown));
}

#[test]
fn hits_are_visible_to_both_players() {
    let mut board = Board::new();

    board
        .place_ship(
            ShipType::PatrolBoat,
            Coord::new(0, 0),
            Direction::Horizontal,
        )
        .unwrap();

    board.fire_at(Coord::new(0, 0));

    let view = BoardView::new(&board, BoardPerspective::Opponent);

    assert!(matches!(view.cells[0][0], CellView::Hit));
}
