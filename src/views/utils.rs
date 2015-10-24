use std::path::Path;
use iron::prelude::*;
use iron::status::Status;
use iron::headers::*;
use persistent::Read;
use views::TemplateCompilerKey;
use dtl::Context;

pub fn render_status(req: &mut Request, ctx: &Context, template: &str, st: Status) -> IronResult<Response> {
	let template_compiler = req.get::<Read<TemplateCompilerKey>>().unwrap();
    let response_text = match template_compiler.render(Path::new(template), ctx) {
    	Ok(text) => text,
    	Err(error) => panic!("Could not render template {}: {}", template, error)
    };
    let mut res = Response::with((st, response_text));
    res.headers.set(ContentType::html());
    Ok(res)
}

pub fn render_ok(req: &mut Request, ctx: &Context, template: &str) -> IronResult<Response> {
	render_status(req, ctx, template, Status::Ok)
}

pub fn redirect(loc: String) -> IronResult<Response> {
	let mut res = Response::with(Status::SeeOther);
	res.headers.set(Location(loc));
	Ok(res)
}

pub fn not_found() -> IronResult<Response> {
	Ok(Response::with(Status::NotFound))
}