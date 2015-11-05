extern crate rand;
extern crate iron;
extern crate persistent;
extern crate mount;
#[macro_use]
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
extern crate clap;

mod models;
mod views;
mod db;
mod dtl_impls;
mod forms;
mod session;
mod config;

use std::path::Path;
use clap::{App, SubCommand};
use db::DbConnectionPool;
use models::{User, UserRole};
use config::Config;


fn main() {
	let config = config::get_config(Path::new("./config.toml"));
	let pool = db::get_db_connection_pool(&config.db_connection_string);

    let matches = App::new("Zima water supplies web server")
        .subcommand(SubCommand::with_name("start")
            .about("starts web server"))
        .subcommand(SubCommand::with_name("add-admin")
        	.about("adds new admin")
        	.args_from_usage("<name> 'Username for created admin'
        					  <password> 'Password for created admin'"))
        .subcommand_required(true)
        .get_matches();
    if matches.subcommand_matches("start").is_some() {
    	start_server(config, pool);
	} else if let Some(ref sub_matches) = matches.subcommand_matches("add-admin") {
    	let name = sub_matches.value_of("name")
    		.expect("Could not get _name_ param from command line");
    	let password = sub_matches.value_of("password")
    		.expect("Could not get _password_ param from command line");
    	add_admin(pool, name, password);
    }
}

fn start_server(config: Config, pool: DbConnectionPool) {
	let root = views::get_root(pool);
	println!("Server started");
    iron::Iron::new(root).http(&config.http_address as &str).expect(
    	&format!("Could not start server at {}", config.http_address));
}

fn add_admin(pool: DbConnectionPool, name: &str, password: &str) {
	let connection = pool.get().expect("Could not get connection in main::add_admin");
	User::create(&connection, name.to_string(), password.to_string(), UserRole::Admin, None)
		.expect("Could not add admin");
	println!("Admin {} added", name);
}