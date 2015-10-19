mod main;
mod consumer;
mod user;

use iron::prelude::*;
use iron::headers::*;
use iron::middleware::{Handler, AroundMiddleware};
use iron::status;
use iron_mountrouter::{Router, MethodPicker};
use session::CurrentSession;


pub fn append_entry(router: &mut Router) {
	let mut subrouter = Router::new();

	let mut main_picker = MethodPicker::new();
	main_picker.get(self::main::entry);
	subrouter.add_route("/", main_picker, false);

	consumer::append_entry(&mut subrouter);
	user::append_entry(&mut subrouter);
	
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