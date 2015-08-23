mod main;

use router::Router;


pub fn append_entry(router: &mut Router) {
	router.get("/admin/", self::main::entry);
	router.post("/admin/add_user/", self::main::add_user);
}
