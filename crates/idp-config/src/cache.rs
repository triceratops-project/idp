use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct CacheConfig {
    uri: Url,
}

impl CacheConfig {
    pub fn uri(&self) -> &Url {
        &self.uri
    }
}
