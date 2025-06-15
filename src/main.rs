use crate::error::Result;
use clap::Parser;
use colored::Colorize;

mod cli;
mod config;
mod crypto;
mod error;
mod github;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file if present
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("{}: Could not load .env file: {}", "Warning".yellow(), e);
    }

    let cli = cli::Cli::parse();
    match &cli.command {
        cli::Commands::Validate(args) => cli::validate::run(args)?,
        cli::Commands::Encrypt(args) => cli::encrypt::run(args)?,
        cli::Commands::Decrypt(args) => cli::decrypt::run(args)?,
        cli::Commands::EncryptAll(args) => cli::encrypt_all::run(args)?,
        cli::Commands::DecryptAll(args) => cli::decrypt_all::run(args)?,
        cli::Commands::Push(args) => cli::push::run(args).await?,
    }
    Ok(())
}
