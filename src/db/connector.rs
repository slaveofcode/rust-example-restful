use diesel::{PgConnection, r2d2::ConnectionManager};
use r2d2::{Pool};
// use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool(database_url: String, max_conn: Option<u32>) -> PgPool {
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(max_conn.unwrap_or(3))
        .build(manager)
        .expect("Could not initial database pool manager")
}