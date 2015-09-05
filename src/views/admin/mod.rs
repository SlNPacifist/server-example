mod main;

use iron::prelude::*;
use iron::headers::*;
use iron::middleware::{Handler, AroundMiddleware};
use iron::status;
use router::Router;
use session::CurrentSession;


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

impl Handler for AdminHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		if let Ok(session) = req.get::<CurrentSession>() {
			if session.user.role.is_admin() {
				return self.org.handle(req);
			}
		}
		let mut res = Response::with(status::SeeOther);
		res.headers.set(Location(format!("/login/?reason=forbidden&next=/{}", req.url.path.join("/"))));
		Ok(res)
	}
}

struct AdminPreprocessor;

impl AroundMiddleware for AdminPreprocessor {
	fn around(self, handler: Box<Handler>) -> Box<Handler> {
		Box::new(AdminHandler { org: handler } )
	}
}