use serde::{Deserialize, Serialize};

use crate::api::types::ApiCoord;
use crate::app::game_session::{TurnEvent, TurnOutcome};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Fire { coord: ApiCoord },
}
