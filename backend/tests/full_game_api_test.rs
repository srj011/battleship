use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

use std::sync::{Arc, Mutex};
use std::usize;

use battleship::api::routes::create_router;
use battleship::app::session_manager::SessionManager;

#[tokio::test]
async fn full_game_flow_via_http() {
    let app = create_router(Arc::new(Mutex::new(SessionManager::new())));

    /*
    ------------------------
    1 CREATE GAME
    ------------------------
    */

    let response = app
        .clone()
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

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let game_id = json["game_id"].as_str().unwrap().to_string();

    /*
    ------------------------
    2 GET RANDOM FLEET
    ------------------------
    */

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/random-fleet")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let fleet = serde_json::from_slice::<serde_json::Value>(&body).unwrap();

    /*
    ------------------------
    3 PLACE FLEET
    ------------------------
    */

    let request_body = serde_json::json!({
        "player": "player1",
        "fleet": fleet
    });

    let uri = format!("/api/v1/game/{}/place-fleet", game_id);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&uri)
                .header("content-type", "application/json")
                .body(Body::from(request_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    /*
    ------------------------
    4 FIRE SHOTS
    ------------------------
    */

    let fire_body = r#"{
        "player":"player1",
        "coord":{"row":0,"col":0}
    }"#;

    let uri = format!("/api/v1/game/{}/fire", game_id);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&uri)
                .header("content-type", "application/json")
                .body(Body::from(fire_body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    /*
    ------------------------
    5 FETCH SNAPSHOT
    ------------------------
    */

    let uri = format!("/api/v1/game/{}?player=player1", game_id);

    let response = app
        .oneshot(Request::builder().uri(&uri).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
