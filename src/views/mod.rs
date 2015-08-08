mod main;
mod consumer;

use db::{DbConnectionPool, Database};
use iron::prelude::*;
use router::Router;
use persistent::Read;

pub fn get_root(pool: DbConnectionPool) -> Chain {
	let mut router = Router::new();
	router.get("/", main::entry);
	router.get("/consumer/:id/", consumer::entry);
	let mut chain = Chain::new(router);
	chain.link_before(Read::<Database>::one(pool));
	chain
}