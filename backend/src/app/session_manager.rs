use rand::RngExt;
use rand::distr::Alphanumeric;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::game_session::GameSession;

pub struct SessionManager {
    sessions: HashMap<Uuid, Arc<Mutex<GameSession>>>,
    game_codes: HashMap<String, Uuid>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            game_codes: HashMap::new(),
        }
    }

    pub fn get_session(&self, id: &Uuid) -> Option<Arc<Mutex<GameSession>>> {
        self.sessions.get(id).cloned()
    }

    pub fn get_session_by_code(&self, code: &str) -> Option<Arc<Mutex<GameSession>>> {
        self.game_codes
            .get(code)
            .and_then(|id| self.get_session(&id))
    }

    pub fn create_vs_ai(&mut self) -> (String, Uuid) {
        let id = Uuid::new_v4();
        let (session, player1_token) = GameSession::new_vs_ai();
        let session_arc = Arc::new(Mutex::new(session));

        let code = loop {
            let code = Self::generate_code();
            if !self.game_codes.contains_key(&code) {
                break code;
            }
        };
        self.game_codes.insert(code.clone(), id);
        self.sessions.insert(id, session_arc);

        (code, player1_token)
    }

    pub fn create_multiplayer(&mut self) -> (String, Uuid) {
        let id = Uuid::new_v4();
        let (session, player1_token) = GameSession::new_vs_multiplayer();
        let session_arc = Arc::new(Mutex::new(session));

        let code = loop {
            let code = Self::generate_code();
            if !self.game_codes.contains_key(&code) {
                break code;
            }
        };
        self.game_codes.insert(code.clone(), id);
        self.sessions.insert(id, session_arc);

        (code, player1_token)
    }

    fn generate_code() -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect()
    }
}
