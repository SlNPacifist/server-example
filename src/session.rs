use std::collections::HashMap;
use models::User;

#[derive(Debug, Clone)]
pub struct Session {
	pub id: String,
	pub user: User,
}

impl Session {
	pub fn new(user: User) -> Session {
		Session {
			id: "abc".to_string(),
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