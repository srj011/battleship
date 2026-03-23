use serde::Serialize;
use tokio::sync::broadcast;
use uuid::Uuid;

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
    pub player_board: BoardView,
    pub opponent_board: BoardView,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct GameUpdate {
    pub event: TurnEvent,
    pub turn: Turn,
    pub status: GameStatus,
}

pub struct GameSession {
    game: GameState,
    ai: Option<AiPlayer>,
    history: Vec<TurnEvent>,
    tx: broadcast::Sender<GameUpdate>,
    player1_token: Uuid,
    player2_token: Option<Uuid>,
}

impl GameSession {
    pub fn new_vs_ai() -> Self {
        let player1 = Player::new();
        let player1_token = Uuid::new_v4();

        let player2 = Player::new();

        let (tx, _) = broadcast::channel(32);
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
            tx,
            player1_token,
            player2_token: None,
        }
    }

    pub fn new_vs_multiplayer() -> Self {
        let player1 = Player::new();
        let player1_token = Uuid::new_v4();

        let player2 = Player::new();

        let (tx, _) = broadcast::channel(32);

        let game = GameState::new(player1, player2);

        Self {
            game,
            ai: None,
            history: Vec::new(),
            tx,
            player1_token,
            player2_token: None,
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

    pub fn subscribe(&self) -> broadcast::Receiver<GameUpdate> {
        self.tx.subscribe()
    }

    pub fn player1_token(&self) -> Uuid {
        self.player1_token
    }

    pub fn player2_token(&self) -> Option<Uuid> {
        self.player2_token
    }

    pub fn player_from_token(&self, token: Uuid) -> Option<Turn> {
        if token == self.player1_token {
            Some(Turn::Player1)
        } else if Some(token) == self.player2_token {
            Some(Turn::Player2)
        } else {
            None
        }
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
        // REST API helper method
        let mut events = Vec::new();

        // Player turn
        let update = self.fire_once(acting_player, coord)?;
        events.push(update.event);

        // AI turn
        self.ai_turn()?;

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

        let update = GameUpdate {
            event,
            turn: self.game.current_turn(),
            status: self.game.status(),
        };

        let _ = self.tx.send(update);
        Ok(update)
    }

    pub fn ai_turn(&mut self) -> Result<(), GameError> {
        let Some(mut ai) = self.ai.take() else {
            return Ok(());
        };

        while self.game.status() == GameStatus::Ongoing && self.game.current_turn() == Turn::Player2
        {
            let coord = ai.next_shot();
            let update = self.fire_once(Turn::Player2, coord)?;
            let event = update.event;

            if event.result == ShotResult::AlreadyShot {
                panic!("AI fired at an already-shot cell {coord:?}");
            }

            ai.process_result(coord, event.result);

            if event.result == ShotResult::Miss {
                break;
            }
        }
        self.ai = Some(ai);

        Ok(())
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
