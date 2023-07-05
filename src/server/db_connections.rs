use std::env;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub fn create_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL is not set!");
    let manager = ConnectionManager::<PgConnection>::new(url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Error creting database connection pool!")
}
