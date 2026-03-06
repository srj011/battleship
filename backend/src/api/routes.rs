use axum::{
    Router,
    routing::{get, post},
};
use std::sync::{Arc, Mutex};

use super::handlers::{create_game, fire, get_game, root};
use crate::app::session_manager::SessionManager;

pub fn create_router(manager: Arc<Mutex<SessionManager>>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/game", post(create_game))
        .route("/game/{id}", get(get_game))
        .route("/game/{id}/fire", post(fire))
        .with_state(manager)
}
