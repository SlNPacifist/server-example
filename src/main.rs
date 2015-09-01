extern crate iron;
extern crate persistent;
extern crate mount;
extern crate router;
extern crate staticfile;
extern crate oatmeal_raisin;
extern crate dtl;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate chrono;
extern crate urlencoded;

mod models;
mod views;
mod db;
mod dtl_impls;
mod forms;
mod session;

fn main() {
	let pool = db::get_db_connection_pool();
	let root = views::get_root(pool);
	println!("Server started");
    iron::Iron::new(root).http("localhost:3000").unwrap();
}