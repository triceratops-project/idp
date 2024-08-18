mod cache;
mod database;

use std::{error::Error, fmt};

use cache::Cache;
use database::Database;
use fred::prelude::RedisPool;
use idp_config::IdpConfig;
use sea_orm::DatabaseConnection;

pub struct IdpState {
    db: DatabaseConnection,
    cache: RedisPool,
}

impl IdpState {
    pub async fn new(config: &IdpConfig) -> Result<Self, StateError> {
        let db = Database::new(config.database()).await?;
        let cache = Cache::new(config.cache()).await?;

        Ok(Self { db, cache })
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub fn cache(&self) -> &RedisPool {
        &self.cache
    }
}

#[derive(Debug)]
pub struct StateError;

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error building state")
    }
}

impl Error for StateError {}
