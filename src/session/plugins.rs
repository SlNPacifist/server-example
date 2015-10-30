use iron::prelude::Request;
use iron::typemap::Key;
use plugin::{Plugin, Pluggable};
use oatmeal_raisin as or;
use persistent::State;
use views::SessionStorageKey;
use super::{Session, SessionStorage};


pub enum SessionError {
	NoSessionId,
	NoSession,
}

pub struct CurrentSession;

impl Key for CurrentSession { type Value = Session; }
impl<'a, 'b> Plugin<Request<'a, 'b>> for CurrentSession {
	type Error = SessionError;
	
	fn eval(req: &mut Request) -> Result<Session, SessionError> {
		let session_id = {
			let jar = req.get_mut::<or::CookieJar>().expect("Could not get cookies in request");
			match jar.find("session-id") {
				Some(cookie) => cookie.value.clone(),
				None => return Err(SessionError::NoSessionId),
			}
		};
		let arc_session_storage = req.get::<State<SessionStorageKey>>()
			.expect("Could not get session storage in request");
		let session_storage = arc_session_storage.read()
			.expect("Could not read session storage");
		match session_storage.by_id(&session_id) {
			Some(session) => Ok(session.clone()),
			None => Err(SessionError::NoSession),
		}
	}
}