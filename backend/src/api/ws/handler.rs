use axum::{
    extract::{
        Path, Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt, stream::SplitSink};
use rand::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::broadcast;
use tokio::time::{interval, sleep};
use tracing::{debug, error, info, instrument, warn};

use crate::api::errors::ApiError;
use crate::api::types::{ApiCoord, ApiShipPlacement, WsQuery};
use crate::api::ws::messages::{ClientMessage, ServerMessage};
use crate::app::game_session::{GameSession, GameUpdate};
use crate::app::session_manager::SessionManager;
use crate::game::coord::Coord;
use crate::game::game_state::Turn;
use crate::game::player::Player;
use crate::game::ship::ShipPlacement;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);

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

#[instrument(name = "ws", skip_all, fields(game=%game_code, player=?player))]
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
            warn!("session not found");

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
        info!(event = "reconnected");
    } else {
        info!(event = "connected");
    }

    // Initial message
    let initial_message = {
        let session = session_arc.lock().unwrap();
        let snapshot = session.snapshot_for(player);
        let (player_ready, opponent_ready) = session.ready_status(player);
        let rematch_state = session.rematch_state();

        ServerMessage::GameState {
            player,
            turn: session.current_turn(),
            status: session.status(),
            player_board: snapshot.player_board,
            opponent_board: snapshot.opponent_board,
            player_fleet: snapshot.player_fleet,
            opponent_fleet: snapshot.opponent_fleet,
            opponent_present: snapshot.opponent_present,
            player_ready,
            opponent_ready,
            rematch_state,
        }
    };

    let _ = sender.send(to_ws_message(initial_message)).await;

    let mut last_client_ping = Instant::now();
    let mut heartbeat = interval(HEARTBEAT_INTERVAL);

    // Event loop
    loop {
        tokio::select! {
            // HEARTBEAT
            _ = heartbeat.tick() => {
                if last_client_ping.elapsed() > HEARTBEAT_TIMEOUT {
                    warn!(event="heartbeat_timeout");
                    break;
                }
            }

            // CLIENT -> SERVER
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Ok(ClientMessage::Fire { coord }) => {
                                debug!(action = "fire", row = coord.row, col = coord.col);
                                if let Err(err) = handle_fire(session_arc.clone(), player, coord).await {
                                    error!(?err, "fire handling failed");
                                    let server_msg = map_error_to_message(err);
                                    let error_msg = to_ws_message(server_msg);
                                    send_ws(&mut sender, error_msg).await;
                                }
                            },

                            Ok(ClientMessage::RandomFleet) => {
                                info!(msg="random_fleet", "message received");
                                let message = handle_random_fleet().await;
                                send_ws(&mut sender, to_ws_message(message)).await;
                            },

                            Ok(ClientMessage::PlaceFleet { fleet }) => {
                                info!(msg="place_fleet", "message received");
                                if let Err(err) = handle_place_fleet(session_arc.clone(), player, fleet).await {
                                    error!(?err, "placement error");
                                    let server_msg = map_error_to_message(err);
                                    let error_msg = to_ws_message(server_msg);
                                    send_ws(&mut sender, error_msg).await;
                                }
                            },

                            Ok(ClientMessage::RequestRematch) => {
                                info!(msg="request_rematch", "message received");
                                if let Err(err) = handle_rematch(session_arc.clone(), player).await {
                                    error!(?err, "rematch error");
                                    let server_msg = map_error_to_message(err);
                                    let error_msg = to_ws_message(server_msg);
                                    send_ws(&mut sender, error_msg).await;
                                }
                            }

                            Ok(ClientMessage::CancelRematch) => {
                                info!(msg="cancel_rematch", "message received");
                                {
                                    let mut session = session_arc.lock().unwrap();
                                    session.cancel_rematch(player);
                                }
                            }

                            Ok(ClientMessage::RejectRematch) => {
                                info!(msg="reject_rematch", "message received");
                                {
                                    let mut session = session_arc.lock().unwrap();
                                    session.reject_rematch(player);
                                }
                            }

                            Ok(ClientMessage::LeaveGame) => {
                                info!(msg="leave_game", "message received");
                                {
                                    let mut session = session_arc.lock().unwrap();
                                    session.handle_leave(player);
                                }

                                if let Err(e) = sender.close().await {
                                    debug!(?e, "failed to close socket");
                                }
                                info!(event = "disconnected");
                                return;
                            }

                            Ok(ClientMessage::Ping) => {
                                last_client_ping = Instant::now();
                                debug!(event="ping_received");

                                send_ws(&mut sender, to_ws_message(ServerMessage::Pong)).await;
                                debug!(event="pong_sent");
                            }

                            Err(err) => {
                                warn!(?err, "invalid message");
                                let error_msg = map_error_to_message(ApiError::InvalidMessage);
                                send_ws(&mut sender, to_ws_message(error_msg)).await;
                            }
                        }
                    },
                    Some(Ok(_)) => {},
                    Some(Err(err)) => {
                        error!(?err, "ws error");
                        break;
                    },
                    None => {
                        info!(event = "disconnected");

                        let spawned_at = {
                            let mut session = session_arc.lock().unwrap();
                            session.mark_disconnected(player);
                            session.disconnected_at(player).unwrap()
                        };

                        let session_clone = session_arc.clone();

                        info!(event = "abandon_timer_started", timeout = 30);
                        tokio::spawn(async move {
                            sleep(Duration::from_secs(30)).await;

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

            // SERVER -> CLIENT
            update = rx.recv() => {
                match update {
                    Ok(update) => {
                        let message = {
                            let session = session_arc.lock().unwrap();
                            let snapshot = session.snapshot_for(player);
                            let (player_ready, opponent_ready) = session.ready_status(player);
                            let rematch_state = session.rematch_state();

                            match update {
                                GameUpdate::StateChanged => ServerMessage::GameState {
                                    player,
                                    turn: session.current_turn(),
                                    status: session.status(),
                                    player_board: snapshot.player_board,
                                    opponent_board: snapshot.opponent_board,
                                    player_fleet: snapshot.player_fleet,
                                    opponent_fleet: snapshot.opponent_fleet,
                                    opponent_present: snapshot.opponent_present,
                                    player_ready,
                                    opponent_ready,
                                    rematch_state
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

                                GameUpdate::RematchCancelled{ player } => ServerMessage::RematchCancelled { player },

                                GameUpdate::RematchRejected{ player } => ServerMessage::RematchRejected { player }
                            }
                        };

                        send_ws(&mut sender, to_ws_message(message)).await;
                    },
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        warn!("broadcast lagged");
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

    let should_start_ai = {
        let mut session = session_arc.lock().unwrap();
        session.player_fire(player, coord)?;
        session.is_ai_turn()
    };

    if should_start_ai {
        debug!(event = "ai_turn_scheduled");
        tokio::spawn(run_delayed_ai_turn(session_arc));
    } else {
        debug!(event = "ai_turn_not_scheduled");
    }

    Ok(())
}

async fn run_delayed_ai_turn(session_arc: Arc<Mutex<GameSession>>) {
    debug!(event = "ai_turn_loop_started");

    loop {
        let delay = random_ai_delay();
        debug!(event = "ai_turn_delay_started", delay_ms = delay.as_millis());
        sleep(delay).await;

        let should_continue = {
            let mut session = session_arc.lock().unwrap();

            match session.ai_fire_once() {
                Ok(should_continue) => {
                    debug!(event = "ai_turn_step_completed", should_continue);
                    should_continue
                }
                Err(err) => {
                    error!(?err, "ai fire handling failed");
                    false
                }
            }
        };

        if !should_continue {
            break;
        }
    }

    debug!(event = "ai_turn_loop_finished");
}

fn random_ai_delay() -> Duration {
    let mut rng = rand::rng();
    let delay = Duration::from_millis(rng.random_range(700..=1_600));
    debug!(event = "ai_turn_delay_selected", delay_ms = delay.as_millis());
    delay
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
    let (_, response) = e.to_response();
    let message = response.message;

    ServerMessage::Error {
        message: message.to_string(),
    }
}

fn to_ws_message(message: ServerMessage) -> Message {
    Message::Text(
        serde_json::to_string(&message)
            .unwrap_or_else(|err| {
                warn!(error = ?err, "serde_serialization_failed");
                "{\"type\":\"error\", \"message\":\"internal\"}".into()
            })
            .into(),
    )
}
