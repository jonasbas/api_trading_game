use api_trading_game::bootstrap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bootstrap().await;

    Ok(())
}
