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
	router.add_route("/news/", NewsHandler(Box::new(subrouter)), true);
}

fn append_single_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	subrouter.add_route("/",
		picker!(get => self::single::show,
				post => self::single::save),
		true);
	router.add_route("/:news-id/", SingleNewsHandler(Box::new(subrouter)), false);
}


struct NewsHandler(Box<Handler>);

impl Handler for NewsHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		update_var(req, "admin_menu_news", Box::new(true));
		self.0.handle(req)
	}
}


pub struct SingleNewsHandler(Box<Handler>);

impl SingleNewsHandler {
	fn get_news(req: &mut Request) -> Option<News> {
		let id_opt;
		{
			let ref params = req.extensions.get::<Router>()
				.expect("Could not get router params in NewsHandler::get_news");
			id_opt = match i32::from_str(params.get("news-id")
				.expect("Could not get id param in NewsHandler::get_news")) {
					
				Ok(consumer_id) => Some(consumer_id),
				Err(_) => None
			};
		}
		match id_opt {
			Some(id) => { 
				let pool = req.get::<Read<Database>>()
					.expect("Could not get connection pool in ConsumerHandler::get_consumer");
				let connection = pool.get()
					.expect("Could not get connection in ConsumerHandler::get_consumer");
				News::by_id(&connection, id)
			},
			None => None
		}
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