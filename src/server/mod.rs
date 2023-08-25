use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use diesel::r2d2::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tokio::signal;

use crate::server::{
    db_connections::get_connection,
    handlers::{
        cargo::list_cargo_infos,
        player::list_player_ships,
        port::list_all_ports,
        ship::{buy_ship_of_type, list_ship_types},
    },
};

use self::{
    db_connections::create_connection_pool,
    handlers::player::{create_player, display_player, rename_player},
    state::ServerState,
};

mod db_connections;
mod handlers;
mod state;
mod user_utils;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub async fn start_server() {
    let db_connections = create_connection_pool();
    apply_migrations(db_connections.clone());

    let server_state = Arc::new(ServerState::new(db_connections));

    let user_routes = Router::new()
        .route("/create", post(create_player))
        .route("/:id", get(display_player))
        .route("/:id/rename", post(rename_player))
        .route("/:id/ships", get(list_player_ships));

    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/ships", get(list_ship_types))
        .route("/ship/:ship_type_id/buy", post(buy_ship_of_type))
        .route("/cargo", get(list_cargo_infos))
        .route("/ports", get(list_all_ports))
        .nest("/player", user_routes)
        .with_state(server_state);

    println!("Bind server on port 3000.");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn apply_migrations(db_connections: Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>) {
    get_connection(db_connections)
        .expect("Error getting connection to apply migrations. Start aborted.")
        .run_pending_migrations(MIGRATIONS)
        .expect("Error applying migrations. Start aborted.");
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
