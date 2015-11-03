mod main;
mod add;

use std::str::FromStr;
use iron::prelude::*;
use iron::middleware::{Handler, AroundMiddleware};
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
	add_picker.post(self::add::procerss_add_news);
	subrouter.add_route("/add/", add_picker, false);
//	
//	let mut add_payment_picker = MethodPicker::new();
//	add_payment_picker.post(self::main::add_payment);
//	subrouter.add_route("/add_payment/", add_payment_picker, false);
//	
//	let mut preprocessor = Chain::new(subrouter);
//	preprocessor.around(NewsPreprocessor);
//	router.add_route("/consumer/:id/", preprocessor, true);
//	router.add_route("/news/add/", self::main::add_consumer, true);
	router.add_route("/news/", subrouter, true);
}

struct NewsHandler(Box<Handler>);

impl NewsHandler {
	fn get_news(req: &mut Request) -> Option<News> {
		let id_opt;
		{
			let ref params = req.extensions.get::<Router>()
				.expect("Could not get router params in NewsHandler::get_news");
			id_opt = match i32::from_str(params.get("id")
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

struct NewsPreprocessor;

impl AroundMiddleware for NewsPreprocessor {
	fn around(self, handler: Box<Handler>) -> Box<Handler> {
		Box::new(NewsHandler(handler) )
	}
}