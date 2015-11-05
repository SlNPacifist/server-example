extern crate r2d2;
extern crate r2d2_postgres;
extern crate dtl;

use chrono::NaiveDate;
use super::Connection;

#[derive(Clone, Debug)]
pub struct News {
    pub id: i32,
    pub text: String,
    pub header: String,
    pub publication_date: NaiveDate,
}

impl News {
	pub fn ordered_by_date(c: &Connection, limit: i64) -> Vec<News> {
		let stmt = c.prepare("
				SELECT id, text, header, publication_date
				FROM news
				ORDER BY publication_date DESC, id DESC
				LIMIT $1
			").expect("Could not prepare query for News::ordered_by_date");
		stmt.query(&[&limit])
			.expect("Could not execute query for News::ordered_by_date")
			.iter().map(|row| {
				News {
					id: row.get(0),
					text: row.get(1),
					header: row.get(2),
					publication_date: row.get(3),
				}
			}).collect()
	}
	
	pub fn insert(c: &Connection, text: String, header: String, publication_date: NaiveDate) {
		c.execute("
			INSERT INTO news (text, header, publication_date)
			VALUES ($1, $2, $3)",
			&[&text, &header, &publication_date])
			.expect("Could not execute query for News::insert");
	}
	
	pub fn by_id(c: &Connection, id: i32) -> Option<News> {
        let stmt = c
        	.prepare("SELECT id, text, header, publication_date FROM consumer where id = $1")
        	.expect("Could not prepare query for News::by_id");
        stmt.query(&[&id])
        	.expect("Could not execute query for News::by_id")
        	.iter().next()
        	.map(|r| {

        	News {
				id: r.get(0),
				text: r.get(1),
				header: r.get(2),
				publication_date: r.get(3),
			}
        })
	}
}