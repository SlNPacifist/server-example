mod main;

use iron::prelude::*;
use iron::middleware::{Handler, AroundMiddleware};
use iron::status;
use iron::typemap::Key;
use router::Router;
use oatmeal_raisin as or;
use persistent::State;
use models::{User, UserRole};
use views::SessionStorageKey;
use session::SessionStorage;


pub fn append_entry(router: &mut Router) {
	let mut get_entry = Chain::new(self::main::entry);
	get_entry.around(AdminPreprocessor);
	router.get("/admin/", get_entry);
	let mut post_entry = Chain::new(self::main::add_user);
	post_entry.around(AdminPreprocessor);
	router.post("/admin/add_user/", post_entry);
}

struct AdminHandler {
	org: Box<Handler>
}

impl AdminHandler {
	fn get_user(req: &mut Request) -> Option<User> {
		let opt_session_id = {
			let jar = req.get_mut::<or::CookieJar>().unwrap();
			match jar.find("session-id") {
				Some(cookie) => Some(cookie.value.clone()),
				None => None
			}
		};
		match opt_session_id {
			Some(session_id) => {
				let arc_session_storage = req.get::<State<SessionStorageKey>>().unwrap();
				let session_storage = arc_session_storage.read().unwrap();
				match session_storage.by_id(&session_id) {
					Some(session) => Some(session.user.clone()),
					None => None
				}
			}
			None => None
		}
	}
}

impl Key for AdminHandler { type Value = User; }

impl Handler for AdminHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		if let Some(user) = Self::get_user(req) {
			match user.role {
				UserRole::Admin => {
					req.extensions.insert::<AdminHandler>(user);
					return self.org.handle(req);
				},
				_ => {}
			}
		}
		Ok(Response::with(status::NotFound))
	}
}

struct AdminPreprocessor;

impl AroundMiddleware for AdminPreprocessor {
	fn around(self, handler: Box<Handler>) -> Box<Handler> {
		Box::new(AdminHandler { org: handler } )
	}
}