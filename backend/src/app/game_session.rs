use serde::Serialize;

use crate::app::board_view::{BoardPerspective, BoardView};
use crate::game::ai::AiPlayer;
use crate::game::coord::Coord;
use crate::game::errors::GameError;
use crate::game::game_state::{GameState, GameStatus, Turn};
use crate::game::player::{Player, ShotResult};
use crate::game::ship::ShipPlacement;

#[derive(Debug, Clone, Copy, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct TurnOutcome {
    events: Vec<TurnEvent>,
    status: GameStatus,
}

#[derive(Serialize)]
pub struct GameSnapshot {
    turn: Turn,
    history: Vec<TurnEvent>,
    status: GameStatus,
    player_board: BoardView,
    opponent_board: BoardView,
}

#[derive(Clone, Copy, Serialize)]
pub struct GameUpdate {
    event: TurnEvent,
    turn: Turn,
    status: GameStatus,
}

pub struct GameSession {
    game: GameState,
    ai: Option<AiPlayer>,
    history: Vec<TurnEvent>,
}

impl GameSession {
    pub fn new_vs_ai() -> Self {
        let player1 = Player::new();
        let player2 = Player::new();
        let mut game = GameState::new(player1, player2);

        let ai = Some(AiPlayer::new());
        // AI Fleet
        let ai_fleet = Player::generate_random_fleet();
        game.place_fleet(Turn::Player2, ai_fleet)
            .expect("AI fleet placement failed");

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

    pub fn snapshot_for(&self, viewer: Turn) -> GameSnapshot {
        let player = self.game.player(viewer);
        let opponent = self.game.player(viewer.opponent());
        GameSnapshot {
            turn: self.current_turn(),
            history: self.history.clone(),
            status: self.status(),
            player_board: BoardView::new(player.board(), BoardPerspective::Owner),
            opponent_board: BoardView::new(opponent.board(), BoardPerspective::Opponent),
        }
    }

    pub fn place_fleet(
        &mut self,
        player: Turn,
        placements: Vec<ShipPlacement>,
    ) -> Result<(), GameError> {
        self.game.place_fleet(player, placements)
    }

    pub fn player_fire(
        &mut self,
        acting_player: Turn,
        coord: Coord,
    ) -> Result<TurnOutcome, GameError> {
        let mut events = Vec::new();

        // Player turn
        let update = self.fire_once(acting_player, coord)?;
        events.push(update.event);

        // AI turn
        if let Some(mut ai) = self.ai.take() {
            while self.game.status() == GameStatus::Ongoing
                && self.game.current_turn() == Turn::Player2
            {
                let ai_coord = ai.next_shot();
                let update = self.fire_once(Turn::Player2, ai_coord)?;
                let ai_event = update.event;
                events.push(ai_event);

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

    pub fn fire_once(
        &mut self,
        acting_player: Turn,
        coord: Coord,
    ) -> Result<GameUpdate, GameError> {
        if acting_player != self.game.current_turn() {
            return Err(GameError::NotPlayersTurn);
        }

        let mut events = Vec::with_capacity(1);
        let event = self.record_turn(&mut events, acting_player, coord)?;

        Ok(GameUpdate {
            event,
            turn: self.game.current_turn(),
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
