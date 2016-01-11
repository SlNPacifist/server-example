use iron::prelude::*;
use persistent::Read;
use chrono;
use db::Database;
use models::VolumePayment;
use dtl_impls::VolumePaymentList;
use super::ConsumerHandler;
use views::utils::*;


pub fn entry(req: &mut Request) -> IronResult<Response> {
	let connection = req.get::<Read<Database>>()
		.expect("Could not get connection pool in admin::consumers::single::entry")
		.get().expect("Could not get connection in admin::consumers::single::entry");
	let consumer = req.extensions.remove::<ConsumerHandler>()
		.expect("Could not get consumer in admin::consumers::single::entry");
	let payments = VolumePayment::for_consumer(&connection, consumer.id);
	let mut volume_sum = 0.0;
	let mut money_sum = 0.0;
	for p in payments.iter() {
		volume_sum += p.volume;
		money_sum += p.sum;
	}
	update_var(req, "consumer", consumer); 
	update_var(req, "payments", VolumePaymentList::new(payments)); 
	update_var(req, "total_volume_sum", volume_sum); 
	update_var(req, "total_money_sum", money_sum); 
	update_var(req, "today", chrono::Local::today()); 
	render_ok(req, "admin/consumers/single.htmt")
}