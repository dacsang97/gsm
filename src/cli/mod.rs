// CLI module (command-line interface)

pub mod decrypt;
pub mod decrypt_all;
pub mod encrypt;
pub mod encrypt_all;
pub mod push;
pub mod validate;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "gsm")]
#[command(about = "GitHub Secrets Manager CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Validate a configuration file
    Validate(validate::ValidateArgs),
    /// Encrypt a raw config file
    Encrypt(encrypt::EncryptArgs),
    /// Decrypt an encrypted config file
    Decrypt(decrypt::DecryptArgs),
    /// Encrypt all raw config files
    EncryptAll(encrypt_all::EncryptAllArgs),
    /// Decrypt all encrypted config files
    DecryptAll(decrypt_all::DecryptAllArgs),
    /// Push secrets to GitHub repositories
    Push(push::PushArgs),
}
