use axum::{
    extract::{
        Path, Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::api::errors::ApiError;
use crate::api::types::{ApiCoord, ApiShipPlacement, WsQuery};
use crate::api::ws::messages::{ClientMessage, ServerMessage};
use crate::app::game_session::GameSession;
use crate::app::session_manager::SessionManager;
use crate::game::coord::Coord;
use crate::game::errors::GameError;
use crate::game::game_state::Turn;
use crate::game::player::Player;
use crate::game::ship::ShipPlacement;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(id): Path<Uuid>,
    Query(query): Query<WsQuery>,
    State(manager): State<Arc<Mutex<SessionManager>>>,
) -> Result<impl IntoResponse, ApiError> {
    let session_arc = {
        let manager = manager.lock().unwrap();
        manager.get_session(&id).ok_or(ApiError::SessionNotFound)?
    };

    let player = {
        let session = session_arc.lock().unwrap();
        session
            .player_from_token(query.token)
            .ok_or(ApiError::InvalidPlayer)?
    };

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, id, player, manager)))
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

    let (session_arc, mut rx) = match session_opt {
        Some(session_arc) => {
            let rx = {
                let session = session_arc.lock().unwrap();
                session.subscribe()
            };
            (session_arc, rx)
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
        let session = session_arc.lock().unwrap();
        let snapshot = session.snapshot_for(player);

        ServerMessage::GameState {
            turn: session.current_turn(),
            status: session.status(),
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
                                if let Err(e) = handle_fire(session_arc.clone(), player, coord).await {
                                    eprintln!("[WS] error: {e:?}");
                                    let server_msg = map_error_to_message(e);
                                    let error_msg = to_ws_message(server_msg);
                                    let _ = sender.send(error_msg).await;
                                }
                            },

                            Ok(ClientMessage::RandomFleet) => {
                                eprintln!("[WS] Random Fleet requested");
                                let message = handle_random_fleet().await;
                                let _ = sender.send(to_ws_message(message)).await;
                            },

                            Ok(ClientMessage::PlaceFleet { fleet }) => {
                                eprintln!("[WS] Fleet placement requested");
                                match handle_place_fleet(session_arc.clone(), player, fleet).await {
                                    Ok(message) => {
                                        let _ = sender.send(to_ws_message(message)).await;
                                    },
                                    Err(e) => {
                                        eprintln!("[WS] Placement error: {e:?}");
                                        let server_msg = map_error_to_message(e);
                                        let error_msg = to_ws_message(server_msg);
                                        let _ = sender.send(error_msg).await;
                                    }
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
                            let session = session_arc.lock().unwrap();
                            let snapshot = session.snapshot_for(player);

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

async fn handle_random_fleet() -> ServerMessage {
    let fleet = Player::generate_random_fleet();
    let api_fleet: Vec<ApiShipPlacement> = fleet.into_iter().map(Into::into).collect();

    eprintln!("[WS] Random fleet generated");
    ServerMessage::RandomFleet { fleet: api_fleet }
}

async fn handle_place_fleet(
    session_arc: Arc<Mutex<GameSession>>,
    player: Turn,
    fleet: Vec<ApiShipPlacement>,
) -> Result<ServerMessage, ApiError> {
    let placements: Vec<ShipPlacement> = fleet
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, _>>()?;

    let mut session = session_arc.lock().unwrap();
    session.place_fleet(player, placements)?;
    let snapshot = session.snapshot_for(player);

    Ok(ServerMessage::GameState {
        turn: session.current_turn(),
        status: session.status(),
        player_board: snapshot.player_board,
        opponent_board: snapshot.opponent_board,
    })
}

async fn handle_fire(
    session_arc: Arc<Mutex<GameSession>>,
    player: Turn,
    coord: ApiCoord,
) -> Result<(), ApiError> {
    let coord: Coord = coord.try_into()?;

    let mut session = session_arc.lock().unwrap();
    session.player_fire(player, coord)?;

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

fn to_ws_message(message: ServerMessage) -> Message {
    Message::Text(
        serde_json::to_string(&message)
            .unwrap_or_else(|_| "{\"type\":\"error\", \"message\":\"internal\"}".into())
            .into(),
    )
}
