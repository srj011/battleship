use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub row: usize,
    pub col: usize,
}
