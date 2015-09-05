extern crate r2d2;
extern crate r2d2_postgres;
extern crate dtl;

use std;
use postgres::Result;
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

impl User {
	pub fn create(c: &Connection, login: String, password: String, role: UserRole, consumer_id: Option<i32>) -> Result<u64> {
		c.execute(
			"INSERT INTO \"user\" (login, password, role, consumer_id) VALUES ($1, crypt($2, gen_salt('bf', 8)), $3, $4)",
			&[&login, &password, &role.get_id(), &consumer_id]
		)
	}
	
	pub fn by_login_and_password(c: &Connection, login: String, password: String) -> Option<User> {
        let stmt = c.prepare("SELECT login, role, consumer_id FROM \"user\" WHERE login = $1 AND password = crypt($2, password);").unwrap();
        match stmt.query(&[&login, &password]).unwrap().iter().next() {
        	Some(row) => Some(User {
    			login: row.get(0),
    			role: UserRole::from_id(row.get(1)).unwrap(),
    			consumer_id: row.get(2),
    			consumer: None,
			}),
        	None => None
        }
	}
	
	pub fn all(c: &Connection) -> Vec<User> {
        let stmt = c.prepare("SELECT login, consumer_id, role FROM user LEFT JOIN consumer (address) ON (consumer_id = consumer.id)").unwrap();
        let mut res = Vec::new();
        for row in stmt.query(&[]).unwrap() {
        	let role_id: i32 = row.get(2);
        	let role = match role_id {
        		0 => UserRole::Admin,
        		_ => UserRole::User,
        	};
        	let consumer_id: Option<i32> = row.get(1);
        	let consumer = match consumer_id {
        		None => None,
        		Some(id) => {
        			Some(Consumer {
        				id: id,
        				address: row.get(3),
        			})
        		}
        	};
            res.push(User {
				login: row.get(0),
				consumer_id: consumer_id,
				consumer: consumer,
				role: role,
            });
    	}
        res
    }
}