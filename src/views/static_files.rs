use std::path::Path;
use staticfile::Static;
use mount::Mount;
use router::Router;


pub fn append_entry(router: &mut Router) {
	let mut mounter = Mount::new();
	mounter.mount("/s/", Static::new(Path::new("/home/slnpacifist/eclipse_workspace/shop/src/static")));
	router.get("/s/*", mounter);
}