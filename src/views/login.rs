use std::io::{Result, Error, ErrorKind};
use iron::prelude::*;
use iron::status;
use iron::headers::*;
use persistent::{Read, State};
use iron_mountrouter::{Router, MethodPicker};
use oatmeal_raisin as or;
use dtl::{Context, HashMapContext};
use urlencoded::{QueryMap, UrlEncodedBody, UrlEncodedQuery};
use models::User;
use db::Database;
use views::SessionStorageKey;
use views::utils::{render_status, redirect};
use forms::*;
use session::{Session, SessionStorage};


fn entry(req: &mut Request) -> IronResult<Response> {
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
	render_status(req, &ctx, "login.htmt", res_status)
}

pub fn login_user(req: &mut Request) -> IronResult<Response> {
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
					let is_sanitized = match form.next {
						Some(ref url) if url.starts_with("//") => false,
						Some(ref url) if url.starts_with("/") => true,
						_ => false
					};
					match is_sanitized {
						true => form.next.unwrap(),
						false => "/".to_string()
					}
				}
				None => {
					"?user_not_logged_in".to_string()
				}
			}
		}
		Err(err) => {
			println!("{:?}", err);
			"?user_not_logged_in".to_string()
		}
	};
	redirect(location)
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
	let mut picker = MethodPicker::new();
	picker.get(entry);
	
	let mut chain = Chain::new(login_user);
	chain.link_after(or::SetCookie);
	picker.post(chain);
	
	router.add_route("/login/", picker, false);
}