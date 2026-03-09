use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::game::errors::GameError;

pub enum ApiError {
    SessionNotFound,
    InvalidCoordinates,
    Game(GameError),
}

impl From<GameError> for ApiError {
    fn from(err: GameError) -> Self {
        ApiError::Game(err)
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: &'static str,
}

impl ApiError {
    fn status_message(&self) -> (StatusCode, &'static str) {
        match self {
            ApiError::SessionNotFound => (StatusCode::NOT_FOUND, "Session not found"),
            ApiError::InvalidCoordinates => (StatusCode::BAD_REQUEST, "Invalid coordinate"),
            ApiError::Game(GameError::NotPlayersTurn) => (StatusCode::BAD_REQUEST, "Not your turn"),
            ApiError::Game(GameError::GameAlreadyFinished) => {
                (StatusCode::BAD_REQUEST, "Game already finished")
            }
            ApiError::Game(GameError::InvalidGameState) => {
                (StatusCode::BAD_REQUEST, "Invalid game state")
            }
            ApiError::Game(GameError::Placement(_)) => {
                (StatusCode::BAD_REQUEST, "Invalid ship placement")
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = self.status_message();
        let body = Json(ErrorResponse { error: message });

        (status, body).into_response()
    }
}
