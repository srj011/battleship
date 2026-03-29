use serde::Serialize;

use crate::game::ship::ShipType;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ShipStatus {
    pub ship_type: ShipType,
    pub damage: u8,
    pub sunk: bool,
}

impl ShipStatus {
    pub fn new(ship_type: ShipType, damage: u8, sunk: bool) -> Self {
        Self {
            ship_type,
            damage,
            sunk,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FleetView {
    pub ships: Vec<ShipStatus>,
}

impl FleetView {
    pub fn new(ships: Vec<ShipStatus>) -> Self {
        Self { ships }
    }
}
