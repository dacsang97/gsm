use crate::config;
use crate::error::Result;
use clap::Parser;

/// Validate a configuration file
#[derive(Parser, Debug)]
pub struct ValidateArgs {
    /// Path to the config file
    #[arg(short, long, default_value = "examples/production.yaml")]
    pub file: String,
}

pub fn run(args: &ValidateArgs) -> Result<()> {
    match config::load_config_from_file(&args.file) {
        Ok(_) => {
            println!("Config file '{}' is valid ✅", args.file);
            Ok(())
        }
        Err(e) => {
            eprintln!("Config file '{}' is invalid: {}", args.file, e);
            Err(e.into())
        }
    }
}
