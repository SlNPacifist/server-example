mod add;
mod list;

use iron::prelude::*;
use iron::middleware::Handler;
use iron_mountrouter::Router;
use views::utils::*;


pub fn append_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	subrouter.add_route("/add/", picker!(post => self::add::add_user), false);
	subrouter.add_route("/", picker!(get => self::list::all), false);
	let entry = move |req: &mut Request| {
		update_var(req, "admin_menu_users", Box::new(true));
		subrouter.handle(req)
	};
	router.add_route("/users/", entry, true);
}