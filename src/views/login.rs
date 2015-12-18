use std::io::{Result, Error, ErrorKind};
use iron::prelude::*;
use iron::status;
use iron::headers::*;
use persistent::{Read, State};
use iron_mountrouter::Router;
use oatmeal_raisin as or;
use urlencoded::UrlEncodedQuery;
use models::User;
use db::Database;
use views::SessionStorageKey;
use views::utils::*;
use forms::*;
use session::{Session, SessionStorage};


fn entry(req: &mut Request) -> IronResult<Response> {
    let mut next = None;
    let mut is_forbidden = false;
	if let Ok(url_query) = req.get_ref::<UrlEncodedQuery>() {
		if let Ok(next_url) = parse_single_field(url_query.get("next"), "") {
			next = Some(next_url.to_string());
		}
		match parse_single_field(url_query.get("reason"), "") {
			Ok("forbidden") => is_forbidden = true,
			_ => {}
		}
	};
	if let Some(n) = next {
		update_var(req, "next", n);
	}
	let res_status = if is_forbidden {
		update_var(req, "is_forbidden", true);
		status::Forbidden
	} else {
		status::Ok
	};
	render_status(req, "login.htmt", res_status)
}

pub fn login_user(req: &mut Request) -> IronResult<Response> {
	let location = match UserLoginForm::from_request(req) {
		Ok(form) => {
			let connection = req.get::<Read<Database>>()
				.expect("Could not get connection pool in views::login::login_user")
				.get().expect("Could not get connection in views::login::login_user");
			match User::by_login_and_password(&connection, form.login, form.password) {
				Some(user) => {
					let session = Session::new(user);
					{
						let jar = req.get_mut::<or::CookieJar>()
							.expect("Could not get cookie storage in views::login::login_user");
						let mut cookie = or::Cookie::new("session-id".into(), session.id.clone().into());
						cookie.path = Some("/".to_string());
						jar.add(cookie);
					}
					{
						let arc_session_storage = req.get::<State<SessionStorageKey>>()
							.expect("Could not get session storage in views::login::login_user");
						let mut session_storage = arc_session_storage.write()
							.expect("Could not add new session in views::login::login_user");
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
	pub fn from_request(req: &mut Request) -> Result<Self> {
		let source = try!(get_body(req));
		Ok(UserLoginForm {
			login: try!(Self::get_login(source.get("login"))),
			password: try!(Self::get_password(source.get("password"))),
			next: Self::get_next(source.get("next")),
		})
	}
}

pub fn append_entry(router: &mut Router) {
	let mut chain = Chain::new(login_user);
	chain.link_after(or::SetCookie);
	router.add_route("/login/", picker!(get => entry, post => chain), false);
}