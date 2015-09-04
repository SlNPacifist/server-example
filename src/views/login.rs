use std::path::Path;
use std::io::{Result, Error, ErrorKind};
use iron::prelude::*;
use iron::status;
use iron::headers::*;
use persistent::{Read, State};
use router::Router;
use oatmeal_raisin as or;
use dtl::{Context, HashMapContext};
use urlencoded::{QueryMap, UrlEncodedBody, UrlEncodedQuery};
use models::User;
use db::Database;
use views::{TemplateCompilerKey, SessionStorageKey};
use forms::*;
use session::{Session, SessionStorage};


fn entry(req: &mut Request) -> IronResult<Response> {
	let template_compiler = req.get::<Read<TemplateCompilerKey>>().unwrap();
    let mut ctx = HashMapContext::new();
	let res_status = {
		match req.get_ref::<UrlEncodedQuery>() {
			Ok(url_query) => {
				if let Ok(next_url) = parse_single_field(url_query.get("next"), "") {
					ctx.set("next", Box::new(next_url.to_string()));
				}
				match parse_single_field(url_query.get("reason"), "") {
					Ok("forbidden") => {
						ctx.set("is_forbidden", Box::new(true));
						status::Forbidden
					}
					_ => status::Ok,
				}
			},
			_ => status::Ok,
		}
	};
    let response_text = template_compiler.render(Path::new("login.htmt"), &ctx).unwrap();
    let mut res = Response::with((res_status, response_text));
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
					Location(form.next.unwrap_or("/".to_string()))
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
	next: Option<String>,
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
	fn get_next(source: Option<&Vec<String>>) -> Option<String> {
		match parse_single_field(source, "next") {
			Ok(next) => Some(next.to_string()),
			_ => None,
		}
	}
	pub fn new(source: &QueryMap) -> Result<Self> {
		Ok(UserLoginForm {
			login: try!(Self::get_login(source.get("login"))),
			password: try!(Self::get_password(source.get("password"))),
			next: Self::get_next(source.get("next")),
		})
	}
}

pub fn append_entry(router: &mut Router) {
	router.get("/login/", entry);
	let mut chain = Chain::new(login_user);
	chain.link_after(or::SetCookie);
	router.post("/login/", chain);
}