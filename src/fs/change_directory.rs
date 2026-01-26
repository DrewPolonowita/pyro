use std::path::{Path, PathBuf};
use crate::cli::error::{Result, Error, ErrorType, AppError};

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
    let is_dir = &new_path.is_dir();
    let new_path = soft_canonicalize(&new_path);

    if *is_dir {
        return Ok(new_path);
    } else {
        return Err(AppError::from(ERROR_INVALID_PATH));
    }
}

fn soft_canonicalize(path: &Path) -> PathBuf {
    let mut new_path = std::path::PathBuf::new();

    for entry in path {
        if entry == ".." {
            let _ = new_path.pop();
        } else {
            new_path.push(entry);
        }
    }

    new_path
}

const ERROR_INVALID_PATH: Error = Error {
    error_type: ErrorType::NotFound,
    error_message: "The directory given is invalid"
};