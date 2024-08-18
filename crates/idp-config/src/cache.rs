use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct CacheConfig {
    uri: Url,
    pool_size: u32,
}

impl CacheConfig {
    pub fn uri(&self) -> &Url {
        &self.uri
    }

    pub fn pool_size(&self) -> &u32 {
        &self.pool_size
    }
}
