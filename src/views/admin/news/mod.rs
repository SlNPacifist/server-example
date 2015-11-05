mod main;
mod add;
mod single;
mod forms;

use std::str::FromStr;
use iron::prelude::*;
use iron::middleware::Handler;
use iron::typemap::Key;
use iron_mountrouter::{Router, MethodPicker};
use persistent::Read;
use models::News;
use db::Database;
use views::utils::*;


pub fn append_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	
	let mut news_picker = MethodPicker::new();
	news_picker.get(self::main::all_news);
	subrouter.add_route("/", news_picker, false);
	
	let mut add_picker = MethodPicker::new();
	add_picker.get(self::add::add_news);
	add_picker.post(self::add::process_add_news);
	subrouter.add_route("/add/", add_picker, false);
	
	append_single_entry(&mut subrouter);
	router.add_route("/news/", subrouter, true);
}

fn append_single_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	
	let mut picker = MethodPicker::new();
	picker.get(self::single::show);
	picker.post(self::single::save);
	subrouter.add_route("/", picker, true);
	
	let preprocessor = NewsHandler(Box::new(subrouter));
	router.add_route("/:news-id/", preprocessor, false);
}

pub struct NewsHandler(Box<Handler>);

impl NewsHandler {
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

impl Key for NewsHandler { type Value = News; }

impl Handler for NewsHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		match Self::get_news(req) {
			Some(news) => {
				req.extensions.insert::<NewsHandler>(news);
				self.0.handle(req)
			},
			None =>	not_found()
		}
	}
}