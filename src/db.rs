use crate::config::AppConfig;
use anyhow::Result;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, Statement, ConnectionTrait, DbBackend};
use std::time::Duration;
use tracing::info;

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init_db(config: &AppConfig) -> Result<()> {
    let mut opt = ConnectOptions::new(config.database.url.clone());
    opt.max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect_timeout(Duration::from_secs(config.database.connect_timeout))
        .idle_timeout(Duration::from_secs(config.database.idle_timeout))
        .max_lifetime(Duration::from_secs(config.database.max_lifetime))
        .sqlx_logging(true);

    let db = Database::connect(opt).await?;

    // Perform a simple query to check if the connection is valid
    let result: Result<Option<i32>, sea_orm::DbErr> = db
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "SELECT 1",
            [],
        ))
        .await?
        .map(|qr| qr.try_get_by_index::<i32>(0))
        .transpose();

    match result {
        Ok(_) => info!("Database connection successful"),
        Err(e) => return Err(anyhow::anyhow!("Failed to connect to database: {}", e)),
    }

    DB.set(db)
        .map_err(|_| anyhow::anyhow!("Failed to set global database connection"))?;

    Ok(())
}

pub fn get_db() -> &'static DatabaseConnection {
    DB.get().expect("Database not initialized")
}
