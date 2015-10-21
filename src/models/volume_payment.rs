use super::Connection;
use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct VolumePayment {
	pub id: i32,
	pub volume: f32,
	pub consumer_id: i32,
	pub payment_date: NaiveDate,
	pub sum: f32,
}

impl VolumePayment {
	pub fn for_consumer(c: &Connection, consumer_id: i32) -> Vec<VolumePayment> {
        let stmt = c.prepare("SELECT id, volume, sum, payment_date FROM volume_payment where consumer_id = $1 ORDER BY payment_date").unwrap();
        let mut res = Vec::new();
        for row in stmt.query(&[&consumer_id]).unwrap() {
            res.push(VolumePayment {
                id: row.get(0),
                volume: row.get(1),
                sum: row.get(2),
                payment_date: row.get(3),
                consumer_id: consumer_id,
            });
    	}
        res
    }
	
	pub fn insert(c: &Connection, volume: f32, consumer_id: i32, payment_date: NaiveDate, sum: f32) {
		c.execute("INSERT INTO volume_payment (volume, consumer_id, payment_date, sum) VALUES ($1, $2, $3, $4)",
			&[&volume, &consumer_id, &payment_date, &sum]
		).unwrap();
	}
}
