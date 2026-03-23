use axum::{
    Router,
    routing::{get, post},
};
use std::sync::{Arc, Mutex};

use super::handlers::{create_game, fire, get_game, health, join_game, place_fleet, random_fleet};
use super::ws::handler::ws_handler;
use crate::app::session_manager::SessionManager;

pub fn create_router(manager: Arc<Mutex<SessionManager>>) -> Router {
    let game_routes = Router::new()
        .route("/", post(create_game))
        .route("/{id}", get(get_game))
        .route("/{id}/join", post(join_game))
        .route("/{id}/place-fleet", post(place_fleet))
        .route("/{id}/fire", post(fire))
        .route("/{id}/ws/{player}", get(ws_handler));

    let api_v1 = Router::new()
        .nest("/game", game_routes)
        .route("/random-fleet", get(random_fleet));

    Router::new()
        .route("/api/health", get(health))
        .nest("/api/v1", api_v1)
        .with_state(manager)
}
