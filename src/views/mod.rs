mod utils;
mod main;
mod admin;
mod login;
mod static_files;

use std::path::PathBuf;
use std::sync::Arc;
use dtl::{TemplateCompiler, HashMapContext};
use iron::prelude::*;
use iron::typemap::Key;
use iron::middleware::BeforeMiddleware;
use persistent::{Read, State};
use iron_mountrouter::Router;
use db::{DbConnectionPool, Database};
use session::MemorySessionStorage;

pub struct TemplateCompilerKey;
impl Key for TemplateCompilerKey { type Value = Arc<TemplateCompiler>; }

pub struct SessionStorageKey;
impl Key for SessionStorageKey { type Value = MemorySessionStorage; }

pub struct ContextKey;
impl Key for ContextKey { type Value = HashMapContext; }

struct ContextPreprocessor;

impl BeforeMiddleware for ContextPreprocessor {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		req.extensions.insert::<ContextKey>(HashMapContext::new());
		Ok(())
	}
}

struct TemplateCompilerPreprocessor {
	compiler: Arc<TemplateCompiler>
}

impl TemplateCompilerPreprocessor {
	fn new(compiler: TemplateCompiler) -> Self {
		TemplateCompilerPreprocessor {
			compiler: Arc::new(compiler)
		}
	}
}

impl BeforeMiddleware for TemplateCompilerPreprocessor {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		req.extensions.insert::<TemplateCompilerKey>(self.compiler.clone());
		Ok(())
	}
}

pub fn get_root(pool: DbConnectionPool) -> Chain {
	let mut router = Router::new();
	main::append_entry(&mut router);
	admin::append_entry(&mut router);
	login::append_entry(&mut router);
	static_files::append_entry(&mut router);
    let mut root = PathBuf::new();
    root.push("./src/templates");
    let template_compiler = TemplateCompiler::new(root)
    	.expect("Could not create template compiler in view::get_root");
    let template_compiler_preprocessor = TemplateCompilerPreprocessor::new(template_compiler);
    let session_storage = MemorySessionStorage::new();
	let mut chain = Chain::new(router);
	chain.link_before(Read::<Database>::one(pool));
	chain.link_before(template_compiler_preprocessor);
	chain.link_before(ContextPreprocessor);
	chain.link_before(State::<SessionStorageKey>::one(session_storage));
	chain
}