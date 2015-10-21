use std::path::Path;
use std::io::{Result, Error, ErrorKind};
use iron::prelude::*;
use iron::status;
use iron::headers::*;
use iron_mountrouter::{Router, MethodPicker};
use persistent::Read;
use urlencoded::{QueryMap, UrlEncodedBody};
use dtl::{Context, HashMapContext};
use models::{User, UserRole};
use db::Database;
use views::TemplateCompilerKey;
use forms::*;


pub fn append_entry(router: &mut Router) {
	let mut subrouter = Router::new();

	let mut add_user_picker = MethodPicker::new();
	add_user_picker.post(add_user);
	add_user_picker.get(entry);
	subrouter.add_route("/add/", add_user_picker, false);
	
	router.add_route("/user/", subrouter, true);
}

pub fn entry(req: &mut Request) -> IronResult<Response> {
	let template_compiler = req.get::<Read<TemplateCompilerKey>>().unwrap();
    let ctx = HashMapContext::new();
    let response_text = template_compiler.render(Path::new("admin/add_user.htmt"), &ctx).unwrap();
    let mut res = Response::with((status::Ok, response_text));
    res.headers.set(ContentType::html());
    Ok(res)
}

pub fn add_user(req: &mut Request) -> IronResult<Response> {
	let mut res = Response::with(status::SeeOther);
	let location = match AddUserForm::new(&req.get::<UrlEncodedBody>().unwrap()) {
		Ok(form) => {
			let connection = req.get::<Read<Database>>().unwrap().get().unwrap();
			match User::create(&connection, form.login, form.password, form.role, form.consumer_id) {
				Ok(_) => Location("/admin/user/add/?user_added".to_string()),
				Err(err) => {
					println!("{:?}", err);
					Location("/admin/user/add/?user_not_added".to_string())
				}
			}
		}
		Err(err) => {
			println!("{:?}", err);
			Location("/admin/user/add/?user_not_added".to_string())
		}
	};
	res.headers.set(location);
	Ok(res)
}

#[derive(Debug, Clone)]
struct AddUserForm {
	login: String,
	password: String,
	role: UserRole,
	consumer_id: Option<i32>,
}

impl AddUserForm {
	fn get_login(source: Option<&Vec<String>>) -> Result<String> {
		let login = try!(parse_single_field(source, "login")).to_string();
		if login.len() == 0 {
			Err(Error::new(ErrorKind::InvalidInput, "login field is empty"))
		} else {
			Ok(login)
		}
	}
	fn get_password(source: Option<&Vec<String>>) -> Result<String> {
		let password = try!(parse_single_field(source, "password")).to_string();
		if password.len() == 0 {
			Err(Error::new(ErrorKind::InvalidInput, "password field is empty"))
		} else {
			Ok(password)
		}
	}
	fn get_role(source: Option<&Vec<String>>) -> Result<UserRole> {
		let role_id = try!(parse_single_i32(source, "role"));
		match role_id {
			0 => Ok(UserRole::Admin),
			1 => Ok(UserRole::User),
			_ => Err(Error::new(ErrorKind::InvalidInput, format!("unknown user role id: {}", role_id)))
		}
	}
	fn get_consumer_id(source: Option<&Vec<String>>) -> Result<Option<i32>> {
		let consumer_id = try!(parse_single_i32(source, "consumer_id"));
		match consumer_id {
			-1 => Ok(None),
			_ if consumer_id >= 0 => Ok(Some(consumer_id)),
			_ => Err(Error::new(ErrorKind::InvalidInput, format!("consumer_id field form is less then 0: {}", consumer_id)))
		}
	}
	pub fn new(source: &QueryMap) -> Result<Self> {
		Ok(AddUserForm {
			login: try!(Self::get_login(source.get("login"))),
			password: try!(Self::get_password(source.get("password"))),
			role: try!(Self::get_role(source.get("role"))),
			consumer_id: try!(Self::get_consumer_id(source.get("consumer_id"))),
		})
	}
}