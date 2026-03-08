use serde::Serialize;

use crate::game::ai::AiPlayer;
use crate::game::coord::Coord;
use crate::game::game_state::{GameError, GameState, GameStatus, Turn};
use crate::game::player::{Player, ShotResult};

#[derive(Clone, Copy, Serialize)]
pub struct TurnEvent {
    player: Turn,
    coord: Coord,
    result: ShotResult,
}

impl TurnEvent {
    pub fn new(player: Turn, coord: Coord, result: ShotResult) -> Self {
        Self {
            player,
            coord,
            result,
        }
    }
}

#[derive(Serialize)]
pub struct TurnOutcome {
    events: Vec<TurnEvent>,
    status: GameStatus,
}

#[derive(Serialize)]
pub struct GameSnapshot {
    turn: Turn,
    history: Vec<TurnEvent>,
    status: GameStatus,
}

pub struct GameSession {
    game: GameState,
    ai: Option<AiPlayer>,
    history: Vec<TurnEvent>,
}

impl GameSession {
    pub fn new_vs_ai() -> Self {
        let mut player1 = Player::new();
        let mut player2 = Player::new();

        player1.place_random_ships();
        player2.place_random_ships();

        let game = GameState::new(player1, player2);
        let ai = Some(AiPlayer::new());
        Self {
            game,
            ai,
            history: Vec::new(),
        }
    }

    pub fn new_vs_multiplayer() -> Self {
        let player1 = Player::new();
        let player2 = Player::new();

        let game = GameState::new(player1, player2);

        Self {
            game,
            ai: None,
            history: Vec::new(),
        }
    }

    pub fn status(&self) -> GameStatus {
        self.game.status()
    }

    pub fn current_turn(&self) -> Turn {
        self.game.current_turn()
    }

    pub fn events(&self) -> &[TurnEvent] {
        &self.history
    }

    pub fn snapshot(&self) -> GameSnapshot {
        GameSnapshot {
            turn: self.current_turn(),
            history: self.history.clone(),
            status: self.status(),
        }
    }

    pub fn player_fire(
        &mut self,
        acting_player: Turn,
        coord: Coord,
    ) -> Result<TurnOutcome, GameError> {
        if acting_player != self.game.current_turn() {
            return Err(GameError::NotPlayersTurn);
        }

        let mut events = Vec::new();

        // Player turn
        self.record_turn(&mut events, acting_player, coord)?;

        // AI turn
        if let Some(mut ai) = self.ai.take() {
            while self.game.status() == GameStatus::Ongoing
                && self.game.current_turn() == Turn::Player2
            {
                let ai_coord = ai.next_shot();
                let ai_event = self.record_turn(&mut events, Turn::Player2, ai_coord)?;

                if ai_event.result == ShotResult::AlreadyShot {
                    panic!("AI fired at an already-shot cell {ai_coord:?}");
                }

                ai.process_result(ai_coord, ai_event.result);

                if ai_event.result == ShotResult::Miss {
                    break;
                }
            }

            self.ai = Some(ai);
        }

        Ok(TurnOutcome {
            events,
            status: self.game.status(),
        })
    }

    fn record_turn(
        &mut self,
        events: &mut Vec<TurnEvent>,
        player: Turn,
        coord: Coord,
    ) -> Result<TurnEvent, GameError> {
        let event = TurnEvent::new(player, coord, self.game.take_turn(coord)?);
        events.push(event);
        self.history.push(event);

        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use rand::RngExt;

    use super::*;
    use crate::game::coord::Coord;
    use crate::game::game_state::Turn;

    #[test]
    fn player_fire_creates_event() {
        let mut session = GameSession::new_vs_ai();

        let result = session
            .player_fire(Turn::Player1, Coord::new(0, 0))
            .unwrap();

        assert!(!result.events.is_empty());
    }

    #[test]
    fn event_history_grows_after_turn() {
        let mut session = GameSession::new_vs_ai();

        session
            .player_fire(Turn::Player1, Coord::new(0, 0))
            .unwrap();

        assert!(!session.events().is_empty());
    }
    #[test]
    fn full_game_simulation_completes() {
        use crate::game::board::BOARD_SIZE;

        let mut rng = rand::rng();
        let mut session = GameSession::new_vs_ai();

        for _ in 0..500 {
            if session.status() == GameStatus::Finished {
                return;
            }

            if session.current_turn() == Turn::Player1 {
                let coord = Coord::new(
                    rng.random_range(0..BOARD_SIZE),
                    rng.random_range(0..BOARD_SIZE),
                );

                let _ = session.player_fire(Turn::Player1, coord);
            }
        }

        panic!("Game did not finish within expected number of turns");
    }
}
