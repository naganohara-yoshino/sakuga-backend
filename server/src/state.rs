use opendal::Operator;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub cache: ConnectionManager,
    pub storage: Operator,
}
