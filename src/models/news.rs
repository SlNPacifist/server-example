extern crate r2d2;
extern crate r2d2_postgres;
extern crate dtl;

use chrono::NaiveDate;
use postgres::Result;
use postgres::rows::Row;
use super::Connection;

#[derive(Clone, Debug)]
pub struct News {
    pub id: i32,
    pub text: String,
    pub header: String,
    pub publication_date: NaiveDate,
}

fn row_to_news(row: Row) -> News {
	News {
		id: row.get(0),
		text: row.get(1),
		header: row.get(2),
		publication_date: row.get(3),
	}
}

impl News {
	pub fn ordered_by_date(c: &Connection, limit: i64) -> Vec<News> {
		c.prepare("
				SELECT id, text, header, publication_date
				FROM news
				ORDER BY publication_date DESC, id DESC
				LIMIT $1")
			.expect("Could not prepare query for News::ordered_by_date")
			.query(&[&limit])
			.expect("Could not execute query for News::ordered_by_date")
			.iter()
			.map(row_to_news).collect()
	}
	
	pub fn insert(c: &Connection, text: String, header: String, publication_date: NaiveDate) {
		c.execute("
			INSERT INTO news (text, header, publication_date)
			VALUES ($1, $2, $3)",
			&[&text, &header, &publication_date])
			.expect("Could not execute query for News::insert");
	}
	
	pub fn update(c: &Connection, id: i32, text: String, header: String) -> Result<()> {
		c.execute("
			UPDATE news SET text=$1, header=$2
			WHERE id = $3",
			&[&text, &header, &id]
		).map(|_| ())
	}
	
	pub fn by_id(c: &Connection, id: i32) -> Option<News> {
        c.prepare("SELECT id, text, header, publication_date FROM news WHERE id = $1")
        	.expect("Could not prepare query for News::by_id")
        	.query(&[&id])
        	.expect("Could not execute query for News::by_id")
        	.iter().next()
        	.map(row_to_news)
	}
}