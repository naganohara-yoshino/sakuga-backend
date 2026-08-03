use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
    #[error("Cache error: {0}")]
    Cache(#[from] redis::RedisError),
    #[error("Storage error: {0}")]
    Storage(#[from] opendal::Error),
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("An error occurred: {0}")]
    General(String),
}

pub type AppResult<T> = Result<T, AppError>;
