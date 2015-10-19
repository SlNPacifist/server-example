use std::path::Path;
use iron::prelude::*;
use iron::status;
use iron::headers::*;
use persistent::Read;
use iron_mountrouter::{Router, MethodPicker};
use dtl::{Context, HashMapContext};
use views::TemplateCompilerKey;


fn entry(req: &mut Request) -> IronResult<Response> {
	let template_compiler = req.get::<Read<TemplateCompilerKey>>().unwrap();
    let ctx = HashMapContext::new();
    let response_text = template_compiler.render(Path::new("main.htmt"), &ctx).unwrap();
    let mut res = Response::with((status::Ok, response_text));
    res.headers.set(ContentType::html());
    Ok(res)
}

pub fn append_entry(router: &mut Router) {
	let mut picker = MethodPicker::new();
	picker.get(entry);
	router.add_route("/", picker, false); 
}