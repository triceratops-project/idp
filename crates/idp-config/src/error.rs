use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO Error")]
    Io(#[from] io::Error),
    #[error("Deserialization Error")]
    Deserialization(#[from] toml::de::Error),
    #[error("Config file version mismatch")]
    VersionMismatch,
}
