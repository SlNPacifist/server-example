use std::path::Path;
use iron::prelude::*;
use iron::status::Status;
use iron::headers::*;
use views::{TemplateCompilerKey, ContextKey};
use dtl::{Context, Value};

pub fn render_status(req: &mut Request, template: &str, st: Status) -> IronResult<Response> {
	let template_compiler = req.extensions.get::<TemplateCompilerKey>().unwrap();
	let ctx = req.extensions.get::<ContextKey>().unwrap();
    let response_text = match template_compiler.render(Path::new(template), ctx) {
    	Ok(text) => text,
    	Err(error) => panic!("Could not render template {}: {}", template, error)
    };
    let mut res = Response::with((st, response_text));
    res.headers.set(ContentType::html());
    Ok(res)
}

pub fn render_ok(req: &mut Request, template: &str) -> IronResult<Response> {
	render_status(req, template, Status::Ok)
}

pub fn redirect(loc: String) -> IronResult<Response> {
	let mut res = Response::with(Status::SeeOther);
	res.headers.set(Location(loc));
	Ok(res)
}

pub fn not_found() -> IronResult<Response> {
	Ok(Response::with(Status::NotFound))
}

pub fn update_var(req: &mut Request, name: &str, val: Box<Value>) {
	let ctx = req.extensions.get_mut::<ContextKey>().unwrap();
	ctx.set(name, val);
}