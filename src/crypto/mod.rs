// Encryption/decryption engine module

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use sha2::Sha256;
use thiserror::Error;

const PBKDF2_ITER: u32 = 100_000;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Key error: {0:?}")]
    KeyError(String),
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
}

pub type Result<T> = std::result::Result<T, CryptoError>;

pub fn derive_key(password: &[u8], salt: &[u8]) -> [u8; KEY_LEN] {
    let mut key = [0u8; KEY_LEN];
    pbkdf2_hmac::<Sha256>(password, salt, PBKDF2_ITER, &mut key);
    key
}

pub fn encrypt(plaintext: &[u8], password: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let mut salt = [0u8; SALT_LEN];
    rand::rng().fill_bytes(&mut salt);
    let key = derive_key(password, &salt);
    let cipher =
        Aes256Gcm::new_from_slice(&key).map_err(|e| CryptoError::KeyError(format!("{:?}", e)))?;
    let mut nonce = [0u8; NONCE_LEN];
    rand::rng().fill_bytes(&mut nonce);
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;
    Ok((salt.to_vec(), nonce.to_vec(), ciphertext))
}

pub fn decrypt(ciphertext: &[u8], password: &[u8], salt: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
    let key = derive_key(password, salt);
    let cipher =
        Aes256Gcm::new_from_slice(&key).map_err(|e| CryptoError::KeyError(format!("{:?}", e)))?;
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;
    Ok(plaintext)
}
