extern crate iron;
extern crate persistent;
extern crate router;
extern crate dtl;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

mod models;
mod dtl_impls;

use std::path::Path;
use dtl::{Context, HashMapContext, Template};
use iron::prelude::*;
use iron::status;
use iron::headers::*;
use iron::typemap::Key;
use persistent::Read;
use router::Router;
use models::*;
use dtl_impls::ConsumerList;
use r2d2_postgres::PostgresConnectionManager;

struct Database {
	pool: r2d2::Pool<PostgresConnectionManager>,
}

impl Key for Database { type Value = r2d2::Pool<PostgresConnectionManager>; }

fn get_db_connection_pool() -> r2d2::Pool<PostgresConnectionManager> {
    let config = r2d2::Config::default();
    let manager = PostgresConnectionManager::new(
    	"postgresql://slnpacifist:postgres@localhost/water",
        postgres::SslMode::None)
    .unwrap();
    r2d2::Pool::new(config, manager).unwrap()
}

fn main_page(req: &mut Request) -> IronResult<Response> {
	let pool = req.get::<Read<Database>>().unwrap();
	let consumers = Consumer::all(pool.get().unwrap());
    let mut ctx = HashMapContext::new();
    let tmp: ConsumerList = consumers.into();
    ctx.set("consumers", Box::new(tmp));
    let mut tpl = Template::new(Path::new("main.htmt"), Path::new("/home/slnpacifist/eclipse_workspace/shop/src/views"));
    match tpl.compile() {
        Ok(_) => {},
        Err(e) => panic!("{}", e),
    };
    let mut res = Response::with((status::Ok, tpl.render(&mut ctx)));
    res.headers.set(ContentType::html());
    Ok(res)
}

fn main() {
	let pool = get_db_connection_pool();
	let mut router = Router::new();
	router.get("/", main_page);
	let mut chain = Chain::new(router);
	chain.link_before(Read::<Database>::one(pool));
    Iron::new(chain).http("localhost:3000").unwrap();
}