use iron::prelude::*;
use persistent::Read;
use models::Consumer;
use dtl_impls::ConsumerWithPaymentInfoList;
use db::Database;
use views::utils::*;


pub fn all(req: &mut Request) -> IronResult<Response> {
	let connection = req.get::<Read<Database>>()
		.expect("Could not get connection pool in views::admin::consumer::list::all")
		.get().expect("Could not get connection in views::admin::consumer::list::all");
	let consumers = Consumer::with_last_payment(&connection);
    update_var(req, "consumers", ConsumerWithPaymentInfoList::new(consumers));
    render_ok(req, "admin/consumers/all.htmt")
}