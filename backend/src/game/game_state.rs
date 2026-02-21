use super::coord::Coord;
use super::player::{Player, ShotResult};

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

impl GameState {
    pub fn new(player1: Player, player2: Player) -> Self {
        Self {
            player1,
            player2,
            turn: Turn::Player1,
            status: GameStatus::Ongoing,
        }
    }

    pub fn take_turn(&mut self, coord: Coord) -> ShotResult {
        if let GameStatus::Finished = self.status {
            panic!("Game is already finished");
        }

        let result = match self.turn {
            Turn::Player1 => self.player2.fire_at(coord),
            Turn::Player2 => self.player1.fire_at(coord),
        };

        let opponent_lost = match self.turn {
            Turn::Player1 => self.player2.has_lost(),
            Turn::Player2 => self.player1.has_lost(),
        };

        if opponent_lost {
            self.status = GameStatus::Finished;
            return result;
        }

        if matches!(result, ShotResult::Miss) {
            self.switch_turn();
        }

        result
    }

    fn switch_turn(&mut self) {
        self.turn = match self.turn {
            Turn::Player1 => Turn::Player2,
            Turn::Player2 => Turn::Player1,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::coord::Coord;
    use crate::game::ship::Direction;

    fn setup_players() -> (Player, Player) {
        let mut p1 = Player::new();
        let mut p2 = Player::new();

        p1.place_ship(Coord { row: 0, col: 0 }, 2, Direction::Horizontal)
            .unwrap();

        p2.place_ship(Coord { row: 0, col: 0 }, 2, Direction::Horizontal)
            .unwrap();

        (p1, p2)
    }

    #[test]
    fn turn_switches_on_miss() {
        let (p1, p2) = setup_players();
        let mut game = GameState::new(p1, p2);

        // Player1 fires miss
        let _ = game.take_turn(Coord { row: 5, col: 5 });

        // Next turn should belong to Player2
        assert!(matches!(game.turn, Turn::Player2));
    }

    #[test]
    fn game_ends_when_ship_destroyed() {
        let (p1, p2) = setup_players();
        let mut game = GameState::new(p1, p2);

        game.take_turn(Coord { row: 0, col: 0 });
        game.take_turn(Coord { row: 0, col: 1 });

        assert!(matches!(game.status, GameStatus::Finished));
    }
}
