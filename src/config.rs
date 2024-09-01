use config::{Config, ConfigError, Environment, File};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

use tracing::Level;

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

impl LoggingConfig {
    pub fn parse_level(&self) -> Result<Level, String> {
        self.level.parse().map_err(|_| format!("Invalid log level: {}", self.level))
    }
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub logging: LoggingConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        let config: AppConfig = s.try_deserialize()?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        // Add validation logic here
        // For example:
        if self.database.max_connections < self.database.min_connections {
            return Err(ConfigError::Message("max_connections must be greater than or equal to min_connections".into()));
        }
        Ok(())
    }

    pub fn get_database_url(&self) -> &str {
        &self.database.url
    }

    pub fn get_server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

pub static CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    Arc::new(AppConfig::new().expect("Failed to load configuration"))
});
