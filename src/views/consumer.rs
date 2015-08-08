use iron::prelude::*;
use iron::status;
use iron::headers::*;
use persistent::Read;
use router::Router;
use db::Database;
use models::{Consumer, VolumePayment};
use dtl_impls::VolumePaymentList;
use dtl::{Context, HashMapContext, Template};
use std::path::Path;
use std::str::FromStr;


pub fn entry(req: &mut Request) -> IronResult<Response> {
	let pool = req.get::<Read<Database>>().unwrap();
	let ref params = req.extensions.get::<Router>().unwrap();
    let mut ctx = HashMapContext::new();
	match i32::from_str(params.find("id").unwrap()) {
		Ok(consumer_id) => {
			let connection = pool.get().unwrap();
			match Consumer::by_id(&connection, consumer_id) {
				Some(consumer) => {
					ctx.set("consumer", Box::new(consumer));
					let payments: VolumePaymentList = VolumePayment::for_consumer(&connection, consumer_id).into();
					ctx.set("payments", Box::new(payments));
				},
				None => ctx.set("consumer", Box::new(false))
			}
		},
		Err(_) => ctx.set("consumer", Box::new(false)),
	}
    let mut tpl = Template::new(Path::new("consumer.htmt"), Path::new("/home/slnpacifist/eclipse_workspace/shop/src/templates"));
    tpl.compile().unwrap();
    let mut res = Response::with((status::Ok, tpl.render(&mut ctx)));
    res.headers.set(ContentType::html());
    Ok(res)
}
