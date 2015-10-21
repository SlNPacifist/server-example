use r2d2::{Pool, Config};
use r2d2_postgres::PostgresConnectionManager;
use postgres::SslMode;
use iron::typemap::Key;
use backport::ResultExpect;

pub type DbConnectionPool = Pool<PostgresConnectionManager>;

pub struct Database;

impl Key for Database { type Value = DbConnectionPool; }

pub fn get_db_connection_pool(connection_string: &str) -> DbConnectionPool {
    let config = Config::default();
    let manager = PostgresConnectionManager::new(connection_string, SslMode::None).expect_b(
    	&format!("Could not connect to database {}", connection_string));
    Pool::new(config, manager).expect_b("Could not create connection pool")
}
