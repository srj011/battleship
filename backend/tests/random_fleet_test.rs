use battleship::game::player::Player;
use battleship::game::ship::{FLEET, ShipType};
use std::collections::HashSet;

#[test]
fn random_fleet_contains_all_ship_types() {
    let fleet = Player::generate_random_fleet();

    let types: HashSet<ShipType> = fleet.iter().map(|p| p.ship_type).collect();

    let expected: HashSet<ShipType> = FLEET.iter().copied().collect();

    assert_eq!(types, expected);
}
