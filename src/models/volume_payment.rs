use super::Connection;

#[derive(Clone, Debug)]
pub struct VolumePayment {
	pub id: i32,
	pub volume: f32,
	pub consumer_id: i32
}

impl VolumePayment {
	pub fn for_consumer(c: &Connection, consumer_id: i32) -> Vec<VolumePayment> {
        let stmt = c.prepare("SELECT id, volume FROM volume_payment where consumer_id = $1").unwrap();
        let mut res = Vec::new();
        for row in stmt.query(&[&consumer_id]).unwrap() {
            res.push(VolumePayment {
                id: row.get(0),
                volume: row.get(1),
                consumer_id: consumer_id,
            });
    	}
        res
    }
}
