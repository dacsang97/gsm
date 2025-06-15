use crate::config;
use crate::error::Result;
use crate::github;
use clap::Parser;

/// Push secrets to GitHub repositories
#[derive(Parser, Debug)]
pub struct PushArgs {
    /// Path to the raw config file
    #[arg(short, long)]
    pub file: String,
}

pub async fn run(args: &PushArgs) -> Result<()> {
    let config = config::load_config_from_file(&args.file)?;
    let token = std::env::var("GITHUB_TOKEN")?;
    let client = reqwest::Client::new();
    for repo in &config.repositories {
        println!("Pushing secrets to repo: {}...", repo);
        let public_key = github::get_repo_public_key(&client, &config.org, repo, &token).await?;
        for (secret_name, value) in &config.env {
            let encrypted = github::encrypt_github_secret(&public_key.key, value)?;
            github::push_repo_secret(
                &client,
                &config.org,
                repo,
                &token,
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
