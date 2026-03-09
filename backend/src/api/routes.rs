use axum::{
    Router,
    routing::{get, post},
};
use std::sync::{Arc, Mutex};

use super::handlers::{create_game, fire, get_game, health, place_fleet};
use crate::app::session_manager::SessionManager;

pub fn create_router(manager: Arc<Mutex<SessionManager>>) -> Router {
    let api_v1 = Router::new()
        .route("/game", post(create_game))
        .route("/game/{id}", get(get_game))
        .route("/game/{id}/place-fleet", post(place_fleet))
        .route("/game/{id}/fire", post(fire))
        .with_state(manager.clone());

    Router::new()
        .route("/api/health", get(health))
        .nest("/api/v1", api_v1)
}
