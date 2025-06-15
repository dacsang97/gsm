// GitHub API integration module

use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
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

/// GitHub API client for managing repositories and secrets
pub struct GithubClient {
    client: reqwest::Client,
    token: String,
    base_url: String,
}

impl GithubClient {
    /// Create a new GitHub client
    pub fn new(token: String, base_url: Option<String>) -> Self {
        GithubClient {
            client: reqwest::Client::new(),
            token,
            base_url: base_url.unwrap_or_else(|| "https://api.github.com".to_string()),
        }
    }

    /// Get the public key for a repository
    pub async fn get_repo_public_key(&self, org: &str, repo: &str) -> Result<PublicKey> {
        let url = format!(
            "{}/repos/{}/{}/actions/secrets/public-key",
            self.base_url, org, repo
        );
        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", "gsm-cli")
            .send()
            .await?;

        if !resp.status().is_success() {
            return Err(GithubError::HttpError(resp.text().await?));
        }

        let public_key: PublicKey = resp.json().await?;
        Ok(public_key)
    }

    /// Push a secret to a repository
    pub async fn push_repo_secret(
        &self,
        org: &str,
        repo: &str,
        secret_name: &str,
        encrypted_value: &str,
        key_id: &str,
    ) -> Result<()> {
        let url = format!(
            "{}/repos/{}/{}/actions/secrets/{}",
            self.base_url, org, repo, secret_name
        );
        let body = SecretBody {
            encrypted_value,
            key_id,
        };
        let resp = self
            .client
            .put(&url)
            .header("Authorization", format!("token {}", self.token))
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
}

#[derive(Serialize)]
pub struct SecretBody<'a> {
    pub encrypted_value: &'a str,
    pub key_id: &'a str,
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
