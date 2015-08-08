use iron::prelude::*;
use iron::status;
use iron::headers::*;
use persistent::Read;
use db::Database;
use models::Consumer;
use dtl_impls::ConsumerList;
use dtl::{Context, HashMapContext, Template};
use std::path::Path;


pub fn entry(req: &mut Request) -> IronResult<Response> {
	let pool = req.get::<Read<Database>>().unwrap();
	let consumers = Consumer::all(&pool.get().unwrap());
    let mut ctx = HashMapContext::new();
    let tmp: ConsumerList = consumers.into();
    ctx.set("consumers", Box::new(tmp));
    let mut tpl = Template::new(Path::new("main.htmt"), Path::new("/home/slnpacifist/eclipse_workspace/shop/src/templates"));
    tpl.compile().unwrap();
    let mut res = Response::with((status::Ok, tpl.render(&mut ctx)));
    res.headers.set(ContentType::html());
    Ok(res)
}
