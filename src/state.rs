use sea_orm::Database;

use crate::env;


#[derive(Clone, Debug)]
pub struct AppState {
    pub database: sea_orm::DatabaseConnection,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            database: Database::connect(&env::ENV.database_url).await.unwrap(),
        }
    }
}
