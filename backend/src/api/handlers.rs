use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde_json::json;
use std::sync::{Arc, Mutex};

use super::errors::ApiError;
use super::types::*;
use crate::app::game_session::{GameSnapshot, TurnOutcome};
use crate::app::session_manager::SessionManager;
use crate::game::board::within_bounds;
use crate::game::coord::Coord;
use crate::game::player::Player;
use crate::game::ship::ShipPlacement;

pub async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

pub async fn create_game(
    State(manager): State<Arc<Mutex<SessionManager>>>,
    Json(request): Json<CreateGameRequest>,
) -> Json<CreateGameResponse> {
    let mut manager = manager.lock().unwrap();

    let (game_code, player_token) = match request.mode {
        GameMode::Ai => manager.create_vs_ai(),
        GameMode::Multiplayer => manager.create_multiplayer(),
    };

    Json(CreateGameResponse {
        game_code,
        player_token,
    })
}

pub async fn join_game(
    Path(code): Path<String>,
    State(manager): State<Arc<Mutex<SessionManager>>>,
) -> Result<Json<JoinGameResponse>, ApiError> {
    let session_arc = {
        let manager = manager.lock().unwrap();
        manager
            .get_session_by_code(&code)
            .ok_or(ApiError::SessionNotFound)?
    };

    let mut session = session_arc.lock().unwrap();

    let player_token = session.join_player()?;

    Ok(Json(JoinGameResponse { player_token }))
}

pub async fn get_game(
    Path(code): Path<String>,
    State(manager): State<Arc<Mutex<SessionManager>>>,
    Query(query): Query<GetGameQuery>,
) -> Result<Json<GameSnapshot>, ApiError> {
    let session_arc = {
        let manager = manager.lock().unwrap();
        manager
            .get_session_by_code(&code)
            .ok_or(ApiError::SessionNotFound)?
    };
    let session = session_arc.lock().unwrap();

    let player = session
        .player_from_token(query.player_token)
        .ok_or(ApiError::InvalidPlayer)?;
    Ok(Json(session.snapshot_for(player)))
}

pub async fn place_fleet(
    Path(code): Path<String>,
    State(manager): State<Arc<Mutex<SessionManager>>>,
    Json(request): Json<PlaceFleetRequest>,
) -> Result<Json<GameSnapshot>, ApiError> {
    let placements: Vec<ShipPlacement> = request
        .fleet
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, _>>()?;

    let session_arc = {
        let manager = manager.lock().unwrap();
        manager
            .get_session_by_code(&code)
            .ok_or(ApiError::SessionNotFound)?
    };
    let mut session = session_arc.lock().unwrap();

    let player = session
        .player_from_token(request.player_token)
        .ok_or(ApiError::InvalidPlayer)?;

    session.place_fleet(player, placements)?;

    Ok(Json(session.snapshot_for(player)))
}

pub async fn random_fleet() -> Json<Vec<ApiShipPlacement>> {
    Json(
        Player::generate_random_fleet()
            .into_iter()
            .map(Into::into)
            .collect(),
    )
}

pub async fn fire(
    Path(code): Path<String>,
    State(manager): State<Arc<Mutex<SessionManager>>>,
    Json(request): Json<FireRequest>,
) -> Result<Json<TurnOutcome>, ApiError> {
    let coord: Coord = request.coord.try_into()?;

    if !within_bounds(coord) {
        return Err(ApiError::InvalidCoordinates);
    }

    let session_arc = {
        let manager = manager.lock().unwrap();
        manager
            .get_session_by_code(&code)
            .ok_or(ApiError::SessionNotFound)?
    };
    let mut session = session_arc.lock().unwrap();

    let player = session
        .player_from_token(request.player_token)
        .ok_or(ApiError::InvalidPlayer)?;

    let outcome = session.player_fire(player, coord)?;

    Ok(Json(outcome))
}
