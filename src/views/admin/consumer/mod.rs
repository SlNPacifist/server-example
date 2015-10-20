mod main;

use std::str::FromStr;
use iron::prelude::*;
use iron::middleware::{Handler, AroundMiddleware};
use iron::status;
use iron::typemap::Key;
use iron_mountrouter::{Router, MethodPicker};
use persistent::Read;
use models::Consumer;
use db::Database;


pub fn append_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	
	let mut consumer_picker = MethodPicker::new();
	consumer_picker.get(self::main::entry);
	subrouter.add_route("/", consumer_picker, false);
	
	let mut add_payment_picker = MethodPicker::new();
	add_payment_picker.post(self::main::add_payment);
	subrouter.add_route("/add_payment/", add_payment_picker, false);
	
	let mut preprocessor = Chain::new(subrouter);
	preprocessor.around(ConsumerPreprocessor);
	router.add_route("/consumer/:id/", preprocessor, true);
	router.add_route("/consumer/add/", self::main::add_consumer, true);
}

struct ConsumerHandler {
	org: Box<Handler>
}

impl ConsumerHandler {
	fn get_consumer(req: &mut Request) -> Option<Consumer> {
		let id_opt;
		{
			let ref params = req.extensions.get::<Router>().unwrap();
			id_opt = match i32::from_str(params.get("id").unwrap()) {
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