use iron::prelude::*;
use iron_mountrouter::{Router, MethodPicker};
use dtl::HashMapContext;
use views::utils::render_ok;


fn entry(req: &mut Request) -> IronResult<Response> {
    let ctx = HashMapContext::new();
    render_ok(req, &ctx, "main.htmt")
}

pub fn append_entry(router: &mut Router) {
	let mut picker = MethodPicker::new();
	picker.get(entry);
	router.add_route("/", picker, false); 
}