use iron::prelude::*;
use persistent::Read;
use models::Consumer;
use dtl_impls::{ConsumerWithPaymentInfoList};
use db::Database;
use views::utils::*;


pub fn entry(req: &mut Request) -> IronResult<Response> {
	let pool = req.get::<Read<Database>>().unwrap();
	let consumers = Consumer::ordered_by_last_payment(&pool.get().unwrap());
    let tmp: ConsumerWithPaymentInfoList = consumers.into();
    update_var(req, "consumers", Box::new(tmp));
    render_ok(req, "admin/main.htmt")
}