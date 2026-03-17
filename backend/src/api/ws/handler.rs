use axum::{
    extract::{
        Path, State,
        ws::{self, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::api::ws::messages::ClientMessage;
use crate::app::session_manager::SessionManager;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(id): Path<Uuid>,
    State(manager): State<Arc<Mutex<SessionManager>>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, id, manager))
}

async fn handle_socket(socket: WebSocket, game_id: Uuid, manager: Arc<Mutex<SessionManager>>) {
    println!("Websocket connected for game {game_id}");
    let (mut sender, mut receiver) = socket.split();

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(ws::Message::Text(text)) => match serde_json::from_str::<ClientMessage>(&text) {
                Ok(client_msg) => match client_msg {
                    ClientMessage::Fire { coord } => {
                        println!("Fire request received: {coord:?}");

                        let session = {
                            let manager = manager.lock().unwrap();
                            manager.get_session(&game_id).unwrap().clone()
                        };

                        let result = {
                            let mut session = session.lock().unwrap();
                            session.player_fire(
                                crate::game::game_state::Turn::Player1,
                                coord.try_into().unwrap(),
                            )
                        };

                        println!("Fire result: {result:?}");

                        let reply = ws::Message::Text("ack".into());
                        sender.send(reply).await.unwrap();
                    }
                },
                Err(err) => println!("Invalid message: {err}"),
            },
            Ok(_) => {}
            Err(err) => {
                println!("Websocket error: {err}");
                break;
            }
        }
    }
    println!("Websocket disconnected");
}
