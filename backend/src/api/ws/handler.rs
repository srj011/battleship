use axum::{
    extract::{
        Path, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
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
    Path((id, player)): Path<(Uuid, Turn)>,
    State(manager): State<Arc<Mutex<SessionManager>>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, id, player, manager))
}

async fn handle_socket(
    socket: WebSocket,
    game_id: Uuid,
    player: Turn,
    manager: Arc<Mutex<SessionManager>>,
) {
    let (mut sender, mut receiver) = socket.split();

    // Get Session
    let session_opt = {
        let manager = manager.lock().unwrap();
        manager.get_session(&game_id)
    };

    let (session, mut rx) = match session_opt {
        Some(s) => {
            let session = s;
            let rx = {
                let session_guard = session.lock().unwrap();
                session_guard.subscribe()
            };
            (session, rx)
        }
        None => {
            eprintln!("[WS] Error: Session not found");

            let error_msg = ServerMessage::Error {
                message: "Session Not Found".into(),
            };
            let _ = sender
                .send(Message::Text(
                    serde_json::to_string(&error_msg)
                        .unwrap_or_else(|_| "{\"type\":\"error\", \"message\":\"internal\"}".into())
                        .into(),
                ))
                .await;
            return;
        }
    };

    eprintln!("[WS] Connected: {game_id} as {player:?}");

    // Inital message
    let initial_message = {
        let session_guard = session.lock().unwrap();
        let snapshot = session_guard.snapshot_for(player);

        ServerMessage::GameState {
            turn: session_guard.current_turn(),
            status: session_guard.status(),
            player_board: snapshot.player_board,
            opponent_board: snapshot.opponent_board,
        }
    };

    let _ = sender
        .send(Message::Text(
            serde_json::to_string(&initial_message)
                .unwrap_or_else(|_| "{\"type\":\"error\", \"message\":\"internal\"}".into())
                .into(),
        ))
        .await;

    // Event loop
    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Ok(ClientMessage::Fire { coord }) => {
                                if let Err(e) = handle_fire(session.clone(), player, coord).await {
                                    eprintln!("[WS] error: {e:?}");
                                    let error_msg = error_to_ws_message(e);
                                    let _ = sender.send(error_msg).await;
                                }
                            },
                            Err(err) => {
                                eprintln!("Invalid message: {err}");
                            }
                        }
                    },
                    Some(Ok(_)) => {},
                    Some(Err(err)) => {
                        eprintln!("[WS] error: {err}");
                        break;
                    },
                    None => {
                        eprintln!("[WS] client disconnected");
                        break;
                    }
                }
            }
            update = rx.recv() => {
                match update {
                    Ok(update) => {
                        let message = {
                            let session_guard = session.lock().unwrap();
                            let snapshot = session_guard.snapshot_for(player);

                            ServerMessage::GameUpdate {
                                event: update.event,
                                turn: update.turn,
                                status: update.status,
                                player_board: snapshot.player_board,
                                opponent_board: snapshot.opponent_board,
                            }
                        };

                        let _ = sender.send(Message::Text(serde_json::to_string(&message)
                            .unwrap_or_else(|_| "{\"type\":\"error\", \"message\":\"internal\"}".into())
                            .into())).await;
                    },
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        eprintln!("[WS] lagged");
                        continue;
                    }
                    Err(_) => break,
                }
            }
        }
    }
    eprintln!("[WS] Disconnected: {game_id} for {player:?}");
}

async fn handle_fire(
    session: Arc<Mutex<GameSession>>,
    player: Turn,
    coord: ApiCoord,
) -> Result<(), ApiError> {
    let coord: Coord = coord.try_into()?;

    let mut session_guard = session.lock().unwrap();
    session_guard.fire_once(player, coord)?;
    session_guard.ai_turn()?;

    Ok(())
}

fn map_error_to_message(e: ApiError) -> ServerMessage {
    let message = match e {
        ApiError::Game(GameError::NotPlayersTurn) => "Not your turn",
        ApiError::InvalidCoordinates => "Invalid coordinates",
        _ => "Internal error",
    };

    ServerMessage::Error {
        message: message.to_string(),
    }
}

fn error_to_ws_message(e: ApiError) -> Message {
    let server_msg = map_error_to_message(e);

    Message::Text(
        serde_json::to_string(&server_msg)
            .unwrap_or_else(|_| "{\"type\":\"error\", \"message\":\"internal\"}".into())
            .into(),
    )
}
