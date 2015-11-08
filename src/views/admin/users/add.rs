use std::io::{Result, Error, ErrorKind};
use iron::prelude::*;
use persistent::Read;
use models::{User, UserRole};
use db::Database;
use views::utils::*;
use forms::*;


pub fn add_user(req: &mut Request) -> IronResult<Response> {
	let loc = match AddUserForm::from_request(req) {
		Ok(form) => {
			let connection = req.get::<Read<Database>>()
				.expect("Could not get connection pool in views::admin::user::add_user")
				.get().expect("Could not get connection in views::admin::user::add_user");
			match User::create(&connection, form.login, form.password, form.role, form.consumer_id) {
				Ok(_) => "/admin/users/?user_added",
				Err(err) => {
					println!("{:?}", err);
					"/admin/users/?user_not_added"
				}
			}
		}
		Err(err) => {
			println!("{:?}", err);
			"/admin/users/?user_not_added"
		}
	};
	redirect(loc.to_string())
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
	pub fn from_request(req: &mut Request) -> Result<Self> {
		let source = try!(get_body(req));
		Ok(AddUserForm {
			login: try!(Self::get_login(source.get("login"))),
			password: try!(Self::get_password(source.get("password"))),
			role: try!(Self::get_role(source.get("role"))),
			consumer_id: try!(Self::get_consumer_id(source.get("consumer_id"))),
		})
	}
}