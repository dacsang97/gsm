use crate::error::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Generate output path based on input path and suffix
pub fn get_output_path(input_path: &Path, suffix: &str, new_ext: &str) -> PathBuf {
    let stem = input_path.file_stem().unwrap_or_default().to_string_lossy();
    let parent = input_path.parent().unwrap_or_else(|| Path::new("."));
    let out_name = format!("{}.{}.{}", stem, suffix, new_ext);
    parent.join(out_name)
}

/// Process all .yaml files in a directory with a given processor function
pub fn process_directory<F>(
    parent_dir: &Path,
    input_subdir: &str,
    output_subdir: &str,
    processor: F,
) -> Result<()>
where
    F: Fn(&Path, &Path) -> Result<()>,
{
    let input_dir = parent_dir.join(input_subdir);
    let output_dir = parent_dir.join(output_subdir);

    fs::create_dir_all(&output_dir)?;

    for entry in fs::read_dir(&input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map(|e| e == "yaml").unwrap_or(false) {
            let filename = path.file_name().unwrap_or_default();
            let output_path = output_dir.join(filename);
            processor(&path, &output_path)?;
        }
    }

    Ok(())
}
