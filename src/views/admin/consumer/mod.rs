mod main;

use std::str::FromStr;
use iron::prelude::*;
use iron::middleware::Handler;
use iron::typemap::Key;
use iron_mountrouter::Router;
use persistent::Read;
use models::Consumer;
use db::Database;
use views::utils::*;


pub fn append_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	subrouter.add_route("/", picker!(get => self::main::entry), false);
	subrouter.add_route("/add_payment/", picker!(post => self::main::add_payment), false);
	
	let preprocessor = ConsumerHandler(Box::new(subrouter));
	router.add_route("/consumer/:id/", preprocessor, true);
	router.add_route("/consumer/add/", self::main::add_consumer, true);
}

struct ConsumerHandler(Box<Handler>);

impl ConsumerHandler {
	fn get_consumer(req: &mut Request) -> Option<Consumer> {
		let id_opt;
		{
			let ref params = req.extensions.get::<Router>()
				.expect("Could not get router params in ConsumerHandler::get_consumer");
			id_opt = match i32::from_str(params.get("id")
				.expect("Could not get id param in ConsumerHandler::get_consumer")) {
					
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
				Consumer::by_id(&connection, id)
			},
			None => None
		}
	}
}

impl Key for ConsumerHandler { type Value = Consumer; }

impl Handler for ConsumerHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		match Self::get_consumer(req) {
			Some(consumer) => {
				req.extensions.insert::<ConsumerHandler>(consumer);
				self.0.handle(req)
			},
			None =>	not_found()
		}
	}
}