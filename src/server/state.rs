use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

#[derive(Debug)]
pub struct ServerState {
    pub db_connections: Pool<ConnectionManager<PgConnection>>,
}

impl ServerState {
    pub fn new(db_connections: Pool<ConnectionManager<PgConnection>>) -> Self {
        ServerState { db_connections }
    }
}
