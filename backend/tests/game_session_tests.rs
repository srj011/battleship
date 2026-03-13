use battleship::game::coord::Coord;
use battleship::game::game_state::{GameState, GameStatus, Turn};
use battleship::game::player::Player;
use battleship::game::ship::{Direction, ShipPlacement, ShipType};

fn simple_fleet() -> Vec<ShipPlacement> {
    vec![
        ShipPlacement {
            ship_type: ShipType::Carrier,
            start: Coord::new(0, 0),
            direction: Direction::Horizontal,
        },
        ShipPlacement {
            ship_type: ShipType::Battleship,
            start: Coord::new(1, 0),
            direction: Direction::Horizontal,
        },
        ShipPlacement {
            ship_type: ShipType::Destroyer,
            start: Coord::new(2, 0),
            direction: Direction::Horizontal,
        },
        ShipPlacement {
            ship_type: ShipType::Submarine,
            start: Coord::new(3, 0),
            direction: Direction::Horizontal,
        },
        ShipPlacement {
            ship_type: ShipType::PatrolBoat,
            start: Coord::new(4, 0),
            direction: Direction::Horizontal,
        },
    ]
}

#[test]
fn game_transitions_to_ongoing_when_fleets_placed() {
    let mut game = GameState::new(Player::new(), Player::new());

    game.place_fleet(Turn::Player1, simple_fleet()).unwrap();
    game.place_fleet(Turn::Player2, simple_fleet()).unwrap();

    assert!(matches!(game.status(), GameStatus::Ongoing));
}

#[test]
fn turn_switches_after_miss() {
    let mut game = GameState::new(Player::new(), Player::new());

    game.place_fleet(Turn::Player1, simple_fleet()).unwrap();
    game.place_fleet(Turn::Player2, simple_fleet()).unwrap();

    let _ = game.take_turn(Coord::new(9, 9));

    assert_eq!(game.current_turn(), Turn::Player2);
}
