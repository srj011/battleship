use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::errors::ApiError;
use crate::game::board::within_bounds;
use crate::game::coord::Coord;
use crate::game::game_state::Turn;

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
pub struct FireRequest {
    pub player: Turn,
    pub coord: ApiCoord,
}

#[derive(Deserialize)]
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
