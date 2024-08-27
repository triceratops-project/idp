use std::time::Duration;

use idp_config::DatabaseConfig;
use sea_orm::{ConnectOptions, Database as SeaDatabase, DatabaseConnection};

use crate::error::StateError;

pub struct Database;

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<DatabaseConnection, StateError> {
        let db_uri = config.uri().as_str();
        let mut connection_ops = ConnectOptions::new(db_uri);

        // connection_ops
        //     .max_connections(*config.max_connections())
        //     .min_connections(*config.min_connections())
        //     .connect_timeout(Duration::from_secs(*config.connection_timeout() as u64))
        //     .acquire_timeout(Duration::from_secs(*config.acquire_timeout() as u64))
        //     .idle_timeout(Duration::from_secs(*config.idle_timeout() as u64))
        //     .max_lifetime(Duration::from_secs(*config.max_lifetime() as u64));

        connection_ops
            .max_connections(1000)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(15))
            .max_lifetime(Duration::from_secs(32));

        let db = SeaDatabase::connect(connection_ops)
            .await
            .map_err(|e| StateError::Database(e))?;

        Ok(db)
    }
}
