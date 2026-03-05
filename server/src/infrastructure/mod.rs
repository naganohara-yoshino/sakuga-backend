pub mod cache;
pub mod database;
pub mod storage;

use crate::{config::AppConfig, error::AppResult, state::AppState};

pub async fn init(config: &AppConfig) -> AppResult<AppState> {
    let db = database::connect(config).await?;
    let cache = cache::connect(config).await?;
    let storage = storage::create_operator(config).await?;

    Ok(AppState { db, cache, storage })
}
