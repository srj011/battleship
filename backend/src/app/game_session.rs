use crate::game::ai::AIPlayer;
use crate::game::coord::Coord;
use crate::game::game_state::{GameError, GameState, GameStatus, Turn};
use crate::game::player::{Player, ShotResult};

pub struct TurnOutcome {
    player_result: ShotResult,
    ai_move: Option<(Coord, ShotResult)>,
    status: GameStatus,
}

pub struct GameSession {
    game: GameState,
    ai: Option<AIPlayer>,
}

impl GameSession {
    pub fn new_vs_ai() -> Self {
        let player1 = Player::new();
        let player2 = Player::new();

        let game = GameState::new(player1, player2);
        let ai = Some(AIPlayer::new());
        Self { game, ai }
    }

    pub fn new_vs_multiplayer() -> Self {
        let player1 = Player::new();
        let player2 = Player::new();

        let game = GameState::new(player1, player2);

        Self { game, ai: None }
    }

    pub fn status(&self) -> GameStatus {
        self.game.status()
    }

    pub fn current_turn(&self) -> Turn {
        self.game.current_turn()
    }

    pub fn player_fire(
        &mut self,
        acting_player: Turn,
        coord: Coord,
    ) -> Result<TurnOutcome, GameError> {
        if acting_player != self.game.current_turn() {
            return Err(GameError::NotPlayersTurn);
        }

        let player_result = self.game.take_turn(coord)?;
        let mut ai_move = None;

        if let Some(ai) = &mut self.ai {
            if self.game.status() == GameStatus::Ongoing
                && self.game.current_turn() == Turn::Player2
            {
                let ai_coord = ai.next_shot();
                let ai_result = self.game.take_turn(ai_coord)?;
                ai.process_result(ai_coord, ai_result);

                ai_move = Some((ai_coord, ai_result))
            }
        }

        Ok(TurnOutcome {
            player_result,
            ai_move,
            status: self.game.status(),
        })
    }
}
