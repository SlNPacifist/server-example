use iron::prelude::*;
use iron::status;
use iron::headers::*;
use persistent::Read;
use db::Database;
use models::Consumer;
use dtl_impls::ConsumerList;
use dtl::{Context, HashMapContext, TemplateCompiler};
use std::path::{Path, PathBuf};

pub fn entry(req: &mut Request) -> IronResult<Response> {
	let pool = req.get::<Read<Database>>().unwrap();
	let consumers = Consumer::all(&pool.get().unwrap());
    let mut ctx = HashMapContext::new();
    let tmp: ConsumerList = consumers.into();
    ctx.set("consumers", Box::new(tmp));
    let mut root = PathBuf::new();
    root.push("/home/slnpacifist/eclipse_workspace/shop/src/templates");
    let response_text = TemplateCompiler::render_file(root,	Path::new("main.htmt"),	&ctx).unwrap();
    let mut res = Response::with((status::Ok, response_text));
    res.headers.set(ContentType::html());
    Ok(res)
}
