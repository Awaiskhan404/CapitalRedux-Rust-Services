use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError},
};
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Result<PgPool, PoolError> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)?;
    Ok(pool)
}