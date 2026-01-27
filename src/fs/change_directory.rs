use std::path::{Path, PathBuf};
use crate::cli::error::{Result, Error, ErrorType, AppError};
use crate::cli::run_command::flag_error;
use crate::cli::args::{FlagsBuilder, Flags};

pub fn command_cd(argv: &[String], curr_path: &Path, flags: Flags) -> Result<PathBuf> {
    // From a path adds on a value, checks if it is a real directory and returns a simplified path
    //
    // #Args
    // *value A string containing the value to be added to the end of the path
    // *curr_path A path directory representing the current path from the root
    //
    // #Returns
    // Returns a result enum with a PathBuf type. An error is returned when the new directory
    // is not a valid dir or when not one arg is given

    let _ = flag_error("cd", {
        FlagsBuilder::new().build()
    }, &flags)?;

    // Gets the first argumemt, if it doesn't exist return an error
    let Some(value) = argv.get(0) else {
        return Err(AppError::from(ERROR_NO_ARGUMENTS_GIVEN));
    };

    // If more than one argument exists return an error
    if let Some(_) = argv.get(1) {
        return Err(AppError::from(ERROR_TOO_MANY_ARGS));
    };

    // Create new dir path and check its existance
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
    // Removes .. from the file path to normalize with the shortest path inc symlinks

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

/* Errors */

const ERROR_INVALID_PATH: Error = Error {
    error_type: ErrorType::NotFound,
    error_message: "The directory given is invalid"
};

const ERROR_NO_ARGUMENTS_GIVEN: Error = Error {
    error_type: ErrorType::NotFound,
    error_message: "No arguments given for command; 'cd' takes one arguments"
};

const ERROR_TOO_MANY_ARGS: Error = Error {
    error_type: ErrorType::NotFound,
    error_message: "Too many arguments given for command; 'cd' takes one argument"
};