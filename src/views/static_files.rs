use std::path::Path;
use staticfile::Static;
use iron_mountrouter::Router;
use mount::Mount;


pub fn append_entry(router: &mut Router) {
	let s = Static::new(Path::new("/home/slnpacifist/eclipse_workspace/shop/src/static"));
	let mut mounter = Mount::new();
	mounter.mount("/s/", s);
	router.add_route("/s/", mounter, true);
}