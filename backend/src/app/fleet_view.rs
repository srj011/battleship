use serde::Serialize;

use crate::game::ship::{Ship, ShipType};

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ShipStatus {
    pub ship_type: ShipType,
    pub damage: Option<u8>,
    pub sunk: bool,
}

pub enum FleetPerspective {
    Owner,
    Opponent,
}

#[derive(Debug, Clone, Serialize)]
pub struct FleetView {
    pub ships: Vec<ShipStatus>,
}

impl FleetView {
    pub fn from_fleet(fleet: &[Ship], perspective: FleetPerspective) -> Self {
        let ships = fleet
            .iter()
            .map(|ship| match perspective {
                FleetPerspective::Owner => ShipStatus {
                    ship_type: ship.ship_type(),
                    damage: Some(ship.hits()),
                    sunk: ship.is_sunk(),
                },
                FleetPerspective::Opponent => ShipStatus {
                    ship_type: ship.ship_type(),
                    damage: None,
                    sunk: ship.is_sunk(),
                },
            })
            .collect();
        Self { ships }
    }
}
