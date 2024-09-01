mod config;
mod db;
mod services;
mod routes;

use anyhow::Result;
use crate::services::axum_web_server::AxumWebServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = config::CONFIG.clone();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(config.logging.parse_level().unwrap_or(tracing::Level::INFO))
        .init();

    // Initialize database connection
    db::init_db(&config).await?;

    tracing::info!("Application started successfully");

    // Create and run the Axum web server
    let server = AxumWebServer::new(config);
    server.run().await;

    Ok(())
}
