use crate::cli::{decrypt, utils};
use crate::error::Result;
use clap::Parser;
use std::path::PathBuf;

/// Decrypt all encrypted config files in a folder
#[derive(Parser, Debug)]
pub struct DecryptAllArgs {
    /// Parent input folder containing 'encrypted' and 'raw' subfolders
    #[arg(short, long)]
    pub input: PathBuf,
}

pub fn run(args: &DecryptAllArgs) -> Result<()> {
    utils::process_directory(
        &args.input,
        "encrypted",
        "raw",
        |input_path, output_path| {
            let decrypt_args = decrypt::DecryptArgs {
                file: input_path.to_path_buf(),
                output: Some(output_path.to_path_buf()),
            };
            decrypt::run(&decrypt_args)
        },
    )
}
