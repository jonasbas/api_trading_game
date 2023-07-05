mod models;
mod schema;
mod server;

use dotenv::dotenv;

pub async fn bootstrap() {
    dotenv().ok();
    server::start_server().await;
}
