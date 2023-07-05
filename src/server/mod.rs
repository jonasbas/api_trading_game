use std::sync::Arc;

use axum::{routing::get, Router};

use self::{db_connections::create_connection_pool, state::ServerState};

mod db_connections;
mod state;

pub async fn start_server() {
    let db_connections = create_connection_pool();
    let server_state = Arc::new(ServerState::new(db_connections));

    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .with_state(server_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
