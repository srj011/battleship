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
    InvalidMessage,
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
#[serde(rename_all = "snake_case")]
enum ErrorCode {
    SessionNotFound,
    InvalidPlayer,
    InvalidMessage,
    InvalidCoordinates,
    NotPlayersTurn,
    GameAlreadyFinished,
    InvalidGameState,
    GameFull,
    InvalidPlacement,
    InternalError,
}

impl From<&ApiError> for ErrorCode {
    fn from(value: &ApiError) -> Self {
        match value {
            ApiError::SessionNotFound => ErrorCode::SessionNotFound,
            ApiError::InvalidPlayer => ErrorCode::InvalidPlayer,
            ApiError::InvalidMessage => ErrorCode::InvalidMessage,
            ApiError::InvalidCoordinates => ErrorCode::InvalidCoordinates,
            ApiError::Game(GameError::NotPlayersTurn) => ErrorCode::NotPlayersTurn,
            ApiError::Game(GameError::GameAlreadyFinished) => ErrorCode::GameAlreadyFinished,
            ApiError::Game(GameError::InvalidGameState) => ErrorCode::InvalidGameState,
            ApiError::Game(GameError::GameFull) => ErrorCode::GameFull,
            ApiError::Game(GameError::Placement(_)) => ErrorCode::InvalidPlacement,
            ApiError::Internal => ErrorCode::InternalError,
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: &'static str,
}

impl ApiError {
    pub fn to_response(&self) -> (StatusCode, ErrorResponse) {
        match self {
            ApiError::SessionNotFound => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    code: self.into(),
                    message: "Session not found",
                },
            ),

            ApiError::InvalidPlayer => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: self.into(),
                    message: "Invalid player token",
                },
            ),

            ApiError::InvalidMessage => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: self.into(),
                    message: "Invalid message received",
                },
            ),

            ApiError::InvalidCoordinates => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: self.into(),
                    message: "Invalid coordinate",
                },
            ),

            ApiError::Game(GameError::NotPlayersTurn) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: self.into(),
                    message: "Not your turn",
                },
            ),

            ApiError::Game(GameError::GameAlreadyFinished) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: self.into(),
                    message: "Game already finished",
                },
            ),

            ApiError::Game(GameError::InvalidGameState) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: self.into(),
                    message: "Invalid game state",
                },
            ),

            ApiError::Game(GameError::GameFull) => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    code: self.into(),
                    message: "Game already full",
                },
            ),

            ApiError::Game(GameError::Placement(_)) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: self.into(),
                    message: "Invalid ship placement",
                },
            ),

            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: self.into(),
                    message: "Internal server error",
                },
            ),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, response) = self.to_response();
        let body = Json(response);

        (status, body).into_response()
    }
}
