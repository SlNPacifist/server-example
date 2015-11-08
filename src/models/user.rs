extern crate r2d2;
extern crate r2d2_postgres;
extern crate dtl;

use std;
use postgres::Result;
use postgres::rows::Row;
use super::{Connection, Consumer};

#[derive(Clone, Debug)]
pub enum UserRole {
	Admin,
	User,
}

impl UserRole {
	fn get_id(&self) -> i32 {
		match self {
			&UserRole::Admin => 0,
			&UserRole::User => 1,
		}
	}
	
	fn from_id(id: i32) -> std::result::Result<UserRole, String> {
		match id {
			0 => Ok(UserRole::Admin),
			1 => Ok(UserRole::User),
			_ => Err(format!("No role with id {} found", id)),
		}
	}
	
	pub fn is_admin(&self) -> bool {
		match self {
			&UserRole::Admin => true,
			_ => false
		}
	}
}

#[derive(Clone, Debug)]
pub struct User {
    pub login: String,
    pub consumer: Option<Consumer>,
    pub consumer_id: Option<i32>,
    pub role: UserRole,
}

fn row_to_user(row: Row) -> User {
	User {
		login: row.get(0),
		role: UserRole::from_id(row.get(1)).expect("Could not covert user role id"),
		consumer_id: row.get(2),
		consumer: None,
	}
}

impl User {
	pub fn create(
		c: &Connection,
		login: String,
		password: String,
		role: UserRole,
		consumer_id: Option<i32>) -> Result<u64> {

		c.execute(r#"
				INSERT INTO "user" (login, password, role, consumer_id)
				VALUES ($1, crypt($2, gen_salt('bf', 8)), $3, $4)"#,
				&[&login, &password, &role.get_id(), &consumer_id])
	}
	
	pub fn by_login_and_password(c: &Connection, login: String, password: String) -> Option<User> {
        c.prepare(r#"
	        	SELECT login, role, consumer_id FROM "user"
	        	WHERE login = $1 AND password = crypt($2, password)"#)
        	.expect("Could not prepare query for User::by_login_and_password")
        	.query(&[&login, &password])
        	.expect("Could not execute query for User::by_login_and_password")
        	.iter().next()
        	.map(row_to_user)
	}
	
	pub fn all(c: &Connection) -> Vec<User> {
		c.prepare(r#"
				SELECT login, role, consumer_id
				FROM "user"
				ORDER BY login"#)
			.expect("Could not prepare query for User::all")
			.query(&[])
			.expect("Could not execute query for User::all")
			.iter().map(row_to_user)
			.collect()
	}
}