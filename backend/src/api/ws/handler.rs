use axum::{
    extract::{
        Path, State,
        ws::{self, Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt, stream::SplitSink};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::api::errors::ApiError;
use crate::api::types::ApiCoord;
use crate::api::ws::messages::{ClientMessage, ServerMessage};
use crate::app::game_session::GameSession;
use crate::app::session_manager::SessionManager;
use crate::game::coord::Coord;
use crate::game::errors::GameError;
use crate::game::game_state::Turn;

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
                        eprintln!("[WS] Fire request: {coord:?}");

                        let session_opt = {
                            let manager = manager.lock().unwrap();
                            manager.get_session(&game_id)
                        };

                        let session = {
                            match session_opt {
                                Some(s) => s,
                                None => {
                                    eprintln!("[WS] Error: Session Not Found");

                                    let error_msg = ServerMessage::Error {
                                        message: "Session not found".into(),
                                    };

                                    let _ = sender
                                        .send(Message::Text(
                                            serde_json::to_string(&error_msg).unwrap().into(),
                                        ))
                                        .await;
                                    return;
                                }
                            }
                        };

                        if let Err(e) = handle_fire(&mut sender, session, coord).await {
                            eprintln!("[WS] error: {e:?}");

                            let message = match e {
                                ApiError::Game(GameError::NotPlayersTurn) => "Not your turn",
                                ApiError::InvalidCoordinates => "Invalid coordinate",
                                _ => "Internal error",
                            };

                            let error_msg = ServerMessage::Error {
                                message: message.to_string(),
                            };

                            let _ = sender
                                .send(Message::Text(
                                    serde_json::to_string(&error_msg).unwrap().into(),
                                ))
                                .await;
                        }
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

async fn handle_fire(
    sender: &mut SplitSink<WebSocket, Message>,
    session: Arc<Mutex<GameSession>>,
    coord: ApiCoord,
) -> Result<(), ApiError> {
    let coord: Coord = coord.try_into()?;

    let message = {
        let mut session = session.lock().unwrap();
        let update = session.fire_once(Turn::Player1, coord)?;

        let snapshot = session.snapshot_for(Turn::Player1);
        ServerMessage::GameUpdate {
            event: update.event,
            turn: update.turn,
            status: update.status,
            player_board: snapshot.player_board,
            opponent_board: snapshot.opponent_board,
        }
    };

    sender
        .send(Message::Text(serde_json::to_string(&message)?.into()))
        .await?;

    Ok(())
}
