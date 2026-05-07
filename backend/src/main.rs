use std::sync::{Arc, Mutex};
use std::time::Duration;
use time::macros::format_description;
use tokio::time::interval;
use tower_http::cors::CorsLayer;
use tracing::{debug, info, instrument, warn};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::LocalTime;

use battleship::api::routes::create_router;
use battleship::app::session_manager::SessionManager;

fn init_tracing() {
    let timer = LocalTime::new(format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
    ));

    tracing_subscriber::fmt()
        .with_timer(timer)
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("battleship=debug".parse().unwrap())
                .add_directive("info".parse().unwrap()),
        )
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .try_init()
        .expect("Failed to initialize tracing");
}

#[instrument(name = "cleanup", skip(manager), fields(interval_secs = 300))]
async fn cleanup(manager: Arc<Mutex<SessionManager>>) {
    info!("task started");

    let mut interval = interval(Duration::from_secs(300));
    interval.tick().await;

    loop {
        interval.tick().await;
        debug!("running cleanup");

        if let Ok(mut manager) = manager.lock() {
            let before = manager.count();
            manager.cleanup();
            let removed = before - manager.count();

            if removed > 0 {
                info!(removed, "expired sessions removed")
            }
        } else {
            warn!("cleanup lock poisoned");
        }
    }
}

#[tokio::main]
async fn main() {
    init_tracing();
    info!("server starting...");

    let manager = Arc::new(Mutex::new(SessionManager::new()));

    //Cleanup
    tokio::spawn(cleanup(manager.clone()));

    let app = create_router(manager).layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
