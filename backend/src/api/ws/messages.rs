use serde::{Deserialize, Serialize};

use crate::api::types::{ApiCoord, ApiShipPlacement};
use crate::app::board_view::BoardView;
use crate::app::game_session::TurnEvent;
use crate::game::game_state::{GameStatus, Turn};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Fire { coord: ApiCoord },
    RandomFleet,
    PlaceFleet { fleet: Vec<ApiShipPlacement> },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    GameState {
        turn: Turn,
        status: GameStatus,
        player_board: BoardView,
        opponent_board: BoardView,
    },
    GameUpdate {
        event: TurnEvent,
        turn: Turn,
        status: GameStatus,
        player_board: BoardView,
        opponent_board: BoardView,
    },

    RandomFleet {
        fleet: Vec<ApiShipPlacement>,
    },

    Error {
        message: String,
    },
}
