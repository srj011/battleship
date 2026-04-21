use axum::{
    extract::{
        Path, Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt, stream::SplitSink};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast;

use crate::api::errors::ApiError;
use crate::api::types::{ApiCoord, ApiShipPlacement, WsQuery};
use crate::api::ws::messages::{ClientMessage, ServerMessage};
use crate::app::game_session::{GameSession, GameUpdate};
use crate::app::session_manager::SessionManager;
use crate::game::coord::Coord;
use crate::game::game_state::{GameStatus, Turn};
use crate::game::player::Player;
use crate::game::ship::ShipPlacement;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(code): Path<String>,
    Query(query): Query<WsQuery>,
    State(manager): State<Arc<Mutex<SessionManager>>>,
) -> Result<impl IntoResponse, ApiError> {
    let session_arc = {
        let manager = manager.lock().unwrap();
        manager
            .get_session(&code)
            .ok_or(ApiError::SessionNotFound)?
    };

    let player = {
        let session = session_arc.lock().unwrap();
        session
            .player_from_token(query.player_token)
            .ok_or(ApiError::InvalidPlayer)?
    };

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, code, player, manager)))
}

async fn handle_socket(
    socket: WebSocket,
    game_code: String,
    player: Turn,
    manager: Arc<Mutex<SessionManager>>,
) {
    let (mut sender, mut receiver) = socket.split();

    // Get Session
    let session_opt = {
        let manager = manager.lock().unwrap();
        manager.get_session(&game_code)
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

            send_ws(&mut sender, to_ws_message(error_msg)).await;
            return;
        }
    };

    let is_reconnect = {
        let mut session = session_arc.lock().unwrap();
        if session.is_disconnected(player) {
            session.mark_reconnected(player);
            true
        } else {
            false
        }
    };

    if is_reconnect {
        eprintln!("[WS] Connected: {game_code} as {player:?}");
    } else {
        eprintln!("[WS] Reconnected: {game_code} as {player:?}");
    }

    // Initial message
    let initial_message = {
        let session = session_arc.lock().unwrap();
        let snapshot = session.snapshot_for(player);
        let (player_ready, opponent_ready) = session.ready_status(player);
        let (player_rematch_ready, opponent_rematch_ready) = session.rematch_status(player);

        ServerMessage::GameState {
            player,
            turn: session.current_turn(),
            status: session.status(),
            player_board: snapshot.player_board,
            opponent_board: snapshot.opponent_board,
            player_fleet: snapshot.player_fleet,
            opponent_fleet: snapshot.opponent_fleet,
            opponent_joined: snapshot.opponent_joined,
            player_ready,
            opponent_ready,
            player_rematch_ready,
            opponent_rematch_ready,
        }
    };

    let _ = sender.send(to_ws_message(initial_message)).await;

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
                                    send_ws(&mut sender, error_msg).await;
                                }
                            },

                            Ok(ClientMessage::RandomFleet) => {
                                eprintln!("[WS] Random Fleet requested");
                                let message = handle_random_fleet().await;
                                send_ws(&mut sender, to_ws_message(message)).await;
                            },

                            Ok(ClientMessage::PlaceFleet { fleet }) => {
                                eprintln!("[WS] Fleet placement requested");
                                if let Err(err) = handle_place_fleet(session_arc.clone(), player, fleet).await {
                                    eprintln!("[WS] Placement error: {e:?}");
                                    let server_msg = map_error_to_message(err);
                                    let error_msg = to_ws_message(server_msg);
                                    send_ws(&mut sender, error_msg).await;
                                }
                            },

                            Ok(ClientMessage::Restart) => {
                                eprintln!("[WS] Restart requested");
                                if let Err(e) = handle_rematch(session_arc.clone(), player).await {
                                    eprintln!("[WS] Restart error");
                                    let server_msg = map_error_to_message(e);
                                    let error_msg = to_ws_message(server_msg);
                                    send_ws(&mut sender, error_msg).await;
                                }
                            }

                            Ok(ClientMessage::LeaveGame) => {
                                {
                                    let mut session = session_arc.lock().unwrap();
                                    session.handle_leave(player);
                                }

                                if let Err(e) = sender.close().await {
                                    debug!(?e, "failed to close socket");
                                }
                                return;
                            }

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
                        info!(event = "disconnected");

                        {
                            let mut session = session_arc.lock().unwrap();
                            if matches!(
                                session.status(),
                                GameStatus::Finished{ .. } | GameStatus::Abandoned{ .. }
                            ) {
                                session.handle_leave(player);
                                break;
                            }
                        }

                        let spawned_at = {
                            let mut session = session_arc.lock().unwrap();
                            session.mark_disconnected(player);
                            session.disconnected_at(player).unwrap()
                        };

                        let session_clone = session_arc.clone();

                        info!(event = "abandon_timer_started", timeout = 30);
                        tokio::spawn(async move {
                            time::sleep(Duration::from_secs(30)).await;

                            let mut session = session_clone.lock().unwrap();

                            if let Some(disconnected_at) = session.disconnected_at(player) {
                                if spawned_at == disconnected_at {
                                    session.handle_leave(player);
                                }
                            }
                        });

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
                            let (player_ready, opponent_ready) = session.ready_status(player);
                            let (player_rematch_ready, opponent_rematch_ready) = session.rematch_status(player);

                            match update {
                                GameUpdate::StateChanged => ServerMessage::GameState {
                                    player,
                                    turn: session.current_turn(),
                                    status: session.status(),
                                    player_board: snapshot.player_board,
                                    opponent_board: snapshot.opponent_board,
                                    player_fleet: snapshot.player_fleet,
                                    opponent_fleet: snapshot.opponent_fleet,
                                    opponent_joined: snapshot.opponent_joined,
                                    player_ready,
                                    opponent_ready,
                                    player_rematch_ready,
                                    opponent_rematch_ready,
                                },
                                GameUpdate::ShotFired{ event } => ServerMessage::GameUpdate {
                                    event,
                                    turn: session.current_turn(),
                                    status: session.status(),
                                    player_board: snapshot.player_board,
                                    opponent_board: snapshot.opponent_board,
                                },

                                GameUpdate::PlayerDisconnected{ info } => ServerMessage::PlayerDisconnected { info },

                                GameUpdate::PlayerReconnected{ player } => ServerMessage::PlayerReconnected { player },
                            }
                        };

                        send_ws(&mut sender, to_ws_message(message)).await;
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
}

async fn send_ws(sender: &mut SplitSink<WebSocket, Message>, msg: Message) {
    if let Err(e) = sender.send(msg).await {
        debug!(?e, "send failed");
    }
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
) -> Result<(), ApiError> {
    let placements: Vec<ShipPlacement> = fleet
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, _>>()?;

    let mut session = session_arc.lock().unwrap();
    session.place_fleet(player, placements)?;

    Ok(())
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

async fn handle_rematch(
    session_arc: Arc<Mutex<GameSession>>,
    player: Turn,
) -> Result<(), ApiError> {
    let mut session = session_arc.lock().unwrap();
    session.request_rematch(player)?;
    Ok(())
}

fn map_error_to_message(e: ApiError) -> ServerMessage {
    let (_, message) = e.status_message();

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
