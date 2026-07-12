use dotenvy::dotenv;
use secrecy::SecretString;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: SecretString,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").map_err(|e| ConfigError::MissingEnv(e.to_string()))?;
        let port = std::env::var("PORT").ok().and_then(|v| v.parse::<u16>().ok()).unwrap_or(3000);

        Ok(AppConfig {
            database_url: database_url.into(),
            port,
        })
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid configuration: {0}")]
    MissingEnv(String),
}
