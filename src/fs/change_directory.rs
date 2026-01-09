use crate::Result;

use std::path::{Path, PathBuf};
use std::fs::canonicalize;

pub fn command_cd(value: String, curr_path: &Path) -> Result<PathBuf> {
    // From a path adds on a value, checks if it is a real directory and returns a simplified path
    //
    // #Args
    // *value A string containing the value to be added to the end of the path
    // *curr_path A path directory representing the current path from the root
    //
    // #Returns
    // Returns a result enum with a PathBuf type. An error is returned when the new directory
    // is not a valid dir

    let new_path = curr_path.join(value);
    let is_path = &new_path.is_dir();
    let new_path = canonicalize(new_path)?;

    if *is_path {
        Ok(new_path)
    } else {
        Err(format!(
            "File directory does not exist!"
            ).into())
    }
}