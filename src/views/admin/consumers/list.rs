use iron::prelude::*;
use persistent::Read;
use models::Consumer;
use dtl_impls::ConsumerList;
use db::Database;
use views::utils::*;


pub fn all(req: &mut Request) -> IronResult<Response> {
	let connection = req.get::<Read<Database>>()
		.expect("Could not get connection pool in views::admin::consumer::list::all")
		.get().expect("Could not get connection in views::admin::consumer::list::all");
	let consumers = Consumer::all(&connection);
    update_var(req, "consumers", ConsumerList(consumers));
    render_ok(req, "admin/consumers/all.htmt")
}