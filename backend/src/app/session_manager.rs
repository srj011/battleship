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

    pub fn create_vs_ai(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        let session = Arc::new(Mutex::new(GameSession::new_vs_ai()));
        self.sessions.insert(id, session);
        id
    }

    pub fn create_multiplayer(&mut self) -> Uuid {
        let id = Uuid::new_v4();
        let session = Arc::new(Mutex::new(GameSession::new_vs_multiplayer()));
        self.sessions.insert(id, session);
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vs_ai_creates_session() {
        let mut manager = SessionManager::new();

        let id = manager.create_vs_ai();

        assert!(manager.get_session(&id).is_some());
    }
}
