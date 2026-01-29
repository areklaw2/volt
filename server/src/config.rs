use dotenvy::dotenv;
use secrecy::SecretString;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub clerk_secret_key: SecretString,
    pub database_url: SecretString,
    pub data_in_memory: bool,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let clerk_secret_key = std::env::var("CLERK_SECRET_KEY").map_err(|e| ConfigError::MissingEnv(e.to_string()))?;
        let database_url = std::env::var("DATABASE_URL").map_err(|e| ConfigError::MissingEnv(e.to_string()))?;
        let data_in_memory = std::env::var("DATA_IN_MEMORY").map(|v| v == "true").unwrap_or(false);
        let port = std::env::var("PORT").ok().and_then(|v| v.parse::<u16>().ok()).unwrap_or(3000);

        Ok(AppConfig {
            clerk_secret_key: clerk_secret_key.into(),
            database_url: database_url.into(),
            data_in_memory,
            port,
        })
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid configuration: {0}")]
    MissingEnv(String),
}
