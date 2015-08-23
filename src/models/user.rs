extern crate r2d2;
extern crate r2d2_postgres;
extern crate dtl;

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
}

#[derive(Clone, Debug)]
pub struct User {
    pub login: String,
    pub consumer: Option<Consumer>,
    pub consumer_id: Option<i32>,
    pub role: UserRole,
}

impl User {
//	pub fn by_login(c: &Connection, login: &str) -> Option<User> {
//        let stmt = c.prepare("SELECT user FROM user where id = $1").unwrap();
//        match stmt.query(&[&id]).unwrap().iter().next() {
//        	Some(row) => Some(Consumer { id: id, address: row.get(0) }),
//        	None => None
//        }
//	}
	
	pub fn create(c: &Connection, login: String, password: String, role: UserRole, consumer_id: Option<i32>) -> Result<u64> {
		c.execute(
			"INSERT INTO \"user\" (login, password, role, consumer_id) VALUES ($1, crypt($2, gen_salt('bf', 8)), $3, $4)",
			&[&login, &password, &role.get_id(), &consumer_id]
		)
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