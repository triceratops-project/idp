mod cache;
mod database;
mod server;

use std::{error::Error, fmt, fs};

pub use cache::CacheConfig;
pub use database::DatabaseConfig;
use serde::Deserialize;
pub use server::ServerConfig;

const CONFIG_NAME: &'static str = "config.toml";
const CONFIG_VERSION: u8 = 1;

#[derive(Deserialize)]
pub struct IdpConfig {
    version: u8,
    server: ServerConfig,
    database: DatabaseConfig,
    cache: CacheConfig,
}

impl IdpConfig {
    pub fn read() -> Result<Self, ConfigError> {
        let config_string = fs::read_to_string(CONFIG_NAME).map_err(|_| ConfigError)?;

        let config = toml::from_str::<IdpConfig>(&config_string).map_err(|_| ConfigError)?;

        if config.version > CONFIG_VERSION {
            return Err(ConfigError);
        }

        Ok(config)
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn cache(&self) -> &CacheConfig {
        &self.cache
    }
}

#[derive(Debug)]
pub struct ConfigError;

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error loading config")
    }
}

impl Error for ConfigError {}
