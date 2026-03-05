use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::errors::ApiError;
use crate::app::game_session::TurnOutcome;
use crate::app::session_manager::SessionManager;
use crate::game::coord::Coord;
use crate::game::game_state::{GameError, Turn};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GameMode {
    Ai,
    Multiplayer,
}

#[derive(Deserialize)]
pub struct CreateGameRequest {
    mode: GameMode,
}

#[derive(Serialize)]
pub struct CreateGameResponse {
    game_id: Uuid,
}

#[derive(Deserialize)]
pub struct FireRequest {
    player: Turn,
    coord: Coord,
}

pub async fn root() -> &'static str {
    "Battleship backend running"
}

pub async fn create_game(
    State(manager): State<Arc<Mutex<SessionManager>>>,
    Json(request): Json<CreateGameRequest>,
) -> Json<CreateGameResponse> {
    let mut manager = manager.lock().unwrap();

    let game_id = match request.mode {
        GameMode::Ai => manager.create_vs_ai(),
        GameMode::Multiplayer => manager.create_multiplayer(),
    };

    Json(CreateGameResponse { game_id })
}
