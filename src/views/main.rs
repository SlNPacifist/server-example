use iron::prelude::*;
use iron_mountrouter::{Router, MethodPicker};
use views::utils::render_ok;


fn entry(req: &mut Request) -> IronResult<Response> {
    render_ok(req, "main.htmt")
}

pub fn append_entry(router: &mut Router) {
	let mut picker = MethodPicker::new();
	picker.get(entry);
	router.add_route("/", picker, false); 
}