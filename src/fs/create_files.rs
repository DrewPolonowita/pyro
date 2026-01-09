use std::path::Path;
use std::fs::{create_dir, remove_dir_all, remove_file};
use crate::Result;

pub fn create_directory(path: &Path, filename: String) -> Result<()> {
    let dir_path = path.join(filename);

    create_dir(dir_path)?;

    Ok(())
}

pub fn remove_directory(path: &Path, filename: String, is_recursive: bool) -> Result<()> {
    let dir_path = path.join(filename);
    let metadata = std::fs::metadata(&dir_path)?;

    if metadata.is_dir() {
        if is_recursive {
            std::fs::remove_dir_all(dir_path)?;
        } else {
            std::fs::remove_dir(dir_path)?;
        }

    } else {
        if is_recursive {
            return Err(format!(
                "Flag '--recursive' is not avaliable for files, only directories"
                ).into());
        }

        std::fs::remove_file(dir_path)?;
    }

    Ok(())
}