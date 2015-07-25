extern crate iron;
extern crate dtl;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

mod models;
mod dtl_impls;

use std::path::Path;
use std::sync::Arc;
use dtl::{Context, Template};
use iron::prelude::*;
use iron::status;
use iron::headers::*;
use models::*;
use dtl_impls::ConsumerList;
use r2d2_postgres::PostgresConnectionManager;

fn get_db_connection_pool() -> Arc<r2d2::Pool<PostgresConnectionManager>> {
    let config = r2d2::Config::default();
    let manager = PostgresConnectionManager::new(
    	"postgresql://slnpacifist:postgres@localhost/water",
        postgres::SslMode::None)
    .unwrap();
    Arc::new(r2d2::Pool::new(config, manager).unwrap())
}

fn main() {
	println!("Project started");
	let pool = get_db_connection_pool();
	println!("Got pool");
    Iron::new(move |_: &mut Request| {
		println!("Got request");
		let consumers = Consumer::all(pool.get().unwrap());
		println!("Got {} consumers", consumers.len());
        let mut ctx = Context::new();
        let tmp: ConsumerList = consumers.into();
        ctx.set("consumers", Box::new(tmp));
        let mut tpl = Template::new(Path::new("main.htmt"), Path::new("views"));
        match tpl.compile() {
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        };
        let mut res = Response::with((status::Ok, tpl.render(&mut ctx)));
        res.headers.set(ContentType::html());
        Ok(res)
    }).http("localhost:3000").unwrap();
}