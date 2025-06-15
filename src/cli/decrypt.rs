use crate::cli::{crypto_ops, utils};
use crate::config::EncryptedConfig;
use crate::error::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// Decrypt an encrypted config file
#[derive(Parser, Debug)]
pub struct DecryptArgs {
    /// Path to the encrypted config file
    #[arg(short, long)]
    pub file: PathBuf,
    /// Output file path (optional)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

pub fn run(args: &DecryptArgs) -> Result<()> {
    let input_path = &args.file;
    let output_path = args
        .output
        .clone()
        .unwrap_or_else(|| utils::get_output_path(input_path, "decrypted", "yaml"));

    let content = fs::read_to_string(input_path)?;
    let encrypted_config: EncryptedConfig = serde_yaml::from_str(&content)?;
    let key = std::env::var("ENCRYPTION_KEY")?;
    let config = crypto_ops::decrypt_config(encrypted_config, key.as_bytes())?;

    let yaml = serde_yaml::to_string(&config)?;
    fs::write(&output_path, yaml)?;
    println!(
        "Decrypted '{}' to '{}' ✅",
        input_path.display(),
        output_path.display()
    );
    Ok(())
}
