use serde::{Deserialize, Serialize};

use crate::api::types::ApiCoord;
use crate::app::board_view::BoardView;
use crate::app::game_session::TurnEvent;
use crate::game::game_state::{GameStatus, Turn};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Fire { coord: ApiCoord },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    GameUpdate {
        event: TurnEvent,
        turn: Turn,
        status: GameStatus,
        player_board: BoardView,
        opponent_board: BoardView,
    },

    Error {
        message: String,
    },
}
