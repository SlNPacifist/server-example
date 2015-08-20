mod main;
mod consumer;
mod static_files;

use std::path::PathBuf;
use dtl::TemplateCompiler;
use iron::prelude::*;
use iron::typemap::Key;
use persistent::Read;
use router::Router;
use db::{DbConnectionPool, Database};

pub struct TemplateCompilerKey;

impl Key for TemplateCompilerKey { type Value = TemplateCompiler; }


pub fn get_root(pool: DbConnectionPool) -> Chain {
	let mut router = Router::new();
	main::append_entry(&mut router);
	consumer::append_entry(&mut router);
	static_files::append_entry(&mut router);
    let mut root = PathBuf::new();
    root.push("/home/slnpacifist/eclipse_workspace/shop/src/templates");
    let template_compiler = TemplateCompiler::new(root).unwrap();
	let mut chain = Chain::new(router);
	chain.link_before(Read::<Database>::one(pool));
	chain.link_before(Read::<TemplateCompilerKey>::one(template_compiler));
	chain
}