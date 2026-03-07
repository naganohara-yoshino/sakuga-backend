use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("An error occurred: {0}")]
    General(String),
}

pub type AppResult<T> = Result<T, AppError>;
