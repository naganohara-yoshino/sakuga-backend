use crate::error::AppResult;
use config::Config;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub postgres_host: String,
    pub postgres_port: u16,
    pub postgres_db: String,
    pub postgres_user: String,
    pub postgres_password: String,
    pub redis_host: String,
    pub redis_port: u16,
    pub redis_password: String,
    pub s3_access_key_id: String,
    pub s3_secret_access_key: String,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_region: String,
}

impl AppConfig {
    pub fn load() -> AppResult<Self> {
        dotenvy::dotenv().ok();

        let config = Config::builder()
            .add_source(config::Environment::with_prefix("SERVER").keep_prefix(true))
            .add_source(config::Environment::with_prefix("POSTGRES").keep_prefix(true))
            .add_source(config::Environment::with_prefix("REDIS").keep_prefix(true))
            .add_source(config::Environment::with_prefix("S3").keep_prefix(true))
            .build()?;

        Ok(config.try_deserialize()?)
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
