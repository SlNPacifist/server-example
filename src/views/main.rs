use iron::prelude::*;
use iron_mountrouter::Router;
use persistent::Read;
use views::utils::*;
use db::Database;
use models::News;
use dtl_impls::NewsList;


fn entry(req: &mut Request) -> IronResult<Response> {
	let pool = req.get::<Read<Database>>()
		.expect("Could not get connection pool in views::main::entry");
	let connection = pool.get().expect("Could not get connection in views::main::entry");
	let news = NewsList::new(News::ordered_by_date(&connection, 5));
	update_var(req, "news", news);
    render_ok(req, "main.htmt")
}

pub fn append_entry(router: &mut Router) {
	router.add_route("/", picker!(get => entry), false); 
}