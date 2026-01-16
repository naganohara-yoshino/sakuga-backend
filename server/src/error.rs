use anyhow::Error;

// Temporary
pub type AppError = Error;

pub type AppResult<T> = Result<T, AppError>;
