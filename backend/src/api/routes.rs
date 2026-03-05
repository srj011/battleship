use axum::{
    Router,
    routing::{get, post},
};
use std::sync::{Arc, Mutex};

use super::handlers::*;
use crate::app::session_manager::SessionManager;

pub fn create_router(manager: Arc<Mutex<SessionManager>>) -> Router {
    Router::new().route("/", get(root)).with_state(manager)
}
