use r2d2::{Pool, Config};
use r2d2_postgres::PostgresConnectionManager;
use postgres::SslMode;
use iron::typemap::Key;

pub type DbConnectionPool = Pool<PostgresConnectionManager>;

pub struct Database;

impl Key for Database { type Value = DbConnectionPool; }

pub fn get_db_connection_pool() -> DbConnectionPool {
    let config = Config::default();
    let manager = PostgresConnectionManager::new(
    	"postgresql://slnpacifist:postgres@localhost/water",
        SslMode::None)
    .unwrap();
    Pool::new(config, manager).unwrap()
}
