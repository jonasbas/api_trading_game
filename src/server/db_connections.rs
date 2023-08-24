use std::env;

use axum::http::StatusCode;
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

pub fn create_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL is not set!");
    let manager = ConnectionManager::<PgConnection>::new(url);

    println!("DB pool initiating");
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("error creating db connection pool")
}

pub fn get_connection(
    pool: Pool<ConnectionManager<PgConnection>>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, (StatusCode, String)> {
    let connection = pool.get();

    if connection.is_err() {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Couldn't create connection to database.".to_owned(),
        ));
    }

    Ok(connection.unwrap())
}
