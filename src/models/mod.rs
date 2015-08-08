extern crate r2d2;
extern crate r2d2_postgres;
extern crate dtl;

pub mod consumer;
pub mod volume_payment;

pub type Connection = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;
pub use self::consumer::Consumer;
pub use self::volume_payment::VolumePayment;