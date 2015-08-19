use iron::prelude::*;
use iron::status;
use iron::headers::*;
use persistent::Read;
use router::Router;
use db::Database;
use models::{Consumer, VolumePayment};
use dtl_impls::VolumePaymentList;
use dtl::{Context, HashMapContext, TemplateCompiler};
use std::path::{Path, PathBuf};
use std::str::FromStr;


pub fn entry(req: &mut Request) -> IronResult<Response> {
	let pool = req.get::<Read<Database>>().unwrap();
	let ref params = req.extensions.get::<Router>().unwrap();
    let mut ctx = HashMapContext::new();
    let payments;
	match i32::from_str(params.find("id").unwrap()) {
		Ok(consumer_id) => {
			let connection = pool.get().unwrap();
			match Consumer::by_id(&connection, consumer_id) {
				Some(consumer) => {
					ctx.set("consumer", Box::new(consumer));
					payments = VolumePayment::for_consumer(&connection, consumer_id);
					ctx.set("payments", Box::new(VolumePaymentList::new(payments)));
				},
				None => ctx.set("consumer", Box::new(false))
			}
		},
		Err(_) => ctx.set("consumer", Box::new(false)),
	}
    let mut root = PathBuf::new();
    root.push("/home/slnpacifist/eclipse_workspace/shop/src/templates");
    let response_text = TemplateCompiler::render_file(root,	Path::new("consumer.htmt"),	&ctx).unwrap();
    let mut res = Response::with((status::Ok, response_text));
    res.headers.set(ContentType::html());
    Ok(res)
}
