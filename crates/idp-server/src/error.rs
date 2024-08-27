use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("IO Error")]
    Io(#[from] io::Error),
}
