use std::path::Path;
use std::io::{Result, Error, ErrorKind};
use iron::prelude::*;
use iron::status;
use iron::headers::*;
use persistent::{Read, State};
use router::Router;
use oatmeal_raisin as or;
use dtl::{Context, HashMapContext};
use urlencoded::{QueryMap, UrlEncodedBody};
use models::User;
use db::Database;
use views::{TemplateCompilerKey, SessionStorageKey};
use forms::*;
use session::{Session, SessionStorage};


fn entry(req: &mut Request) -> IronResult<Response> {
	let template_compiler = req.get::<Read<TemplateCompilerKey>>().unwrap();
    let ctx = HashMapContext::new();
    let response_text = template_compiler.render(Path::new("login.htmt"), &ctx).unwrap();
    let mut res = Response::with((status::Ok, response_text));
    res.headers.set(ContentType::html());
    Ok(res)
}

pub fn login_user(req: &mut Request) -> IronResult<Response> {
	let mut res = Response::with(status::SeeOther);
	let location = match UserLoginForm::new(&req.get::<UrlEncodedBody>().unwrap()) {
		Ok(form) => {
			let connection = req.get::<Read<Database>>().unwrap().get().unwrap();
			match User::by_login_and_password(&connection, form.login, form.password) {
				Some(user) => {
					let session = Session::new(user);
					{
						let jar = req.get_mut::<or::CookieJar>().unwrap();
						jar.add(or::Cookie::new("session-id".into(), session.id.clone().into()));
					}
					{
						let arc_session_storage = req.get::<State<SessionStorageKey>>().unwrap();
						let mut session_storage = arc_session_storage.write().unwrap();
						session_storage.insert(session);
					}
					Location("/".to_string())
				}
				None => {
					Location("?user_not_logged_in".to_string())
				}
			}
		}
		Err(err) => {
			println!("{:?}", err);
			Location("?user_not_logged_in".to_string())
		}
	};
	res.headers.set(location);
	Ok(res)
}

#[derive(Debug, Clone)]
struct UserLoginForm {
	login: String,
	password: String,
}

impl UserLoginForm {
	fn get_login(source: Option<&Vec<String>>) -> Result<String> {
		let login = try!(parse_single_field(source, "login")).to_string();
		if login.len() == 0 {
			Err(Error::new(ErrorKind::InvalidInput, "login field is empty"))
		} else {
			Ok(login)
		}
	}
	fn get_password(source: Option<&Vec<String>>) -> Result<String> {
		let password = try!(parse_single_field(source, "password"));
		Ok(password.to_string())
	}
	pub fn new(source: &QueryMap) -> Result<Self> {
		Ok(UserLoginForm {
			login: try!(Self::get_login(source.get("login"))),
			password: try!(Self::get_password(source.get("password"))),
		})
	}
}

pub fn append_entry(router: &mut Router) {
	router.get("/login/", entry);
	let mut chain = Chain::new(login_user);
	chain.link_after(or::SetCookie);
	router.post("/login/", chain);
}