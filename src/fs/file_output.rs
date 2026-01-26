use crate::cli::error::{AppError, Error, ErrorType, Result};
//use crate::cli::args::Flags;

pub fn command_cat(argv: &[String], stdin: &String, curr_path: &std::path::Path) -> Result<String> {
    if argv.is_empty() {
        if stdin.is_empty() {
            return Err(AppError::from(ERROR_NO_ARGS_FOR_CAT));
        }
        Ok(String::from(stdin))
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

pub fn command_echo(argv: &[String]) -> Result<String> {

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

pub fn command_pwd(curr_path: &std::path::Path, argv: &[String]) -> Result<String> {
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