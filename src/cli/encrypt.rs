use crate::config::{EncryptedConfig, EncryptedValue};
use crate::crypto;
use crate::error::Result;
use base64::{Engine as _, engine::general_purpose};
use clap::Parser;
use std::fs;
use std::path::Path;

/// Encrypt a raw config file
#[derive(Parser, Debug)]
pub struct EncryptArgs {
    /// Path to the raw config file
    #[arg(short, long)]
    pub file: String,
    /// Output file path (optional)
    #[arg(short, long)]
    pub output: Option<String>,
}

pub fn run(args: &EncryptArgs) -> Result<()> {
    let input_path = &args.file;
    let output_path = if let Some(ref out) = args.output {
        out.clone()
    } else {
        let input = Path::new(input_path);
        let stem = input.file_stem().unwrap_or_default().to_string_lossy();
        let ext = input.extension().and_then(|e| e.to_str()).unwrap_or("");
        let mut out_name = format!("{}.encrypted.yaml", stem);
        if ext != "" && ext != "yaml" {
            out_name = format!("{}.encrypted.{}", stem, ext);
        }
        let parent = input.parent().unwrap_or_else(|| Path::new("."));
        parent.join(out_name).to_string_lossy().to_string()
    };
    let config = crate::config::load_config_from_file(input_path)?;
    let key = std::env::var("ENCRYPTION_KEY")?;
    let mut encrypted_env = std::collections::HashMap::new();
    for (k, v) in config.env.iter() {
        let (salt, nonce, ciphertext) = crypto::encrypt(v.as_bytes(), key.as_bytes())?;
        encrypted_env.insert(
            k.clone(),
            EncryptedValue {
                salt: general_purpose::STANDARD.encode(&salt),
                nonce: general_purpose::STANDARD.encode(&nonce),
                ciphertext: general_purpose::STANDARD.encode(&ciphertext),
            },
        );
    }
    let encrypted_config = EncryptedConfig {
        org: config.org,
        repositories: config.repositories,
        env: encrypted_env,
    };
    let yaml = serde_yaml::to_string(&encrypted_config)?;
    fs::write(&output_path, yaml)?;
    println!("Encrypted '{}' to '{}' ✅", input_path, output_path);
    Ok(())
}
