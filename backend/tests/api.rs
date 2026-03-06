use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use tower::ServiceExt;

use std::sync::{Arc, Mutex};

use backend::api::routes::create_router;
use backend::app::session_manager::SessionManager;

#[tokio::test]
async fn create_game_returns_game_id() {
    let manager = Arc::new(Mutex::new(SessionManager::new()));
    let app = create_router(manager);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/game")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"mode":"ai"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json.get("game_id").is_some());
}

#[tokio::test]
async fn get_game_snapshot_works() {
    let manager = Arc::new(Mutex::new(SessionManager::new()));
    let app = create_router(manager.clone());

    // Create game
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/game")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"mode":"ai"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let game_id = json["game_id"].as_str().unwrap();

    // Request snapshot
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/game/{}", game_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn fire_endpoint_returns_events() {
    let manager = Arc::new(Mutex::new(SessionManager::new()));
    let app = create_router(manager.clone());

    // Create game
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/game")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"mode":"ai"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let game_id = json["game_id"].as_str().unwrap();

    // Fire shot
    let fire_body = json!({
        "player": "player1",
        "row": 3,
        "col": 4
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/game/{}/fire", game_id))
                .header("content-type", "application/json")
                .body(Body::from(fire_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json.get("events").is_some());
}

#[tokio::test]
async fn fire_rejects_invalid_coordinates() {
    let manager = Arc::new(Mutex::new(SessionManager::new()));
    let app = create_router(manager.clone());

    // Create game
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/game")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"mode":"ai"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let game_id = json["game_id"].as_str().unwrap();

    // Fire invalid coordinate
    let fire_body = json!({
        "player": "player1",
        "row": 50,
        "col": 50
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/game/{}/fire", game_id))
                .header("content-type", "application/json")
                .body(Body::from(fire_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
