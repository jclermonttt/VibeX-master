use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ApiState {
    pub db: Arc<RwLock<DatabaseConnection>>,
}
