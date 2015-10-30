mod plugins;

pub use self::plugins::CurrentSession;

use std::collections::HashMap;
use models::User;
use rand::{self, Rng};

const SESSION_ID_LENGTH: usize = 32;

#[derive(Debug, Clone)]
pub struct Session {
	pub id: String,
	pub user: User,
}

impl Session {
	fn generate_id() -> String {
		rand::OsRng::new().expect("Could not create OsRng in Session::generate_id")
			.gen_ascii_chars().take(SESSION_ID_LENGTH).collect()
	}
	
	pub fn new(user: User) -> Session {
		Session {
			id: Self::generate_id(),
			user: user
		}
	}
}

pub trait SessionStorage {
	fn by_id(&self, id: &str) -> Option<Session>;
	fn insert(&mut self, session: Session);
	fn remove(&mut self, session_id: &str);
}

#[derive(Debug)]
pub struct MemorySessionStorage {
	sessions: HashMap<String, Session>,
}

impl SessionStorage for MemorySessionStorage {
	fn by_id(&self, id: &str) -> Option<Session> {
		match self.sessions.get(id) {
			Some(sess) => Some(sess.clone()),
			None => None
		}
	}
	
	fn insert(&mut self, session: Session) {
		self.sessions.insert(session.id.clone(), session);
	}
	
	fn remove(&mut self, session_id: &str) {
		self.sessions.remove(session_id);
	}
}

impl MemorySessionStorage {
	pub fn new() -> Self {
		MemorySessionStorage {
			sessions: HashMap::new(),
		}
	}
}