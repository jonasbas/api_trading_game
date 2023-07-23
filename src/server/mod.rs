use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::signal;

use crate::server::handlers::ship::list_ship_types;

use self::{
    db_connections::create_connection_pool,
    handlers::player::{create_player, display_player, rename_player},
    state::ServerState,
};

mod db_connections;
mod handlers;
mod state;

pub async fn start_server() {
    let db_connections = create_connection_pool();

    let server_state = Arc::new(ServerState::new(db_connections));

    let user_routes = Router::new()
        .route("/create", post(create_player))
        .route("/:id", get(display_player))
        .route("/:id/rename", post(rename_player));

    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/ships", get(list_ship_types))
        .nest("/player", user_routes)
        .with_state(server_state);

    println!("Bind server on port 3000.");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Signal received, starting graceful shutdown");
}
