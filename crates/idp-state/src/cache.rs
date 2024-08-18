use std::time::Duration;

use fred::{
    prelude::{ClientLike, RedisPool},
    types::{Builder, RedisConfig},
};
use idp_config::CacheConfig;

use crate::StateError;

pub struct Cache;

impl Cache {
    pub async fn new(config: &CacheConfig) -> Result<RedisPool, StateError> {
        let redis_config = RedisConfig::from_url(config.uri().as_str()).map_err(|_| StateError)?;

        let pool = Builder::from_config(redis_config)
            .with_connection_config(|config| config.connection_timeout = Duration::from_secs(8))
            .build_pool(*config.pool_size() as usize)
            .map_err(|_| StateError)?;

        pool.init().await.map_err(|_| StateError)?;
        println!("Cache operational");
        Ok(pool)
    }
}
