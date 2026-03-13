use battleship::game::board::{Board, FireOutcome, within_bounds};
use battleship::game::coord::Coord;
use battleship::game::ship::{Direction, ShipType};

#[test]
fn place_ship_success() {
    let mut board = Board::new();

    let result = board.place_ship(ShipType::Destroyer, Coord::new(0, 0), Direction::Horizontal);

    assert!(result.is_ok());
}

#[test]
fn place_ship_overlap_fails() {
    let mut board = Board::new();

    board
        .place_ship(ShipType::Destroyer, Coord::new(0, 0), Direction::Horizontal)
        .unwrap();

    let result = board.place_ship(ShipType::Submarine, Coord::new(0, 1), Direction::Vertical);

    assert!(result.is_err());
}

#[test]
fn fire_hit_and_miss() {
    let mut board = Board::new();

    board
        .place_ship(
            ShipType::PatrolBoat,
            Coord::new(0, 0),
            Direction::Horizontal,
        )
        .unwrap();

    assert!(matches!(
        board.fire_at(Coord::new(0, 0)),
        FireOutcome::Hit(_)
    ));

    assert!(matches!(board.fire_at(Coord::new(5, 5)), FireOutcome::Miss));
}

#[test]
fn firing_twice_returns_already_shot() {
    let mut board = Board::new();

    board.fire_at(Coord::new(4, 4));

    assert!(matches!(
        board.fire_at(Coord::new(4, 4)),
        FireOutcome::AlreadyShot
    ));
}

#[test]
fn bounds_check() {
    assert!(within_bounds(Coord::new(0, 0)));
    assert!(within_bounds(Coord::new(9, 9)));

    assert!(!within_bounds(Coord::new(10, 0)));
}
