use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("configuration file does not exist: {0}")]
    MissingFile(PathBuf),
    #[error("configuration field must not be empty: {0}")]
    EmptyField(&'static str),
    #[error("configuration field must be greater than zero: {0}")]
    ZeroValue(&'static str),
}
