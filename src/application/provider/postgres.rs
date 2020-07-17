use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
use std::env;

use super::SQLError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

//embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn get_connection() -> Result<DbConnection, SQLError> {
    POOL.get()
        .map_err(|e| SQLError::new(format!("Failed getting db connection: {}", e)))
}

pub fn new_pooled_connection() {
    lazy_static::initialize(&POOL);
}
