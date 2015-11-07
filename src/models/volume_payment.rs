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
        c.prepare("
        	SELECT id, volume, sum, payment_date
        	FROM volume_payment where consumer_id = $1
        	ORDER BY payment_date")
        .expect("Could not prepare query for VolumePayment::for_consumer")
        .query(&[&consumer_id])
		.expect("Could not execute query for VolumePayment::for_consumer")
    	.iter().map(|row| {
            VolumePayment {
                id: row.get(0),
                volume: row.get(1),
                sum: row.get(2),
                payment_date: row.get(3),
                consumer_id: consumer_id,
            }
		}).collect()
    }
	
	pub fn insert(c: &Connection, volume: f32, consumer_id: i32, payment_date: NaiveDate, sum: f32) {
		c.execute("
			INSERT INTO volume_payment (volume, consumer_id, payment_date, sum)
			VALUES ($1, $2, $3, $4)",
			&[&volume, &consumer_id, &payment_date, &sum]
		).expect("Could not execute query for VolumePayment::insert");
	}
}
