use thiserror::Error;

use crate::config::ConfigError;
use crate::crypto::CryptoError;
use crate::github::GithubError;

#[derive(Debug, Error)]
pub enum GsmError {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Crypto(#[from] CryptoError),
    #[error(transparent)]
    Github(#[from] GithubError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Env var error: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Base64 error: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("UTF8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, GsmError>;
