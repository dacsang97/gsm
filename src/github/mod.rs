// GitHub API integration module

use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GithubError {
    #[error("HTTP error: {0}")]
    HttpError(String),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Base64 decode error: {0}")]
    Base64Error(#[from] base64::DecodeError),
    #[error("Sodiumoxide error")]
    SodiumInitError,
    #[error("Invalid public key length")]
    InvalidPublicKeyLength,
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, GithubError>;

#[derive(Debug, Deserialize)]
pub struct PublicKey {
    pub key: String,
    pub key_id: String,
}

#[derive(Serialize)]
pub struct SecretBody<'a> {
    pub encrypted_value: &'a str,
    pub key_id: &'a str,
}

pub async fn get_repo_public_key(
    client: &reqwest::Client,
    org: &str,
    repo: &str,
    token: &str,
) -> Result<PublicKey> {
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/actions/secrets/public-key",
        owner = org,
        repo = repo
    );
    let resp = client
        .get(&url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "gsm-cli")
        .send()
        .await?;
    if !resp.status().is_success() {
        return Err(GithubError::HttpError(resp.text().await?));
    }
    let text = resp.text().await?;
    let value: Value = serde_json::from_str(&text)?;
    Ok(PublicKey {
        key: value["key"].as_str().unwrap().to_string(),
        key_id: value["key_id"].as_str().unwrap().to_string(),
    })
}

pub async fn push_repo_secret(
    client: &reqwest::Client,
    org: &str,
    repo: &str,
    token: &str,
    secret_name: &str,
    encrypted_value: &str,
    key_id: &str,
) -> Result<()> {
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}/actions/secrets/{secret}",
        owner = org,
        repo = repo,
        secret = secret_name
    );
    let body = SecretBody {
        encrypted_value,
        key_id,
    };
    let resp = client
        .put(&url)
        .header("Authorization", format!("token {}", token))
        .header("User-Agent", "gsm-cli")
        .json(&body)
        .send()
        .await?;
    if resp.status().is_success() {
        Ok(())
    } else {
        Err(GithubError::HttpError(resp.text().await?))
    }
}

pub fn encrypt_github_secret(public_key_b64: &str, secret: &str) -> Result<String> {
    use sodiumoxide::crypto::box_::PublicKey as SodiumPublicKey;
    use sodiumoxide::crypto::sealedbox;
    if sodiumoxide::init().is_err() {
        return Err(GithubError::SodiumInitError);
    }
    let public_key_bytes = general_purpose::STANDARD.decode(public_key_b64)?;
    let public_key = SodiumPublicKey::from_slice(&public_key_bytes)
        .ok_or(GithubError::InvalidPublicKeyLength)?;
    let encrypted = sealedbox::seal(secret.as_bytes(), &public_key);
    Ok(general_purpose::STANDARD.encode(&encrypted))
}
