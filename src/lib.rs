mod models;
mod schema;

use axum::{routing::get, Router};
use dotenv::dotenv;

pub async fn start_server() {
    dotenv().ok();

    let app = Router::new().route("/", get(|| async { "Hello world" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
