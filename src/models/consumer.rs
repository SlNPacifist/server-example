extern crate r2d2;
extern crate r2d2_postgres;
extern crate dtl;

use postgres::rows::Row;
use chrono::NaiveDate;
use super::Connection;

#[derive(Clone, Debug)]
pub struct Consumer {
    pub id: i32,
    pub address: String,
    pub phone: String,
    pub name: String,
}

pub fn row_to_consumer(row: &Row) -> Consumer {
    Consumer {
        id: row.get(0),
        address: row.get(1),
        phone: row.get(2),
        name: row.get(3),
    }
}

impl Consumer {
    pub fn by_id(c: &Connection, id: i32) -> Option<Consumer> {
        c.prepare("SELECT id, address, phone, name FROM consumer where id = $1")
            .expect("Could not prepare query for Consumer::by_id")
            .query(&[&id])
            .expect("Could not execute query for Consumer::by_id")
            .iter().next()
            .map(|r| row_to_consumer(&r))
    }
    
    pub fn with_last_payment(c: &Connection) -> Vec<(Consumer, f32, Option<NaiveDate>)> {
        c.prepare("
                SELECT
                    c.id, c.address, c.phone, c.name,
                    COALESCE(SUM(vp.volume), 0) as total_volume,
                    MAX(vp.payment_date) as last_payment_date
                FROM consumer c
                LEFT JOIN volume_payment vp ON (c.id = vp.consumer_id)
                GROUP BY c.id
                ORDER BY c.address")
            .expect("Could not prepare query for Consumer::ordered_by_last_payment")
            .query(&[])
            .expect("Could not execute query for Consumer::ordered_by_last_payment")
            .iter().map(|row| {
                (row_to_consumer(&row),
                row.get(4),
                row.get(5))
            }).collect()
    }
    
    pub fn all(c: &Connection) -> Vec<Consumer> {
        c.prepare("
                SELECT id, address, phone, name
                FROM consumer c
                ORDER BY address")
            .expect("Could not prepare query for Consumer::all")
            .query(&[])
            .expect("Could not execute query for Consumer::all")
            .iter().map(|r| row_to_consumer(&r))
            .collect()
    }
    
    pub fn insert(c: &Connection, address: String, phone: String, name: String) {
        c.execute("INSERT INTO consumer (address, phone, name) VALUES ($1, $2, $3)",
				&[&address, &phone, &name])
            .expect("Could not execute query for Consumer::insert");
    }

	pub fn update(c: &Connection, id: i32, address: String, phone: String, name: String) {
		c.execute("UPDATE  consumer SET address=$1, phone=$2, name=$3 WHERE id=$4",
				&[&address, &phone, &name, &id])
			.expect("Could not execute query for Consumer::update");
	}
}