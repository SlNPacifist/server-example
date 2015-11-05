mod main;
mod consumer;
mod user;
mod news;

use iron::prelude::*;
use iron::middleware::Handler;
use iron_mountrouter::Router;
use session::CurrentSession;
use views::utils::*;


pub fn append_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	subrouter.add_route("/", picker!(get => self::main::entry), false);

	consumer::append_entry(&mut subrouter);
	user::append_entry(&mut subrouter);
	news::append_entry(&mut subrouter);
	
	router.add_route("/admin/", AdminHandler(Box::new(subrouter)), true);
}

struct AdminHandler(Box<Handler>);

impl Handler for AdminHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		if let Ok(session) = req.get::<CurrentSession>() {
			if session.user.role.is_admin() {
				update_var(req, "in_admin", Box::new(true));
				return self.0.handle(req);
			}
		}
		redirect(format!("/login/?reason=forbidden&next=/{}", req.url.path.join("/")))
	}
}