extern crate r2d2;
extern crate r2d2_postgres;
extern crate dtl;

pub struct Consumer {
    pub id: i32,
    pub address: String
}

impl Consumer {
	pub fn all(c: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>) -> Vec<Consumer> {
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
}

pub struct VolumePayment {
	id: i32,
	volume: f32,
	consumer_id: i32
}

//fn main() {
//    let conn = Connection::connect("postgresql://postgres@localhost", &SslMode::None)
//            .unwrap();
//
//    conn.execute("CREATE TABLE person (
//                    id              SERIAL PRIMARY KEY,
//                    name            VARCHAR NOT NULL,
//                    data            BYTEA
//                  )", &[]).unwrap();
//    let me = Person {
//        id: 0,
//        name: "Steven".to_string(),
//        data: None
//    };
//    conn.execute("INSERT INTO person (name, data) VALUES ($1, $2)",
//                 &[&me.name, &me.data]).unwrap();
//
//    let stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
//    for row in stmt.query(&[]).unwrap() {
//        let person = Person {
//            id: row.get(0),
//            name: row.get(1),
//            data: row.get(2)
//        };
//        println!("Found person {}", person.name);
//    }
//}