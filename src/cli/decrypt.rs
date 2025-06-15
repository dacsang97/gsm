use crate::config::{Config, EncryptedConfig};
use crate::crypto;
use crate::error::Result;
use base64::{Engine as _, engine::general_purpose};
use clap::Parser;
use std::fs;
use std::path::Path;

/// Decrypt an encrypted config file
#[derive(Parser, Debug)]
pub struct DecryptArgs {
    /// Path to the encrypted config file
    #[arg(short, long)]
    pub file: String,
    /// Output file path (optional)
    #[arg(short, long)]
    pub output: Option<String>,
}

pub fn run(args: &DecryptArgs) -> Result<()> {
    let input_path = &args.file;
    let output_path = if let Some(ref out) = args.output {
        out.clone()
    } else {
        let input = Path::new(input_path);
        let stem = input.file_stem().unwrap_or_default().to_string_lossy();
        let ext = input.extension().and_then(|e| e.to_str()).unwrap_or("");
        let mut out_name = format!("{}.decrypted.yaml", stem);
        if ext != "" && ext != "yaml" {
            out_name = format!("{}.decrypted.{}", stem, ext);
        }
        let parent = input.parent().unwrap_or_else(|| Path::new("."));
        parent.join(out_name).to_string_lossy().to_string()
    };
    let content = fs::read_to_string(input_path)?;
    let encrypted_config: EncryptedConfig = serde_yaml::from_str(&content)?;
    let key = std::env::var("ENCRYPTION_KEY")?;
    let mut raw_env = std::collections::HashMap::new();
    for (k, v) in encrypted_config.env.iter() {
        let salt = general_purpose::STANDARD.decode(&v.salt)?;
        let nonce = general_purpose::STANDARD.decode(&v.nonce)?;
        let ciphertext = general_purpose::STANDARD.decode(&v.ciphertext)?;
        let plaintext = crypto::decrypt(&ciphertext, key.as_bytes(), &salt, &nonce)?;
        raw_env.insert(k.clone(), String::from_utf8(plaintext)?);
    }
    let config = Config {
        org: encrypted_config.org,
        repositories: encrypted_config.repositories,
        env: raw_env,
    };
    let yaml = serde_yaml::to_string(&config)?;
    fs::write(&output_path, yaml)?;
    println!("Decrypted '{}' to '{}' ✅", input_path, output_path);
    Ok(())
}
