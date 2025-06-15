// Configuration management module

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("File read error: {0}")]
    FileReadError(#[from] std::io::Error),
    #[error("YAML parse error: {0}")]
    YamlParseError(#[from] serde_yaml::Error),
}

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub org: String,
    pub repositories: Vec<String>,
    pub env: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedValue {
    pub salt: String,
    pub nonce: String,
    pub ciphertext: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedConfig {
    pub org: String,
    pub repositories: Vec<String>,
    pub env: HashMap<String, EncryptedValue>,
}

pub fn load_config_from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
