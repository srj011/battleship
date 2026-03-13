use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::errors::ApiError;
use crate::game::board::within_bounds;
use crate::game::coord::Coord;
use crate::game::game_state::Turn;
use crate::game::ship::{Direction, ShipPlacement, ShipType};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameMode {
    Ai,
    Multiplayer,
}

#[derive(Deserialize)]
pub struct CreateGameRequest {
    pub mode: GameMode,
}

#[derive(Serialize)]
pub struct CreateGameResponse {
    pub game_id: Uuid,
}

#[derive(Deserialize)]
pub struct GetGameQuery {
    pub player: Option<Turn>,
}

#[derive(Deserialize, Serialize)]
pub struct ApiCoord {
    pub row: i32,
    pub col: i32,
}

impl TryFrom<ApiCoord> for Coord {
    type Error = ApiError;

    fn try_from(value: ApiCoord) -> Result<Self, Self::Error> {
        if value.row < 0 || value.col < 0 {
            return Err(ApiError::InvalidCoordinates);
        }

        let coord = Coord::new(value.row as usize, value.col as usize);

        if !within_bounds(coord) {
            return Err(ApiError::InvalidCoordinates);
        }

        Ok(coord)
    }
}

impl From<Coord> for ApiCoord {
    fn from(coord: Coord) -> Self {
        Self {
            row: coord.row() as i32,
            col: coord.col() as i32,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ApiShipPlacement {
    pub ship_type: ShipType,
    pub start: ApiCoord,
    pub direction: Direction,
}

impl TryFrom<ApiShipPlacement> for ShipPlacement {
    type Error = ApiError;

    fn try_from(value: ApiShipPlacement) -> Result<Self, Self::Error> {
        let start: Coord = value.start.try_into()?;

        Ok(ShipPlacement {
            ship_type: value.ship_type,
            start,
            direction: value.direction,
        })
    }
}

impl From<ShipPlacement> for ApiShipPlacement {
    fn from(value: ShipPlacement) -> Self {
        Self {
            ship_type: value.ship_type,
            start: value.start.into(),
            direction: value.direction,
        }
    }
}

#[derive(Deserialize)]
pub struct PlaceFleetRequest {
    pub player: Turn,
    pub fleet: Vec<ApiShipPlacement>,
}

#[derive(Deserialize)]
pub struct FireRequest {
    pub player: Turn,
    pub coord: ApiCoord,
}
