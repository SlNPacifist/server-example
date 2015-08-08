extern crate iron;
extern crate persistent;
extern crate router;
extern crate dtl;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

mod models;
mod views;
mod db;
mod dtl_impls;

fn main() {
	let pool = db::get_db_connection_pool();
	let root = views::get_root(pool);
    iron::Iron::new(root).http("localhost:3000").unwrap();
}