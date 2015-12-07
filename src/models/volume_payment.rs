use super::Connection;
use postgres::rows::Row;
use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct VolumePayment {
	pub id: i32,
	pub volume: f32,
	pub consumer_id: i32,
	pub payment_date: NaiveDate,
	pub sum: f32,
}

pub fn row_to_volume_payment(row: Row) -> VolumePayment {
    VolumePayment {
        id: row.get(0),
        volume: row.get(1),
        sum: row.get(2),
        payment_date: row.get(3),
        consumer_id: row.get(4),
    }
}

impl VolumePayment {
	pub fn for_consumer(c: &Connection, consumer_id: i32) -> Vec<VolumePayment> {
        c.prepare("
        	SELECT id, volume, sum, payment_date, consumer_id
        	FROM volume_payment WHERE consumer_id = $1
        	ORDER BY payment_date")
        .expect("Could not prepare query for VolumePayment::for_consumer")
        .query(&[&consumer_id])
		.expect("Could not execute query for VolumePayment::for_consumer")
    	.iter().map(row_to_volume_payment)
    	.collect()
    }
	
	pub fn insert(c: &Connection, volume: f32, consumer_id: i32, payment_date: NaiveDate, sum: f32) {
		c.execute("
			INSERT INTO volume_payment (volume, consumer_id, payment_date, sum)
			VALUES ($1, $2, $3, $4)",
			&[&volume, &consumer_id, &payment_date, &sum]
		).expect("Could not execute query for VolumePayment::insert");
	}
	
	pub fn by_id(c: &Connection, id: i32) -> Option<VolumePayment> {
        c.prepare("
        		SELECT id, volume, sum, payment_date, consumer_id
        		FROM volume_payment WHERE id = $1")
        	.expect("Could not prepare query for VolumePayment::by_id")
        	.query(&[&id])
        	.expect("Could not execute query for VolumePayment::by_id")
        	.iter().next()
        	.map(row_to_volume_payment)
	}
	
	pub fn up_to_date(c: &Connection, consumer_id: i32, payment_date: NaiveDate) -> Vec<VolumePayment> {
        c.prepare("
        	SELECT id, volume, sum, payment_date, consumer_id
        	FROM volume_payment
        	WHERE consumer_id = $1 AND payment_date <= $2
        	ORDER BY payment_date")
        .expect("Could not prepare query for VolumePayment::up_to_date")
        .query(&[&consumer_id, &payment_date])
		.expect("Could not execute query for VolumePayment::up_to_date")
    	.iter().map(row_to_volume_payment)
    	.collect()
	}
}
