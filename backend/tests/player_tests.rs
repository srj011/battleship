use battleship::game::coord::Coord;
use battleship::game::player::{Player, ShotResult};
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
fn player_places_valid_fleet() {
    let mut player = Player::new();

    let fleet = simple_fleet();

    assert!(player.place_fleet(fleet).is_ok());
}

#[test]
fn random_fleet_is_valid() {
    let fleet = Player::generate_random_fleet();

    let mut player = Player::new();

    assert!(player.place_fleet(fleet).is_ok());
}

#[test]
fn fire_at_returns_hit() {
    let mut player = Player::new();

    player.place_fleet(simple_fleet()).unwrap();

    let result = player.fire_at(Coord::new(0, 0));

    assert!(matches!(result, ShotResult::Hit));
}

#[test]
fn player_loses_when_all_ships_sunk() {
    let mut player = Player::new();

    player.place_fleet(simple_fleet()).unwrap();

    for col in 0..5 {
        player.fire_at(Coord::new(0, col));
    }

    assert!(!player.has_lost());
}
