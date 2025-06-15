use crate::config::{Config, EncryptedConfig, EncryptedValue};
use crate::crypto;
use crate::error::Result;
use base64::{Engine as _, engine::general_purpose};
use std::collections::HashMap;

/// Encrypt a Config into an EncryptedConfig
pub fn encrypt_config(config: Config, key: &[u8]) -> Result<EncryptedConfig> {
    let mut encrypted_env = HashMap::new();

    for (k, v) in config.env.iter() {
        let (salt, nonce, ciphertext) = crypto::encrypt(v.as_bytes(), key)?;
        encrypted_env.insert(
            k.clone(),
            EncryptedValue {
                salt: general_purpose::STANDARD.encode(&salt),
                nonce: general_purpose::STANDARD.encode(&nonce),
                ciphertext: general_purpose::STANDARD.encode(&ciphertext),
            },
        );
    }

    Ok(EncryptedConfig {
        org: config.org,
        repositories: config.repositories,
        env: encrypted_env,
    })
}

/// Decrypt an EncryptedConfig into a Config
pub fn decrypt_config(encrypted_config: EncryptedConfig, key: &[u8]) -> Result<Config> {
    let mut raw_env = HashMap::new();

    for (k, v) in encrypted_config.env.iter() {
        let salt = general_purpose::STANDARD.decode(&v.salt)?;
        let nonce = general_purpose::STANDARD.decode(&v.nonce)?;
        let ciphertext = general_purpose::STANDARD.decode(&v.ciphertext)?;
        let plaintext = crypto::decrypt(&ciphertext, key, &salt, &nonce)?;
        raw_env.insert(k.clone(), String::from_utf8(plaintext)?);
    }

    Ok(Config {
        org: encrypted_config.org,
        repositories: encrypted_config.repositories,
        env: raw_env,
    })
}
