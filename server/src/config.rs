use dotenvy::dotenv;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub data_in_memory: bool,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").map_err(|_| ConfigError::MissingEnv("DATABASE_URL".to_string()))?;
        let data_in_memory = std::env::var("DATA_IN_MEMORY").map(|v| v == "true").unwrap_or(false);

        Ok(AppConfig {
            database_url,
            data_in_memory,
        })
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid configuration: {0}")]
    MissingEnv(String),
}
