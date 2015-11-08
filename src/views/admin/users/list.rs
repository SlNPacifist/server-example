use iron::prelude::*;
use persistent::Read;
use models::{User, Consumer};
use dtl_impls::{UserList, ConsumerList};
use db::Database;
use views::utils::*;


pub fn all(req: &mut Request) -> IronResult<Response> {
	let connection = req.get::<Read<Database>>()
		.expect("Could not get connection pool in views::admin::users::list::all")
		.get().expect("Could not get connection in views::admin::users::list::all");
	let users = User::all(&connection);
    update_var(req, "users", Box::new(UserList(users)));
    let consumers = Consumer::all(&connection);
    update_var(req, "consumers", Box::new(ConsumerList(consumers)));
    render_ok(req, "admin/users/all.htmt")
}