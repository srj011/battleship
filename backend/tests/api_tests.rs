use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use std::sync::{Arc, Mutex};

use battleship::api::routes::create_router;
use battleship::app::session_manager::SessionManager;

#[tokio::test]
async fn health_endpoint_works() {
    let app = create_router(Arc::new(Mutex::new(SessionManager::new())));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn create_game_returns_id() {
    let app = create_router(Arc::new(Mutex::new(SessionManager::new())));

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/game")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"mode":"ai"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
