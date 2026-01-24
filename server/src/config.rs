use dotenvy::dotenv;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnv("DATABASE_URL".to_string()))?;

        Ok(AppConfig { database_url })
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid configuration: {0}")]
    MissingEnv(String),
}
