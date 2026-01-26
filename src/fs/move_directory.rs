use crate::cli::error::{AppError, ErrorType, Error, Result, HintError};
use crate::cli::args::Flags;

pub fn move_directory(curr_dir: &std::path::Path, argv: &[String], flags: &Flags) -> Result<()> {
    // Moves a file from a path from one local path to another local path
    //
    // #arguments
    // *curr_dir The current working directory
    // *argv A vector containing the arguments
    // *flags A struct containing the flags
    //
    // #Returns
    // This function only returns errors; this function errors when
    // --force is not used and a file is being replaced
    // all arguments arent supplied in argv
    // file doesn't exist

    let Some(old_local_path) = argv.get(0) else {
        return Err(AppError::from(ERROR_NO_ARGUMENTS_GIVEN));
    };

    let Some(new_local_path) = argv.get(1) else {
        return Err(AppError::from(ERROR_ONE_ARGUMENTS_GIVEN));
    };

    let old_path = curr_dir.join(old_local_path);
    let new_path = curr_dir.join(new_local_path);

    if std::fs::exists(&new_path)? && !flags.force {
        return Err(AppError::from(ERROR_NOT_USING_FORCE));
    }

    std::fs::rename(old_path, new_path)?;

    Ok(())
}

const ERROR_NO_ARGUMENTS_GIVEN: Error = Error {
    error_type: ErrorType::NotFound,
    error_message: "No arguments given for command; 'move' takes two arguments"
};

const ERROR_ONE_ARGUMENTS_GIVEN: Error = Error {
    error_type: ErrorType::NotFound,
    error_message: "One arguments given for command; 'move' takes two arguments"
};

const ERROR_NOT_USING_FORCE: HintError = HintError {
    error_type: ErrorType::InvalidFlag,
    error_message: "The new filepath already contains a file",
    hint_message: "Use --force to override the file"
};