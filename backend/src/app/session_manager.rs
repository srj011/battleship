use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::game_session::GameSession;

pub struct SessionManager {
    sessions: HashMap<Uuid, Arc<Mutex<GameSession>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn get_session(&self, id: &Uuid) -> Option<Arc<Mutex<GameSession>>> {
        self.sessions.get(id).cloned()
    }

    pub fn create_vs_ai(&mut self) -> (Uuid, Uuid) {
        let id = Uuid::new_v4();
        let session = Arc::new(Mutex::new(GameSession::new_vs_ai()));

        let player1_token = {
            let session_guard = session.lock().unwrap();
            session_guard.player1_token()
        };

        self.sessions.insert(id, session);

        (id, player1_token)
    }

    pub fn create_multiplayer(&mut self) -> (Uuid, Uuid) {
        let id = Uuid::new_v4();
        let session = Arc::new(Mutex::new(GameSession::new_vs_multiplayer()));

        let player1_token = {
            let session_guard = session.lock().unwrap();
            session_guard.player1_token()
        };

        self.sessions.insert(id, session);

        (id, player1_token)
    }
}
