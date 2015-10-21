mod main;
mod admin;
mod login;
mod static_files;

use std::path::PathBuf;
use dtl::TemplateCompiler;
use iron::prelude::*;
use iron::typemap::Key;
use persistent::{Read, State};
use iron_mountrouter::Router;
use db::{DbConnectionPool, Database};
use session::MemorySessionStorage;

pub struct TemplateCompilerKey;
impl Key for TemplateCompilerKey { type Value = TemplateCompiler; }

pub struct SessionStorageKey;
impl Key for SessionStorageKey { type Value = MemorySessionStorage; }


pub fn get_root(pool: DbConnectionPool) -> Chain {
	let mut router = Router::new();
	main::append_entry(&mut router);
	admin::append_entry(&mut router);
	login::append_entry(&mut router);
	static_files::append_entry(&mut router);
    let mut root = PathBuf::new();
    root.push("./src/templates");
    let template_compiler = TemplateCompiler::new(root).unwrap();
    let session_storage = MemorySessionStorage::new();
	let mut chain = Chain::new(router);
	chain.link_before(Read::<Database>::one(pool));
	chain.link_before(Read::<TemplateCompilerKey>::one(template_compiler));
	chain.link_before(State::<SessionStorageKey>::one(session_storage));
	chain
}