use iron::prelude::*;
use persistent::Read;
use models::Consumer;
use dtl_impls::{ConsumerWithPaymentInfoList};
use db::Database;
use views::utils::*;


pub fn entry(req: &mut Request) -> IronResult<Response> {
	let pool = req.get::<Read<Database>>()
		.expect("Could not get connection pool in views::admin::main::entry");
	let consumers = Consumer::ordered_by_last_payment(
		&pool.get().expect("Could not get connection in views::admin::main::entry"));
    let tmp: ConsumerWithPaymentInfoList = consumers.into();
    update_var(req, "consumers", Box::new(tmp));
    update_var(req, "admin_menu_main", Box::new(true));
    render_ok(req, "admin/main.htmt")
}