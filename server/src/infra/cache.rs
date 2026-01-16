use redis::aio::ConnectionManager;

use crate::{config::AppConfig, error::AppResult};

pub async fn connect(config: &AppConfig) -> AppResult<ConnectionManager> {
    let redis_url = format!(
        "redis://:{}@{}:{}",
        &config.redis_password, &config.redis_host, config.redis_port
    );
    let client = redis::Client::open(redis_url)?;
    let manager = ConnectionManager::new(client).await?;

    Ok(manager)
}
