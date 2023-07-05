use api_trading_game::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}
