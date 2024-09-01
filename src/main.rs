mod config;
mod db;

use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = config::AppConfig::new()?;

    // Initialize database connection
    db::init_db(&config).await?;

    info!("Application started successfully");

    // Your application logic here

    Ok(())
}
