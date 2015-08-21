mod main;

use std::str::FromStr;
use iron::prelude::*;
use iron::middleware::{Handler, AroundMiddleware};
use iron::status;
use iron::typemap::Key;
use router::Router;
use persistent::Read;
use models::Consumer;
use db::Database;


pub fn append_entry(router: &mut Router) {
	let mut get_entry = Chain::new(self::main::entry);
	get_entry.around(ConsumerPreprocessor);
	router.get("/consumer/:id/", get_entry);
	let mut post_entry = Chain::new(self::main::add_payment);
	post_entry.around(ConsumerPreprocessor);
	router.post("/consumer/:id/add_payment", post_entry);
}

struct ConsumerHandler {
	org: Box<Handler>
}

impl ConsumerHandler {
	fn get_consumer(req: &mut Request) -> Option<Consumer> {
		let id_opt;
		{
			let ref params = req.extensions.get::<Router>().unwrap();
			id_opt = match i32::from_str(params.find("id").unwrap()) {
				Ok(consumer_id) => Some(consumer_id),
				Err(_) => None
			};
		}
		match id_opt {
			Some(id) => { 
				let pool = req.get::<Read<Database>>().unwrap();
				let connection = pool.get().unwrap();
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
				self.org.handle(req)
			},
			None =>	Ok(Response::with(status::NotFound))
		}
	}
}

struct ConsumerPreprocessor;

impl AroundMiddleware for ConsumerPreprocessor {
	fn around(self, handler: Box<Handler>) -> Box<Handler> {
		Box::new(ConsumerHandler { org: handler } )
	}
}