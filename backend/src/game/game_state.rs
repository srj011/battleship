use serde::{Deserialize, Serialize};

use super::coord::Coord;
use super::errors::GameError;
use super::player::{Player, ShipPlacement, ShotResult};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Turn {
    Player1,
    Player2,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GameStatus {
    PlacingShips,
    Ongoing,
    Winner(Turn),
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
            status: GameStatus::Ongoing,
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

    pub fn take_turn(&mut self, coord: Coord) -> Result<ShotResult, GameError> {
        if let GameStatus::Winner(_) = self.status {
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
            self.status = GameStatus::Winner(self.current_turn);
            return Ok(result);
        }

        if matches!(result, ShotResult::Miss) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::coord::Coord;
    use crate::game::ship::{Direction, ShipType};

    fn setup_players() -> (Player, Player) {
        let mut p1 = Player::new();
        let mut p2 = Player::new();

        p1.place_ship(
            ShipType::PatrolBoat,
            Coord::new(0, 0),
            Direction::Horizontal,
        )
        .unwrap();

        p2.place_ship(ShipType::Carrier, Coord::new(0, 0), Direction::Horizontal)
            .unwrap();

        (p1, p2)
    }

    #[test]
    fn turn_switches_on_miss() {
        let (p1, p2) = setup_players();
        let mut game = GameState::new(p1, p2);

        // Player1 fires miss
        let _ = game.take_turn(Coord::new(5, 5));

        // Next turn should belong to Player2
        assert!(matches!(game.current_turn, Turn::Player2));
    }

    #[test]
    fn game_ends_when_ship_destroyed() {
        let (p1, p2) = setup_players();
        let mut game = GameState::new(p1, p2);

        let _ = game.take_turn(Coord::new(0, 0));
        let _ = game.take_turn(Coord::new(0, 1));

        assert!(matches!(game.status, GameStatus::Winner(Turn::Player1)));
    }
}
