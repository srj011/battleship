use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::interval;
use tower_http::cors::CorsLayer;

use battleship::api::routes::create_router;
use battleship::app::session_manager::SessionManager;

async fn cleanup(manager: Arc<Mutex<SessionManager>>) {
    eprintln!("Cleanup task started");

    let mut interval = interval(Duration::from_secs(300));
    interval.tick().await;

    loop {
        interval.tick().await;
        eprintln!("running cleanup");

        if let Ok(mut manager) = manager.lock() {
            let before = manager.count();
            manager.cleanup();
            let removed = before - manager.count();

            if removed > 0 {
                eprintln!(removed, "expired sessions removed")
            }
        } else {
            eprintln!("cleanup lock poisoned");
        }
    }
}

#[tokio::main]
async fn main() {
    let manager = Arc::new(Mutex::new(SessionManager::new()));

    //Cleanup
    tokio::spawn(cleanup(manager.clone()));

    let app = create_router(manager).layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
