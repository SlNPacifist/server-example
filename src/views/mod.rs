mod main;
mod consumer;
mod static_files;

use iron::prelude::*;
use persistent::Read;
use router::Router;
use db::{DbConnectionPool, Database};


pub fn get_root(pool: DbConnectionPool) -> Chain {
	let mut router = Router::new();
	main::append_entry(&mut router);
	consumer::append_entry(&mut router);
	static_files::append_entry(&mut router);
	let mut chain = Chain::new(router);
	chain.link_before(Read::<Database>::one(pool));
	chain
}