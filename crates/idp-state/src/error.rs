use fred::error::RedisError;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StateError {
    #[error("Error initialising database connection")]
    Database(#[from] DbErr),
    #[error("Error initialising cache connection")]
    Cache(#[from] RedisError),
}
