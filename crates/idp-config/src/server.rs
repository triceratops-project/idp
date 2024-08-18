use std::net::IpAddr;

use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct ServerConfig {
    url: Url,
    bind_address: IpAddr,
    port: u16,
}

impl ServerConfig {
    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn bind_address(&self) -> &IpAddr {
        &self.bind_address
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }
}
