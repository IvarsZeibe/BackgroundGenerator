use std::{collections::HashMap, sync::Mutex};

use rand::{distributions::Alphanumeric, Rng};

pub struct Sessions {
    pub sessions_to_users: Mutex<HashMap<String, i32>>,
}

impl Sessions {
    pub fn new() -> Self {
        Self {
            sessions_to_users: Mutex::new(HashMap::<String, i32>::new()),
        }
    }
    fn get_random_code() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect()
    }
    pub fn add(&self, user_id: i32) -> String {
        let session_code: String = Self::get_random_code();
        self.sessions_to_users
            .lock()
            .expect("lock")
            .insert(session_code.clone(), user_id);
        session_code
    }
    pub fn remove(&self, session: &str) {
        self.sessions_to_users.lock().expect("lock").remove(session);
    }
    pub fn get_user(&self, session: &str) -> Option<i32> {
        self.sessions_to_users
            .lock()
            .expect("lock")
            .get(session)
            .copied()
    }
}
