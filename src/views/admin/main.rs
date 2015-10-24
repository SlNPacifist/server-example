use iron::prelude::*;
use persistent::Read;
use dtl::{Context, HashMapContext};
use models::Consumer;
use dtl_impls::{ConsumerWithPaymentInfoList};
use db::Database;
use views::utils::render_ok;


pub fn entry(req: &mut Request) -> IronResult<Response> {
	let pool = req.get::<Read<Database>>().unwrap();
    let mut ctx = HashMapContext::new();
	let consumers = Consumer::ordered_by_last_payment(&pool.get().unwrap());
    let tmp: ConsumerWithPaymentInfoList = consumers.into();
    ctx.set("consumers", Box::new(tmp));
    render_ok(req, &ctx, "admin/main.htmt")
}