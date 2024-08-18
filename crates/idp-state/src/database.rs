use std::time::Duration;

use idp_config::DatabaseConfig;
use sea_orm::{ConnectOptions, Database as SeaDatabase, DatabaseConnection};

use crate::StateError;

pub struct Database;

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<DatabaseConnection, StateError> {
        let db_uri = config.uri().as_str();
        let mut connection_ops = ConnectOptions::new(db_uri);

        connection_ops
            .max_connections(*config.max_connections())
            .min_connections(*config.min_connections())
            .connect_timeout(Duration::from_secs(*config.connection_timeout() as u64))
            .acquire_timeout(Duration::from_secs(*config.acquire_timeout() as u64))
            .idle_timeout(Duration::from_secs(*config.idle_timeout() as u64))
            .max_lifetime(Duration::from_secs(*config.max_lifetime() as u64));

        let db = SeaDatabase::connect(connection_ops)
            .await
            .map_err(|_| StateError)?;
        
        println!("Database operational");
        Ok(db)
    }
}
