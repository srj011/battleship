use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::game::game_state::GameError;

pub enum ApiError {
    Game(GameError),
    SessionNotFound,
}

impl From<GameError> for ApiError {
    fn from(err: GameError) -> Self {
        ApiError::Game(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Game(GameError::NotPlayersTurn) => {
                (StatusCode::BAD_REQUEST, "Not your turn").into_response()
            }
            ApiError::Game(GameError::GameAlreadyFinished) => {
                (StatusCode::BAD_REQUEST, "Game already finished").into_response()
            }
            ApiError::SessionNotFound => {
                (StatusCode::BAD_REQUEST, "Session not found").into_response()
            }
        }
    }
}
