use iron::prelude::*;
use iron::status;
use iron::headers::*;
use persistent::Read;
use db::Database;
use models::VolumePayment;
use dtl_impls::VolumePaymentList;
use dtl::{Context, HashMapContext, TemplateCompiler};
use std::path::{Path, PathBuf};
use super::ConsumerHandler;


pub fn entry(req: &mut Request) -> IronResult<Response> {
	let connection = req.get::<Read<Database>>().unwrap().get().unwrap();
    let mut ctx = HashMapContext::new();
    let consumer = req.extensions.get::<ConsumerHandler>().unwrap();
	let payments = VolumePayment::for_consumer(&connection, consumer.id);
	let mut volume_sum = 0.0;
	for p in payments.iter() {
		volume_sum += p.volume;
	}
	ctx.set("consumer", Box::new(consumer.clone()));
	ctx.set("payments", Box::new(VolumePaymentList::new(payments)));
	ctx.set("total_volume_sum", Box::new(volume_sum));
    let mut root = PathBuf::new();
    root.push("/home/slnpacifist/eclipse_workspace/shop/src/templates");
    let response_text = TemplateCompiler::render_file(root,	Path::new("consumer.htmt"),	&ctx).unwrap();
    let mut res = Response::with((status::Ok, response_text));
    res.headers.set(ContentType::html());
    Ok(res)
}
