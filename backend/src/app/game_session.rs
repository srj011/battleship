use serde::Serialize;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::app::board_view::{BoardPerspective, BoardView};
use crate::app::fleet_view::{FleetPerspective, FleetView};
use crate::game::ai::AiPlayer;
use crate::game::coord::Coord;
use crate::game::errors::GameError;
use crate::game::game_state::{GameState, GameStatus, Turn};
use crate::game::player::{Player, ShotOutcome, ShotResult};
use crate::game::ship::ShipPlacement;

#[derive(Debug, Clone, Serialize)]
pub struct TurnEvent {
    player: Turn,
    coord: Coord,
    outcome: ShotOutcome,
}

impl TurnEvent {
    pub fn new(player: Turn, coord: Coord, outcome: ShotOutcome) -> Self {
        Self {
            player,
            coord,
            outcome,
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
    pub player_fleet: FleetView,
    pub opponent_fleet: FleetView,
}

#[derive(Debug, Clone)]
pub enum GameUpdate {
    StateChanged,
    ShotFired { event: TurnEvent },
}

#[derive(Debug)]
pub enum PlayerSlot {
    Empty,
    Human { token: Uuid },
    AI,
}

pub struct GameSession {
    game: GameState,
    player1: PlayerSlot,
    player2: PlayerSlot,
    ai: Option<AiPlayer>,
    history: Vec<TurnEvent>,
    tx: broadcast::Sender<GameUpdate>,
    player1_wants_rematch: bool,
    player2_wants_rematch: bool,
}

impl GameSession {
    pub fn new_vs_ai() -> (Self, Uuid) {
        let player1_state = Player::new();
        let player1_token = Uuid::new_v4();
        let player1 = PlayerSlot::Human {
            token: player1_token,
        };

        let player2_state = Player::new();
        let player2 = PlayerSlot::AI;

        let (tx, _) = broadcast::channel(32);
        let mut game = GameState::new(player1_state, player2_state);

        // AI Player
        let ai = Some(AiPlayer::new());
        let ai_fleet = Player::generate_random_fleet();
        game.place_fleet(Turn::Player2, ai_fleet)
            .expect("AI fleet placement failed");

        (
            Self {
                game,
                player1,
                player2,
                ai,
                history: Vec::new(),
                tx,
                player1_wants_rematch: false,
                player2_wants_rematch: false,
            },
            player1_token,
        )
    }

    pub fn new_vs_multiplayer() -> (Self, Uuid) {
        let player1_state = Player::new();
        let player1_token = Uuid::new_v4();
        let player1 = PlayerSlot::Human {
            token: player1_token,
        };

        let player2_state = Player::new();
        let player2 = PlayerSlot::Empty;

        let (tx, _) = broadcast::channel(32);

        let game = GameState::new(player1_state, player2_state);

        (
            Self {
                game,
                player1,
                player2,
                ai: None,
                history: Vec::new(),
                tx,
                player1_wants_rematch: false,
                player2_wants_rematch: false,
            },
            player1_token,
        )
    }

    fn restart_game(&mut self) -> Result<(), GameError> {
        match self.game.status() {
            GameStatus::Finished { .. } | GameStatus::Abandoned { .. } => {}
            _ => return Err(GameError::InvalidGameState),
        }

        let player1_state = Player::new();
        let player2_state = Player::new();
        self.game = GameState::new(player1_state, player2_state);

        self.history.clear();

        if self.ai.is_some() {
            self.ai = Some(AiPlayer::new());
            let ai_fleet = Player::generate_random_fleet();
            self.game
                .place_fleet(Turn::Player2, ai_fleet)
                .expect("AI fleet placement failed");
        }

        self.player1_wants_rematch = false;
        self.player2_wants_rematch = false;

        let _ = self.tx.send(GameUpdate::StateChanged);
        Ok(())
    }

    pub fn request_rematch(&mut self, player: Turn) -> Result<bool, GameError> {
        match self.game.status() {
            GameStatus::Finished { .. } | GameStatus::Abandoned { .. } => {}
            _ => return Err(GameError::InvalidGameState),
        }

        // vs AI mode
        if self.ai.is_some() {
            self.restart_game()?;
            return Ok(true);
        }

        // Multiplayer mode
        match player {
            Turn::Player1 => self.player1_wants_rematch = true,
            Turn::Player2 => self.player2_wants_rematch = true,
        }

        let both_ready = self.player1_wants_rematch && self.player2_wants_rematch;
        if both_ready {
            self.restart_game()?;
        }

        let _ = self.tx.send(GameUpdate::StateChanged);
        Ok(both_ready)
    }

    pub fn rematch_status(&self, player: Turn) -> (bool, bool) {
        match player {
            Turn::Player1 => (self.player1_wants_rematch, self.player2_wants_rematch),
            Turn::Player2 => (self.player2_wants_rematch, self.player1_wants_rematch),
        }
    }

    pub fn status(&self) -> GameStatus {
        self.game.status()
    }

    pub fn current_turn(&self) -> Turn {
        self.game.current_turn()
    }

    pub fn player1(&self) -> &PlayerSlot {
        &self.player1
    }

    pub fn player2(&self) -> &PlayerSlot {
        &self.player2
    }

    pub fn events(&self) -> &[TurnEvent] {
        &self.history
    }

    pub fn subscribe(&self) -> broadcast::Receiver<GameUpdate> {
        self.tx.subscribe()
    }

    pub fn ready_status(&self, player: Turn) -> (bool, bool) {
        match player {
            Turn::Player1 => (self.game.player1_ready(), self.game.player2_ready()),
            Turn::Player2 => (self.game.player2_ready(), self.game.player1_ready()),
        }
    }

    pub fn join_player(&mut self) -> Result<Uuid, GameError> {
        match self.player2 {
            PlayerSlot::Empty => {
                let player_token = Uuid::new_v4();
                self.player2 = PlayerSlot::Human {
                    token: player_token,
                };
                Ok(player_token)
            }
            _ => Err(GameError::GameFull),
        }
    }

    pub fn player_from_token(&self, token: Uuid) -> Option<Turn> {
        match (&self.player1, &self.player2) {
            (PlayerSlot::Human { token: t1 }, _) if *t1 == token => Some(Turn::Player1),
            (_, PlayerSlot::Human { token: t2 }) if *t2 == token => Some(Turn::Player2),
            _ => None,
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
            player_fleet: FleetView::from_fleet(player.ships(), FleetPerspective::Owner),
            opponent_fleet: FleetView::from_fleet(opponent.ships(), FleetPerspective::Opponent),
        }
    }

    pub fn place_fleet(
        &mut self,
        player: Turn,
        placements: Vec<ShipPlacement>,
    ) -> Result<(), GameError> {
        self.game.place_fleet(player, placements)?;

        let _ = self.tx.send(GameUpdate::StateChanged);
        Ok(())
    }

    pub fn player_fire(
        &mut self,
        acting_player: Turn,
        coord: Coord,
    ) -> Result<TurnOutcome, GameError> {
        let mut events = Vec::new();

        // Player turn
        let event = self.fire_once(acting_player, coord)?;
        events.push(event);

        // AI turn
        self.ai_turn()?;

        Ok(TurnOutcome {
            events,
            status: self.game.status(),
        })
    }

    pub fn fire_once(&mut self, acting_player: Turn, coord: Coord) -> Result<TurnEvent, GameError> {
        if acting_player != self.game.current_turn() {
            return Err(GameError::NotPlayersTurn);
        }

        let event = self.record_turn(acting_player, coord)?;

        let _ = self.tx.send(GameUpdate::ShotFired {
            event: event.clone(),
        });
        Ok(event)
    }

    pub fn ai_turn(&mut self) -> Result<(), GameError> {
        let Some(mut ai) = self.ai.take() else {
            return Ok(());
        };

        while self.game.status() == GameStatus::Ongoing && self.game.current_turn() == Turn::Player2
        {
            let coord = ai.next_shot();
            let event = self.fire_once(Turn::Player2, coord)?;

            debug_assert!(event.outcome.result != ShotResult::AlreadyShot);

            ai.process_result(coord, &event.outcome);

            if event.outcome.result == ShotResult::Miss {
                break;
            }
        }
        self.ai = Some(ai);

        Ok(())
    }

    fn record_turn(&mut self, player: Turn, coord: Coord) -> Result<TurnEvent, GameError> {
        let outcome = self.game.take_turn(coord)?;
        let event = TurnEvent::new(player, coord, outcome);
        self.history.push(event.clone());

        Ok(event)
    }
}
