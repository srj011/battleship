use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::json;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::errors::ApiError;
use super::types::*;
use crate::app::session_manager::SessionManager;
use crate::game::board::within_bounds;
use crate::game::coord::Coord;
use crate::{
    app::game_session::{GameSnapshot, TurnOutcome},
    game::ship::ShipPlacement,
};

pub async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
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

pub async fn get_game(
    Path(id): Path<Uuid>,
    State(manager): State<Arc<Mutex<SessionManager>>>,
) -> Result<Json<GameSnapshot>, ApiError> {
    let session = {
        let manager = manager.lock().unwrap();
        manager.get_session(&id).ok_or(ApiError::SessionNotFound)?
    };
    let session = session.lock().unwrap();

    Ok(Json(session.snapshot()))
}

pub async fn place_fleet(
    Path(id): Path<Uuid>,
    State(manager): State<Arc<Mutex<SessionManager>>>,
    Json(request): Json<PlaceFleetRequest>,
) -> Result<Json<GameSnapshot>, ApiError> {
    let placements: Vec<ShipPlacement> = request
        .fleet
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, _>>()?;

    let session = {
        let manager = manager.lock().unwrap();
        manager.get_session(&id).ok_or(ApiError::SessionNotFound)?
    };
    let mut session = session.lock().unwrap();

    session.place_fleet(request.player, placements)?;

    Ok(Json(session.snapshot()))
}

pub async fn fire(
    Path(id): Path<Uuid>,
    State(manager): State<Arc<Mutex<SessionManager>>>,
    Json(request): Json<FireRequest>,
) -> Result<Json<TurnOutcome>, ApiError> {
    let coord: Coord = request.coord.try_into()?;

    if !within_bounds(coord) {
        return Err(ApiError::InvalidCoordinates);
    }

    let session = {
        let manager = manager.lock().unwrap();
        manager.get_session(&id).ok_or(ApiError::SessionNotFound)?
    };
    let mut session = session.lock().unwrap();

    let outcome = session.player_fire(request.player, coord)?;

    Ok(Json(outcome))
}
