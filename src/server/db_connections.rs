use std::env;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub fn create_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL is not set!");
    let manager = ConnectionManager::<PgConnection>::new(url);

    println!("DB pool initiating");
    Pool::builder()
        .test_on_check_out(true)
        //TODO: why does it only work if max size is set to 2?
        .max_size(2)
        .build(manager)
        .expect("Error creating database connection pool!")
}
