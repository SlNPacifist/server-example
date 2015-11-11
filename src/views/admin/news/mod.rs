mod main;
mod add;
mod single;
mod forms;

use std::str::FromStr;
use iron::prelude::*;
use iron::middleware::Handler;
use iron::typemap::Key;
use iron_mountrouter::Router;
use persistent::Read;
use models::News;
use db::Database;
use views::utils::*;


pub fn append_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	subrouter.add_route("/", picker!(get => self::main::all_news), false);
	subrouter.add_route("/add/",
		picker!(get => self::add::add_news,
				post => self::add::process_add_news),
		false);
	append_single_entry(&mut subrouter);
	let entry = move |req: &mut Request| {
		update_var(req, "admin_menu_news", true);
		subrouter.handle(req)
	};
	router.add_route("/news/", entry, true);
}

fn append_single_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	subrouter.add_route("/",
		picker!(get => self::single::show,
				post => self::single::save),
		true);
	router.add_route("/:news-id/", SingleNewsHandler(Box::new(subrouter)), false);
}


pub struct SingleNewsHandler(Box<Handler>);

impl SingleNewsHandler {
	fn get_news(req: &mut Request) -> Option<News> {
		req.extensions.get::<Router>()
			.and_then(|params| params.get("id"))
			.and_then(|id| i32::from_str(id).ok())
			.and_then(|id| {
				let connection = req.get::<Read<Database>>()
					.expect("Could not get connection pool in SingleNewsHandler::get_news")
					.get().expect("Could not get connection in SingleNewsHandler::get_news");
				News::by_id(&connection, id)
			})
	}
}

impl Key for SingleNewsHandler { type Value = News; }

impl Handler for SingleNewsHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		match Self::get_news(req) {
			Some(news) => {
				req.extensions.insert::<SingleNewsHandler>(news);
				self.0.handle(req)
			},
			None =>	not_found()
		}
	}
}