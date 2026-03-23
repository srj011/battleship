use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::game::errors::GameError;

#[derive(Debug)]
pub enum ApiError {
    SessionNotFound,
    InvalidCoordinates,
    InvalidPlayer,
    Internal,
    Game(GameError),
}

impl From<GameError> for ApiError {
    fn from(err: GameError) -> Self {
        ApiError::Game(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(_value: serde_json::Error) -> Self {
        ApiError::Internal
    }
}

impl From<axum::Error> for ApiError {
    fn from(_value: axum::Error) -> Self {
        ApiError::Internal
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
            ApiError::InvalidPlayer => (StatusCode::BAD_REQUEST, "Missing player parameter"),
            ApiError::Game(GameError::NotPlayersTurn) => (StatusCode::BAD_REQUEST, "Not your turn"),
            ApiError::Game(GameError::GameAlreadyFinished) => {
                (StatusCode::BAD_REQUEST, "Game already finished")
            }
            ApiError::Game(GameError::InvalidGameState) => {
                (StatusCode::BAD_REQUEST, "Invalid game state")
            }
            ApiError::Game(GameError::GameFull) => (StatusCode::CONFLICT, "Game already full"),
            ApiError::Game(GameError::Placement(_)) => {
                (StatusCode::BAD_REQUEST, "Invalid ship placement")
            }
            ApiError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
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
