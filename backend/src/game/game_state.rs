use serde::{Deserialize, Serialize};

use super::coord::Coord;
use super::errors::GameError;
use super::player::{Player, ShotOutcome, ShotResult};
use super::ship::ShipPlacement;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Turn {
    Player1,
    Player2,
}

impl Turn {
    pub fn opponent(self) -> Self {
        match self {
            Turn::Player1 => Turn::Player2,
            Turn::Player2 => Turn::Player1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GameStatus {
    PlacingShips,
    Ongoing,
    Finished { winner: Turn },
}

pub struct GameState {
    player1: Player,
    player2: Player,
    current_turn: Turn,
    status: GameStatus,
    player1_ready: bool,
    player2_ready: bool,
}

impl GameState {
    pub fn new(player1: Player, player2: Player) -> Self {
        Self {
            player1,
            player2,
            current_turn: Turn::Player1,
            status: GameStatus::PlacingShips,
            player1_ready: false,
            player2_ready: false,
        }
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn current_turn(&self) -> Turn {
        self.current_turn
    }

    pub fn player(&self, player: Turn) -> &Player {
        match player {
            Turn::Player1 => &self.player1,
            Turn::Player2 => &self.player2,
        }
    }

    pub fn place_fleet(
        &mut self,
        player: Turn,
        placements: Vec<ShipPlacement>,
    ) -> Result<(), GameError> {
        if self.status != GameStatus::PlacingShips {
            return Err(GameError::InvalidGameState);
        }

        let target = match player {
            Turn::Player1 => &mut self.player1,
            Turn::Player2 => &mut self.player2,
        };

        match player {
            Turn::Player1 if self.player1_ready => {
                return Err(GameError::InvalidGameState);
            }
            Turn::Player2 if self.player2_ready => {
                return Err(GameError::InvalidGameState);
            }
            _ => {}
        }

        target.place_fleet(placements)?;

        match player {
            Turn::Player1 => self.player1_ready = true,
            Turn::Player2 => self.player2_ready = true,
        }

        if self.player1_ready && self.player2_ready {
            self.status = GameStatus::Ongoing
        }

        Ok(())
    }

    pub fn take_turn(&mut self, coord: Coord) -> Result<ShotOutcome, GameError> {
        if let GameStatus::Finished { winner: _ } = self.status {
            return Err(GameError::GameAlreadyFinished);
        }

        if self.status != GameStatus::Ongoing {
            return Err(GameError::InvalidGameState);
        }

        let result = match self.current_turn {
            Turn::Player1 => self.player2.fire_at(coord),
            Turn::Player2 => self.player1.fire_at(coord),
        };

        let opponent_lost = match self.current_turn {
            Turn::Player1 => self.player2.has_lost(),
            Turn::Player2 => self.player1.has_lost(),
        };

        if opponent_lost {
            self.status = GameStatus::Finished {
                winner: self.current_turn,
            };
            return Ok(result);
        }

        if matches!(result.result, ShotResult::Miss) {
            self.switch_turn();
        }

        Ok(result)
    }

    fn switch_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Turn::Player1 => Turn::Player2,
            Turn::Player2 => Turn::Player1,
        };
    }
}
