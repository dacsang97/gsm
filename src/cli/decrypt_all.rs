use crate::cli::decrypt;
use crate::error::Result;
use clap::Parser;
use std::fs;
use std::path::Path;

/// Decrypt all encrypted config files in a folder
#[derive(Parser, Debug)]
pub struct DecryptAllArgs {
    /// Parent input folder containing 'encrypted' and 'raw' subfolders
    #[arg(short, long)]
    pub input: String,
}

pub fn run(args: &DecryptAllArgs) -> Result<()> {
    let parent_dir = Path::new(&args.input);
    let encrypted_dir = parent_dir.join("encrypted");
    let output_dir = parent_dir.join("raw");
    fs::create_dir_all(&output_dir)?;
    for entry in fs::read_dir(&encrypted_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map(|e| e == "yaml").unwrap_or(false) {
            let file = path.to_string_lossy().to_string();
            let filename = path.file_name().unwrap_or_default().to_string_lossy();
            let out_file = output_dir.join(filename.as_ref());
            let decrypt_args = decrypt::DecryptArgs {
                file,
                output: Some(out_file.to_string_lossy().to_string()),
            };
            decrypt::run(&decrypt_args)?;
        }
    }
    Ok(())
}
