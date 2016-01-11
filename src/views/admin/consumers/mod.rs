mod single;
mod add;
mod list;

use std::str::FromStr;
use iron::prelude::*;
use iron::middleware::Handler;
use iron::typemap::Key;
use iron_mountrouter::Router;
use persistent::Read;
use models::Consumer;
use db::Database;
use views::utils::*;
use views::admin::volume_payments::single::add as add_volume_payment;


pub fn append_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	subrouter.add_route("/add/", self::add::add_consumer, false);
	subrouter.add_route("/", self::list::all, false);
	append_single_entry(&mut subrouter);
	let entry = move |req: &mut Request| {
		update_var(req, "admin_menu_consumers", true);
		subrouter.handle(req)
	};
	router.add_route("/consumers/", entry, true);
}

fn append_single_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	subrouter.add_route("/", picker!(get => self::single::entry), false);
	subrouter.add_route("/add_payment/", picker!(post => add_volume_payment), false);

	let preprocessor = ConsumerHandler(Box::new(subrouter));
	router.add_route("/:id/", preprocessor, true);
}

pub struct ConsumerHandler(Box<Handler>);

impl ConsumerHandler {
	fn get_consumer(req: &mut Request) -> Option<Consumer> {
		req.extensions.get::<Router>()
			.and_then(|params| params.get("id"))
			.and_then(|id| i32::from_str(id).ok())
			.and_then(|id| {
				let connection = req.get::<Read<Database>>()
					.expect("Could not get connection pool in ConsumerHandler::get_consumer")
					.get().expect("Could not get connection in ConsumerHandler::get_consumer");
				Consumer::by_id(&connection, id)
			})
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