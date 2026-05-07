use rand::RngExt;
use rand::distr::slice::Choose;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::info;
use uuid::Uuid;

use super::game_session::{GameMode, GameSession};
use crate::game::game_state::GameStatus;

const CODE_CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";

pub struct SessionManager {
    sessions: HashMap<String, Arc<Mutex<GameSession>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn get_session(&self, code: &str) -> Option<Arc<Mutex<GameSession>>> {
        self.sessions.get(code).cloned()
    }

    pub fn create_session(&mut self, mode: GameMode) -> (String, Uuid) {
        let code = loop {
            let code = Self::generate_code();
            if !self.sessions.contains_key(&code) {
                break code;
            }
        };

        let (session, player1_token) = GameSession::new(code.clone(), mode);
        let session_arc = Arc::new(Mutex::new(session));
        self.sessions.insert(code.clone(), session_arc);

        (code, player1_token)
    }

    fn generate_code() -> String {
        let dist = Choose::new(CODE_CHARS).expect("Code chars array cannot be empty");
        rand::rng()
            .sample_iter(dist)
            .take(6)
            .map(|b| *b as char)
            .collect()
    }

    pub fn cleanup(&mut self) {
        let now = Instant::now();
        let inactivity_timeout = Duration::from_secs(1200); // 20 mins
        let max_age = Duration::from_secs(3600); // 1 hr

        self.sessions.retain(|code, session_arc| {
            let session = session_arc.lock().unwrap();

            if matches!(
                session.status(),
                GameStatus::Finished { .. } | GameStatus::Abandoned { .. }
            ) || now.duration_since(session.last_activity()) > inactivity_timeout
                || now.duration_since(session.created_at()) > max_age
            {
                info!(event = "session_removed", game = %code);
                false
            } else {
                true
            }
        });
    }

    pub fn count(&self) -> usize {
        self.sessions.len()
    }
}
