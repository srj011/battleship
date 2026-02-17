use super::player::Player;

pub enum Turn {
    Player1,
    Player2,
}

pub enum GameStatus {
    Ongoing,
    Finished,
}

pub struct GameState {
    player1: Player,
    player2: Player,
    turn: Turn,
    status: GameStatus,
}
