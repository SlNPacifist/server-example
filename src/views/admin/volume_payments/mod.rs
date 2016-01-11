pub mod single;

use std::str::FromStr;
use iron::prelude::*;
use iron::middleware::Handler;
use iron::typemap::Key;
use iron_mountrouter::Router;
use persistent::Read;
use models::VolumePayment;
use db::Database;
use views::utils::*;

pub fn append_entry(router: &mut Router) {
	let mut subrouter = Router::new();
	subrouter.add_route("/printable/", picker!(get => single::printable), false);
	subrouter.add_route("/", picker!(
			get => single::editable,
			post => single::save
		), false);

	let preprocessor = VolumePaymentHandler(Box::new(subrouter));
	router.add_route("/volume_payments/:vp-id/", preprocessor, true);
}

pub struct VolumePaymentHandler(Box<Handler>);

impl VolumePaymentHandler {
	fn get_volume_payment(req: &mut Request) -> Option<VolumePayment> {
		req.extensions.get::<Router>()
			.and_then(|params| params.get("vp-id"))
			.and_then(|id| i32::from_str(id).ok())
			.and_then(|id| {
				let connection = req.get::<Read<Database>>()
					.expect("Could not get connection pool in VolumePaymentHandler::get_volume_payment")
					.get().expect("Could not get connection in VolumePaymentHandler::get_volume_payment");
				VolumePayment::by_id(&connection, id)
			})
	}
}

impl Key for VolumePaymentHandler { type Value = VolumePayment; }

impl Handler for VolumePaymentHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		match Self::get_volume_payment(req) {
			Some(payment) => {
				req.extensions.insert::<VolumePaymentHandler>(payment);
				self.0.handle(req)
			},
			None =>	not_found()
		}
	}
}