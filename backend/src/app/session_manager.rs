use std::collections::HashMap;
use uuid::Uuid;

use super::game_session::GameSession;

pub struct SessionManager {
    sessions: HashMap<Uuid, GameSession>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn get_session(&self, id: &Uuid) -> Option<&GameSession> {
        self.sessions.get(id)
    }

    pub fn get_mut_session(&mut self, id: &Uuid) -> Option<&mut GameSession> {
        self.sessions.get_mut(id)
    }

    pub fn create_vs_ai(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        let session = GameSession::new_vs_ai();

        self.sessions.insert(id, session);
        id
    }

    pub fn create_multiplayer(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        let session = GameSession::new_vs_multiplayer();

        self.sessions.insert(id, session);
        id
    }
}
