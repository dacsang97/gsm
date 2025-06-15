use crate::config;
use crate::error::Result;
use crate::github::{GithubClient, encrypt_github_secret};
use clap::Parser;
use std::path::PathBuf;

/// Push secrets to GitHub repositories
#[derive(Parser, Debug)]
pub struct PushArgs {
    /// Path to the raw config file
    #[arg(short, long)]
    pub file: PathBuf,
    /// GitHub API base URL (for GitHub Enterprise)
    #[arg(long, default_value = "https://api.github.com")]
    pub api_url: String,
}

pub async fn run(args: &PushArgs) -> Result<()> {
    let config = config::load_config_from_file(&args.file)?;
    let token = std::env::var("GITHUB_TOKEN")?;

    // Create GitHub client with custom API URL support
    let api_url = if args.api_url == "https://api.github.com" {
        None
    } else {
        Some(args.api_url.clone())
    };
    let github_client = GithubClient::new(token, api_url);

    for repo in &config.repositories {
        println!("Pushing secrets to repo: {}...", repo);
        let public_key = github_client.get_repo_public_key(&config.org, repo).await?;
        for (secret_name, value) in &config.env {
            let encrypted = encrypt_github_secret(&public_key.key, value)?;
            github_client
                .push_repo_secret(
                    &config.org,
                    repo,
                    secret_name,
                    &encrypted,
                    &public_key.key_id,
                )
                .await?;
            println!("  - {}: pushed", secret_name);
        }
    }
    println!("All secrets pushed successfully!");
    Ok(())
}
