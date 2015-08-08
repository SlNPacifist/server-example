extern crate r2d2;
extern crate r2d2_postgres;
extern crate dtl;

use super::Connection;

#[derive(Clone, Debug)]
pub struct Consumer {
    pub id: i32,
    pub address: String
}

impl Consumer {
	pub fn all(c: &Connection) -> Vec<Consumer> {
        let stmt = c.prepare("SELECT id, address FROM consumer").unwrap();
        let mut res = Vec::new();
        for row in stmt.query(&[]).unwrap() {
            res.push(Consumer {
                id: row.get(0),
                address: row.get(1)
            });
    	}
        res
    }
	
	pub fn by_id(c: &Connection, id: i32) -> Option<Consumer> {
        let stmt = c.prepare("SELECT address FROM consumer where id = $1").unwrap();
        match stmt.query(&[&id]).unwrap().iter().next() {
        	Some(row) => Some(Consumer { id: id, address: row.get(0) }),
        	None => None
        }
	}
}