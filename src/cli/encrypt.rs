use crate::cli::{crypto_ops, utils};
use crate::config;
use crate::error::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// Encrypt a raw config file
#[derive(Parser, Debug)]
pub struct EncryptArgs {
    /// Path to the raw config file
    #[arg(short, long)]
    pub file: PathBuf,
    /// Output file path (optional)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

pub fn run(args: &EncryptArgs) -> Result<()> {
    let input_path = &args.file;
    let output_path = args
        .output
        .clone()
        .unwrap_or_else(|| utils::get_output_path(input_path, "encrypted", "yaml"));

    let config = config::load_config_from_file(input_path)?;
    let key = std::env::var("ENCRYPTION_KEY")?;
    let encrypted_config = crypto_ops::encrypt_config(config, key.as_bytes())?;

    let yaml = serde_yaml::to_string(&encrypted_config)?;
    fs::write(&output_path, yaml)?;
    println!(
        "Encrypted '{}' to '{}' ✅",
        input_path.display(),
        output_path.display()
    );
    Ok(())
}
