use sea_orm::{ConnectOptions, Database, prelude::*};
use std::time::Duration;

use crate::{config::AppConfig, error::AppResult};

pub async fn connect(config: &AppConfig) -> AppResult<DatabaseConnection> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.postgres_user,
        config.postgres_password,
        config.postgres_host,
        config.postgres_port,
        config.postgres_db
    );
    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(4)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(20))
        .acquire_timeout(Duration::from_secs(20))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(true);

    Ok(Database::connect(opt).await?)
}
