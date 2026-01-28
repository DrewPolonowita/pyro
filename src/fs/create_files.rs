use crate::cli::error::{Result, AppError};
use std::path::Path;
use crate::cli::error::{HintError, ErrorType, Error};

use crate::cli::run_command::flag_error;
use crate::cli::args::{FlagsBuilder, Flags};

pub fn create_from_path(curr_dir: &Path, argv: &[String], flags: Flags) -> Result<()> {
    // Creates a file or directory at the given location with a name
    //
    // #arguments
    // *curr_dir: The current path of the CLI
    // *argv: A vector of provided arguments, this vector must be of length one
    // *flags: A Flags struct containing all the user flag options
    //
    // #returns
    // This function only returns errors

    // Returns an error when the user inputs a not supported flag
    let _ = flag_error("new", {
        FlagsBuilder::new()
        .recursive()
        .force()
        .dir()
        .build()
    }, &flags)?;

    // Errors if only one argument is present
    let Some(local_path) = argv.get(0) else {
        return Err(AppError::from(ERROR_NO_ARGUMENTS_FOR_NEW));
    };

    // Errors if more than one argument is present
    if let Some(_) = argv.get(1) {
        return Err(AppError::from(ERROR_TOO_MANY_ARGUMENTS_FOR_NEW));
    }

    //let path = path.join(local_path);
    let path = &curr_dir.join(&local_path);
    let parent_path = path.parent();

    if flags.dir {

        // Checking to see if the parent path exists
        match parent_path {

            // Parent path is not the root
            Some(parent_path) => {
                match std::fs::symlink_metadata(&parent_path) {
                    // We have access to the parent directory
                    Ok(meta) => {
                        // If the parent is a file then we error (do not replace files with folders)
                        if !meta.is_dir() {
                            return Err(AppError::from(ERROR_FILE_EXISTS_ON_PATH));
                        }

                        // The dir exists and is not a folder --force wont replace it
                        // The dir exists and is a folder --force will replace it
                        if let Ok(meta) = std::fs::symlink_metadata(&path) {
                            if !meta.is_dir() {
                                return Err(AppError::from(ERROR_FILE_EXISTS_ON_PATH));
                            } else if !flags.force {
                                return Err(AppError::from(ERROR_FOLDER_PATH_ALREADY_EXISTS));
                            }
                        }

                        std::fs::create_dir(path)?;

                    },

                    // We do not have access to the parent directory or it doesnt exist
                    Err(_) => {
                        // Folder only created if the recursive tag is used
                        if !flags.recursive {
                            return Err(AppError::from(ERROR_NOT_USING_RECURSIVE));
                        }

                        // Create the directories and the folder
                        std::fs::create_dir_all(path)?;
                    }
                }
            },

            // The parent is the root
            None => {
                std::fs::create_dir(path)?;
            }
        }

    } else {
        // Checking to see if the parent path exists
        match parent_path {
            // Parent path

            Some(parent_path) => {

                match std::fs::symlink_metadata(&parent_path) {

                    // We have access to the parent path and it exists
                    Ok(meta) => {

                        // If the parent is a file then we error (do not replace files with folders)
                        if !meta.is_dir() {
                            return Err(AppError::from(ERROR_FILE_EXISTS_ON_PATH));
                        }

                        // The dir exists and is not a file --force wont replace it
                        // The dir exists and is a file --force will replace it
                        if let Ok(meta) = std::fs::symlink_metadata(&path) {
                            if meta.is_dir() {
                                return Err(AppError::from(ERROR_FOLDER_EXISTS_ON_PATH));
                            } else if !flags.force {
                                return Err(AppError::from(ERROR_FILE_PATH_ALREADY_EXISTS));
                            }
                        }

                        std::fs::File::create(path)?;
                    },

                    // The parent path doesnt exist or we dont have access
                    Err(_) => {

                        // Files only created if the recursive tag is used
                        if !flags.recursive {
                            return Err(AppError::from(ERROR_NOT_USING_RECURSIVE));
                        }
                        if let Some(_) = find_blocking_component(curr_dir, &std::path::PathBuf::from(local_path))? {
                            return Err(AppError::from(ERROR_FILE_EXISTS_ON_PATH));
                        }

                        // Create the directories and the file
                        std::fs::create_dir_all(parent_path)?;
                        std::fs::File::create(path)?;
                    }
                }
            },

            // If the parent is the root
            None => {
                std::fs::File::create(path)?;
            }
        }
    }


    Ok(())
}

pub fn remove_directory(path: &Path, argv: &[String], flags: Flags) -> Result<()> {
    // Deletes a file in a given directory with the filename in argv[0].
    //
    // #arguments
    // *path: A reference to the current working directory
    // *argv: A vector containing the arguments provided by the user
    // *flags: A Flag struct instance containing the user flags
    //
    // #returns
    // Returns an empty result enum and errors when invalid flags are used, non file permissions, invalid number of args etc.

    // Returns an error if invalid flags are provided
    let _ = flag_error("del", {
        FlagsBuilder::new()
        .recursive()
        .build()
    }, &flags)?;

    // Returns an error if there are no args provided
    let Some(filename) = argv.get(0) else {
        return Err(AppError::from(ERROR_NO_ARGUMENTS_FOR_DEL));
    };

    // Return an error if there are too many args provided
    if let Some(_) = argv.get(1) {
        return Err(AppError::from(ERROR_TOO_MANY_ARGUMENTS_FOR_DEL));
    }

    let path = path.join(filename);
    let metadata = std::fs::metadata(&path)?;

    // file removeing is a directory
    if metadata.is_dir() {

        // remove all items in the dir since recursive is a flag
        if flags.recursive {
            std::fs::remove_dir_all(path)?;
        } else {

            // checks how many children are in the dir
            match std::fs::read_dir(&path) {
                Ok(dir_items) => {

                    // dir is not empty, must use recursive flag
                    if dir_items.count() > 0 {
                        return Err(AppError::from(ERROR_NOT_USING_DEL_RECURSIVE));
                    }
                },

                // operation failed, likely permissions error
                Err(e) => return Err(e.into())
            }

            // removes the filke
            std::fs::remove_dir(path)?;
        }

    // file removing is a file
    } else {

        // recursive errors on a file since its only for directories
        if flags.recursive {
            return Err(AppError::from(ERROR_USING_RECURSIVE_ON_A_FILE));
        }

        // removes the path
        std::fs::remove_file(path)?;
    }

    Ok(())
}

fn find_blocking_component(current_dir: &Path, path: &Path) -> Result<Option<std::path::PathBuf>> {
    // This function finds if a file is on the given directory and returns its file path else returns a None
    // This function errors if the path is non existant.

    let mut current_dir = std::path::PathBuf::from(current_dir);

    for comp in path.components() {
        current_dir.push(comp);

        match std::fs::metadata(&current_dir) {
            Ok(meta) => {
                if meta.is_file() {
                    return Ok(Some(current_dir))
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(None);
            },
            Err(e) => return Err(e.into())
        }
    }

    Ok(None)
}

const ERROR_FILE_PATH_ALREADY_EXISTS: HintError = HintError {
    error_type: ErrorType::AlreadyExists,
    error_message: "A file at the specified path already exists",
    hint_message: "Try using --force to overwrite this file"
};

const ERROR_FOLDER_PATH_ALREADY_EXISTS: HintError = HintError {
    error_type: ErrorType::AlreadyExists,
    error_message: "A directory at the specified path already exists",
    hint_message: "Try using --force to overwrite this file"
};

const ERROR_FILE_EXISTS_ON_PATH: HintError = HintError {
    error_type: ErrorType::AlreadyExists,
    error_message: "Cannot create directory because a 'file' already exists on this path",
    hint_message: "Try using the 'del' keyword to remove the conflicting file"
};

const ERROR_FOLDER_EXISTS_ON_PATH: HintError = HintError {
    error_type: ErrorType::AlreadyExists,
    error_message: "Cannot create file because a 'folder' already exists on this path",
    hint_message: "Try using the 'del' keyword to remove the conflicting folder"
};

const ERROR_NOT_USING_RECURSIVE: HintError = HintError {
    error_type: ErrorType::NotFound,
    error_message: "The parent directory of the specified path doesn't exist and 'new' does not create parent directories by default",
    hint_message: "Try using --recursive to create the required parent directories"
};

const ERROR_NOT_USING_DEL_RECURSIVE: HintError = HintError {
    error_type: ErrorType::NotEmpty,
    error_message: "The folder at the specified path is not empty; 'del' will not remove this folder by default",
    hint_message: "Try using --recursive to recursivly delete all sub-files and folders"
};

const ERROR_USING_RECURSIVE_ON_A_FILE: HintError = HintError {
    error_type: ErrorType::InvalidFlag,
    error_message: "Flag '--recursive' is not avaliable for files, only directories",
    hint_message: "Try removing '--recursive' from the querry"
};

const ERROR_NO_ARGUMENTS_FOR_DEL: Error = Error {
    error_type: ErrorType::InvalidToken,
    error_message: "Missing argument for filename; 'del' requires one argument",
};

const ERROR_TOO_MANY_ARGUMENTS_FOR_DEL: Error = Error {
    error_type: ErrorType::InvalidToken,
    error_message: "Too many arguments supplied for command; 'del' only takes one argument",
};

const ERROR_NO_ARGUMENTS_FOR_NEW: Error = Error {
    error_type: ErrorType::InvalidToken,
    error_message: "Missing argument for filename; 'new' requires one argument",
};

const ERROR_TOO_MANY_ARGUMENTS_FOR_NEW: Error = Error {
    error_type: ErrorType::InvalidToken,
    error_message: "Too many arguments supplied for command; 'new' only takes one argument",
};