use std::str::FromStr;
use iron::prelude::*;
use persistent::Read;
use iron_mountrouter::Router;
use db::Database;
use models::{VolumePayment, Consumer};
use views::utils::*;


pub fn printable(req: &mut Request) -> IronResult<Response> {
	let connection = req.get::<Read<Database>>()
		.expect("Could not get connection pool in admin::volume_payment::printable")
		.get().expect("Could not get connection in admin::volume_payment::printable");
	let volume_payment_opt = req.extensions.get::<Router>()
		.and_then(|params| params.get("vp-id"))
		.and_then(|id| i32::from_str(id).ok())
		.and_then(|id| VolumePayment::by_id(&connection, id));
	match volume_payment_opt {
		Some(vp) => {
			let mut sum = 0.0;
			for payment in VolumePayment::up_to_date(&connection, vp.consumer_id, vp.payment_date) {
				sum += payment.sum;
			}
			update_var(req, "total_volume", sum);
			update_var(req, "consumer", Consumer::by_id(&connection, vp.consumer_id).unwrap());
			update_var(req, "payment", vp);
			render_ok(req, "admin/volume_payments/printable.htmt")
		},
		None => not_found()
	}
}