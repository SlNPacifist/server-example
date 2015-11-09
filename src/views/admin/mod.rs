mod consumers;
mod users;
mod news;

use iron::prelude::*;
use iron::middleware::Handler;
use iron_mountrouter::Router;
use session::CurrentSession;
use views::utils::*;


pub fn append_entry(router: &mut Router) {
	let mut subrouter = Router::new();

	consumers::append_entry(&mut subrouter);
	users::append_entry(&mut subrouter);
	news::append_entry(&mut subrouter);
	
	router.add_route("/admin/", move |req: &mut Request| admin_handle(req, &subrouter), true);
}

fn admin_handle<T: Handler>(req: &mut Request, next: &T) -> IronResult<Response> {
	if let Ok(session) = req.get::<CurrentSession>() {
		if session.user.role.is_admin() {
			update_var(req, "in_admin", true);
			return next.handle(req);
		}
	}
	redirect(format!("/login/?reason=forbidden&next=/{}", req.url.path.join("/")))
}