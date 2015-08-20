mod main;
mod consumer;

use db::{DbConnectionPool, Database};
use iron::prelude::*;
use router::Router;
use persistent::Read;
use mount::Mount;
use staticfile::Static;
use std::path::Path;

pub fn get_root(pool: DbConnectionPool) -> Chain {
	let mut router = Router::new();
	router.get("/", main::entry);
	consumer::append_entry(&mut router);
	let mut mounter = Mount::new();
	mounter.mount("/s/", Static::new(Path::new("/home/slnpacifist/eclipse_workspace/shop/src/static")));
	router.get("/s/*", mounter);
	let mut chain = Chain::new(router);
	chain.link_before(Read::<Database>::one(pool));
	chain
}