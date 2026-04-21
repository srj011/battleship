use serde::{Deserialize, Serialize};

use crate::api::types::{ApiCoord, ApiShipPlacement};
use crate::app::board_view::BoardView;
use crate::app::fleet_view::FleetView;
use crate::app::game_session::{DisconnectInfo, TurnEvent};
use crate::game::game_state::{GameStatus, Turn};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Fire { coord: ApiCoord },
    RandomFleet,
    PlaceFleet { fleet: Vec<ApiShipPlacement> },
    LeaveGame,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    GameState {
        player: Turn,
        turn: Turn,
        status: GameStatus,
        player_board: BoardView,
        opponent_board: BoardView,
        player_fleet: FleetView,
        opponent_fleet: FleetView,
        opponent_joined: bool,
        player_ready: bool,
        opponent_ready: bool,
        player_rematch_ready: bool,
        opponent_rematch_ready: bool,
    },
    GameUpdate {
        event: TurnEvent,
        turn: Turn,
        status: GameStatus,
        player_board: BoardView,
        opponent_board: BoardView,
    },

    PlayerDisconnected {
        info: DisconnectInfo,
    },

    PlayerReconnected {
        player: Turn,
    },

    RandomFleet {
        fleet: Vec<ApiShipPlacement>,
    },

    RematchStatus {
        self_ready: bool,
        opponent_ready: bool,
    },

    Error {
        message: String,
    },
}
