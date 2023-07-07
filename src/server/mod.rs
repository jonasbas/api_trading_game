use std::{print, sync::Arc};

use axum::{routing::get, Router};
use tokio::signal;

use self::{
    db_connections::create_connection_pool,
    handlers::player::{create_player, display_player},
    state::ServerState,
};

mod db_connections;
mod handlers;
mod state;

pub async fn start_server() {
    let db_connections = create_connection_pool();

    print!("Create state.");
    let server_state = Arc::new(ServerState::new(db_connections));

    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/player/create", get(create_player))
        .route("/player/:id", get(display_player))
        .with_state(server_state);

    println!("Bind server.");
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
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
