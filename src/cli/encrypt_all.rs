use crate::cli::encrypt;
use crate::error::Result;
use clap::Parser;
use std::fs;
use std::path::Path;

/// Encrypt all raw config files in a folder
#[derive(Parser, Debug)]
pub struct EncryptAllArgs {
    /// Parent input folder containing 'raw' and 'encrypted' subfolders
    #[arg(short, long)]
    pub input: String,
}

pub fn run(args: &EncryptAllArgs) -> Result<()> {
    let parent_dir = Path::new(&args.input);
    let raw_dir = parent_dir.join("raw");
    let output_dir = parent_dir.join("encrypted");
    fs::create_dir_all(&output_dir)?;
    for entry in fs::read_dir(&raw_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map(|e| e == "yaml").unwrap_or(false) {
            let file = path.to_string_lossy().to_string();
            let filename = path.file_name().unwrap_or_default().to_string_lossy();
            let out_file = output_dir.join(filename.as_ref());
            let encrypt_args = encrypt::EncryptArgs {
                file,
                output: Some(out_file.to_string_lossy().to_string()),
            };
            encrypt::run(&encrypt_args)?;
        }
    }
    Ok(())
}
