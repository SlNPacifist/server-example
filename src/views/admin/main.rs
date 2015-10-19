use std::path::Path;
use iron::prelude::*;
use iron::status;
use iron::headers::*;
use persistent::Read;
use dtl::{Context, HashMapContext};
use models::Consumer;
use dtl_impls::{ConsumerWithPaymentInfoList};
use db::Database;
use views::TemplateCompilerKey;


pub fn entry(req: &mut Request) -> IronResult<Response> {
	let pool = req.get::<Read<Database>>().unwrap();
	let consumers = Consumer::ordered_by_last_payment(&pool.get().unwrap());
	let template_compiler = req.get::<Read<TemplateCompilerKey>>().unwrap();
    let mut ctx = HashMapContext::new();
    let tmp: ConsumerWithPaymentInfoList = consumers.into();
    ctx.set("consumers", Box::new(tmp));
    let response_text = template_compiler.render(Path::new("admin/main.htmt"), &ctx).unwrap();
    let mut res = Response::with((status::Ok, response_text));
    res.headers.set(ContentType::html());
    Ok(res)
}