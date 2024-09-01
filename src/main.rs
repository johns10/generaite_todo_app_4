mod config;
mod db;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = &config::CONFIG;

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(config.logging.level.parse()?)
        .init();

    // Initialize database connection
    db::init_db(config).await?;

    info!("Application started successfully");

    // Your application logic here

    Ok(())
}
