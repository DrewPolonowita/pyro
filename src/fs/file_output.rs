use crate::cli::error::{AppError, Error, ErrorType, Result};

use crate::cli::run_command::flag_error;
use crate::cli::args::{FlagsBuilder, Flags};

pub fn command_cat(argv: &[String], stdin: &String, curr_path: &std::path::Path, flags: Flags) -> Result<String> {
    // Returns the file contents of the files in argv to the console. If no args are provided reads from stdin and returns it
    //
    // #arguments
    // *argv: A vector containing the user arguments
    // *stdin: A string reference containing the stdin value
    // *curr_path: A path reference of the current working directory
    // *flags: A Flags struct containing the user flags
    //
    // #returns
    // A result enum with the text to print to the console for stdout

    // Return an error if there are invalid flags
    let _ = flag_error("cat", {
        FlagsBuilder::new().build()
    }, &flags)?;

    if argv.is_empty() {
        if stdin.is_empty() {
            return Err(AppError::from(ERROR_NO_ARGS_FOR_CAT));
        }
        Ok(String::from(stdin.to_uppercase()))
    } else {
        let mut output = String::new();

        for filename in argv {
            let path = curr_path.join(filename);
            if output.is_empty() {
                output = std::fs::read_to_string(path)?
            } else {
                output = format!("{}\n{}", output, &std::fs::read_to_string(path)?);
            }
        }

        Ok(output)
    }
}

pub fn command_echo(argv: &[String], flags: Flags) -> Result<String> {
    // Returns the all user arguments seperated by a space to the console
    //
    // #arguments
    // *argv: A vector containing the user arguments
    // *flags: A Flags struct containing the user flags
    //
    // #returns
    // A result enum with the text to print to the console for stdout

    // Return an error if there are invalid flags
    let _ = flag_error("echo", {
        FlagsBuilder::new().build()
    }, &flags)?;

    // If no arguments are provided return an error
    if argv.is_empty() {
        return Err(AppError::from(ERROR_NO_ARGS_FOR_ECHO));
    }

    let mut output = String::new();

    for arg in argv {
        if output.is_empty() {
            output = arg.to_string()
        } else {
            output = format!("{} {}", output, arg);
        }
    }

    Ok(output)
}

pub fn command_pwd(curr_path: &std::path::Path, argv: &[String], flags: Flags) -> Result<String> {
    // Returns the current working directory
    //
    // #arguments
    // *argv: A vector containing the user arguments
    // *curr_path: A path reference of the current working directory
    //
    // #Returns
    // A result enum with a String of the cwd

    // Return an error if there are invalid flags
    let _ = flag_error("pwd", {
        FlagsBuilder::new().build()
    }, &flags)?;

    if !argv.is_empty() {
        return Err(AppError::from(ERROR_TO_MANY_ARGS_FOR_PWD));
    }

    Ok(format!("\nCurrent working directory: {}\n", curr_path.display()))
}

const ERROR_NO_ARGS_FOR_ECHO: Error = Error {
    error_type: ErrorType::NotFound,
    error_message: "No arguments given for echo command"
};

const ERROR_NO_ARGS_FOR_CAT: Error = Error {
    error_type: ErrorType::NotFound,
    error_message: "No arguments given for cat command"
};

const ERROR_TO_MANY_ARGS_FOR_PWD: Error = Error {
    error_type: ErrorType::InvalidToken,
    error_message: "Too many arguments given; 'cwd' takes no arguments"
};