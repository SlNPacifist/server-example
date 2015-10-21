extern crate rand;
extern crate iron;
extern crate persistent;
extern crate mount;
extern crate iron_mountrouter;
extern crate staticfile;
extern crate oatmeal_raisin;
extern crate plugin;
extern crate dtl;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate chrono;
extern crate urlencoded;
extern crate toml;
extern crate rustc_serialize;

mod models;
mod views;
mod db;
mod dtl_impls;
mod forms;
mod session;
mod config;

use std::path::Path;


fn main() {
	let config = config::get_config(Path::new("./config.toml"));
	let pool = db::get_db_connection_pool(&config.db_connection_string);
	let root = views::get_root(pool);
	println!("Server started");
    iron::Iron::new(root).http(&config.http_address as &str).expect(
    	&format!("Could not start server at {}", config.http_address));
}