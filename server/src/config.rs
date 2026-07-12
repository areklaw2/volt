use dotenvy::dotenv;
use secrecy::SecretString;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: SecretString,
    pub port: u16,
    pub upload_dir: String,
    pub public_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").map_err(|e| ConfigError::MissingEnv(e.to_string()))?;
        let port = std::env::var("PORT").ok().and_then(|v| v.parse::<u16>().ok()).unwrap_or(3000);
        let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads".to_string());
        let public_url = std::env::var("PUBLIC_URL").unwrap_or_else(|_| format!("http://127.0.0.1:{port}"));

        Ok(AppConfig {
            database_url: database_url.into(),
            port,
            upload_dir,
            public_url,
        })
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid configuration: {0}")]
    MissingEnv(String),
}
