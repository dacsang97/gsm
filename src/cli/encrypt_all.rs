use crate::cli::{encrypt, utils};
use crate::error::Result;
use clap::Parser;
use std::path::PathBuf;

/// Encrypt all raw config files in a folder
#[derive(Parser, Debug)]
pub struct EncryptAllArgs {
    /// Parent input folder containing 'raw' and 'encrypted' subfolders
    #[arg(short, long)]
    pub input: PathBuf,
}

pub fn run(args: &EncryptAllArgs) -> Result<()> {
    utils::process_directory(
        &args.input,
        "raw",
        "encrypted",
        |input_path, output_path| {
            let encrypt_args = encrypt::EncryptArgs {
                file: input_path.to_path_buf(),
                output: Some(output_path.to_path_buf()),
            };
            encrypt::run(&encrypt_args)
        },
    )
}
