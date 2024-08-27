use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
// #[serde(default)]
pub struct DatabaseConfig {
    uri: Url,
    // max_connections: u32,
    // min_connections: u32,
    // connection_timeout: u32,
    // acquire_timeout: u32,
    // idle_timeout: u32,
    // max_lifetime: u32,
}

impl DatabaseConfig {
    pub fn uri(&self) -> &Url {
        &self.uri
    }
    // pub fn max_connections(&self) -> &u32 {
    //     &self.max_connections
    // }
    // pub fn min_connections(&self) -> &u32 {
    //     &self.min_connections
    // }
    // pub fn connection_timeout(&self) -> &u32 {
    //     &self.connection_timeout
    // }
    // pub fn acquire_timeout(&self) -> &u32 {
    //     &self.acquire_timeout
    // }

    // pub fn idle_timeout(&self) -> &u32 {
    //     &self.idle_timeout
    // }

    // pub fn max_lifetime(&self) -> &u32 {
    //     &self.max_lifetime
    // }
}

// impl Default for DatabaseConfig {
//     fn default() -> Self {
//         Self {
//             uri: Url::parse("postgres://username:password@127.0.0.1:5432/idp").unwrap(),
//             max_connections: 1000,
//             min_connections: 5,
//             connection_timeout: 8,
//             acquire_timeout: 8,
//             idle_timeout: 15,
//             max_lifetime: 32,
//         }
//     }
// }
