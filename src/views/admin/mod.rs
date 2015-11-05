mod main;
mod consumer;
mod user;
mod news;

use iron::prelude::*;
use iron::middleware::{Handler, AroundMiddleware};
use iron_mountrouter::Router;
use session::CurrentSession;
use views::utils::*;


pub fn append_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	subrouter.add_route("/", picker!(get => self::main::entry), false);

	consumer::append_entry(&mut subrouter);
	user::append_entry(&mut subrouter);
	news::append_entry(&mut subrouter);
	
	let mut preprocessor = Chain::new(subrouter);
	preprocessor.around(AdminPreprocessor);
	router.add_route("/admin/", preprocessor, true);
}

struct AdminHandler {
	org: Box<Handler>
}

impl Handler for AdminHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		if let Ok(session) = req.get::<CurrentSession>() {
			if session.user.role.is_admin() {
				update_var(req, "in_admin", Box::new(true));
				return self.org.handle(req);
			}
		}
		redirect(format!("/login/?reason=forbidden&next=/{}", req.url.path.join("/")))
	}
}

struct AdminPreprocessor;

impl AroundMiddleware for AdminPreprocessor {
	fn around(self, handler: Box<Handler>) -> Box<Handler> {
		Box::new(AdminHandler { org: handler } )
	}
}