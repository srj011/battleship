use std::sync::{Arc, Mutex};

use battleship::api::routes::create_router;
use battleship::app::session_manager::SessionManager;

#[tokio::main]
async fn main() {
    let manager = Arc::new(Mutex::new(SessionManager::new()));

    let app = create_router(manager);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
