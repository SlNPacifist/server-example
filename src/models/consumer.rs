extern crate r2d2;
extern crate r2d2_postgres;
extern crate dtl;

use chrono::NaiveDate;
use super::Connection;

#[derive(Clone, Debug)]
pub struct Consumer {
    pub id: i32,
    pub address: String
}

impl Consumer {
	pub fn by_id(c: &Connection, id: i32) -> Option<Consumer> {
        let stmt = c.prepare("SELECT address FROM consumer where id = $1").expect(
        	"Could not prepare query for Consumer::by_id");
        match stmt.query(&[&id]).expect("Could not execute query for Consumer::by_id").iter()
        	.next() {
        		
        	Some(row) => Some(Consumer { id: id, address: row.get(0) }),
        	None => None
        }
	}
	
	pub fn ordered_by_last_payment(c: &Connection) -> Vec<(Consumer, f32, Option<NaiveDate>)> {
		let stmt = c.prepare("
				SELECT
					c.id, c.address,
					COALESCE(SUM(vp.volume), 0) as total_volume,
					MAX(vp.payment_date) as last_payment_date
				FROM consumer c
				LEFT JOIN volume_payment vp ON (c.id = vp.consumer_id)
				GROUP BY c.id
				ORDER BY last_payment_date NULLS FIRST
			").expect("Could not prepare query for Consumer::ordered_by_last_payment");
		let mut res = Vec::new();
		for row in stmt.query(&[]).expect(
			"Could not execute query for Consumer::ordered_by_last_payment") {
				
			res.push((
				Consumer {id: row.get(0), address: row.get(1)},
				row.get(2),
				row.get(3),
			));
		}
		res
	}
	
	pub fn insert(c: &Connection, address: String) {
		c.execute("INSERT INTO consumer (address) VALUES ($1)",
			&[&address]
		).expect("Could not execute query for Consumer::insert");
	}
}