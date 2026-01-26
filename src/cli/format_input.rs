use crate::Result;
use crate::cli::args::{FlagsBuilder, Command, Args, ArgRunStatus};
use crate::cli::error::{AppError, ErrorType, Error, HintError};

pub fn format_input(user_input: &str) -> Result<Args>{
    // Takes a user input as a string and returns a the tokenized and characterized
    // version of the user input for the script to read
    //
    // #Args
    // *user_input A string reference containing the user input of the cmd line to tokenize
    //
    // #Returns
    // Returns a result enum with an error message to be printed or the users arguments
    //

    // removes whitespace and newline characters
    let user_input = user_input.trim();

    // error when the input is nothing
    if user_input.is_empty() {
        return Err(AppError::from(ERROR_NO_INPUT));
    }

    let mut tokens = split(user_input);

    let mut flag_builder = FlagsBuilder::new();

    let mut main_command: Option<Command> = None;
    let mut argv = Vec::new();

    while let Some(token) = tokens.next() {
        let token = token?;

        if let Some(command) = Command::from(token.as_str()) {
            if !matches!(main_command, None) {
                return Err(AppError::from(ERROR_HAS_TWO_COMMANDS));
            }
            main_command = Some(command);

        } else if !flag_builder.parse(token.as_str())? {
            argv.push(token);
        }
    }

    Ok(Args {
        command: main_command,
        argv: argv,
        flags: flag_builder.build()
    })
}

fn split(s: &str) -> impl Iterator<Item = Result<String>> {
    // Splits a token
    let mut chars = s.chars();
    let mut current_token = String::new();

    std::iter::from_fn(move || {

        while let Some(char) = chars.next() {
            if char == '"' || char == '\'' {
                while let Some(next_char) = chars.next() {
                    if next_char == '"' || next_char == '\'' {
                        break
                    }

                    current_token.push(next_char)
                }

            } else if char == ' ' {

                if !current_token.is_empty() {
                    let token = std::mem::take(&mut current_token);
                    return Some(Ok(token));
                }

            } else if char == '\\' {
                let Some(next_char) = chars.next() else {
                    return Some(Err(AppError::from(ERROR_STRING_ENDS_IN_BACKSLASH)));
                };

                current_token.push(next_char)
            } else {
                current_token.push(char)
            }
        }

        if !current_token.is_empty() {
            let token = std::mem::take(&mut current_token);
            return Some(Ok(token));
        }

        None
    })
}

pub fn parse_pipe_args(s: &mut String) -> impl Iterator<Item = ArgRunStatus> {
    //
    //

    let mut chars = s.chars();
    let mut current_token = String::new();
    let mut next_is_conditional = false;

    std::iter::from_fn(move || {
        while let Some(char) = chars.next() {
            if char == '"' || char == '\'' {

                current_token.push(char);

                while let Some(next_char) = chars.next() {

                    current_token.push(next_char);

                    if next_char == '"' || next_char == '\'' {
                        break
                    }
                }

            } else if char == '\\' {

                current_token.push(char);
                if let Some(next_char) = chars.next() {
                    current_token.push(next_char);
                }

            } else if char == '&' {

                let Some(next_char) = chars.next() else {
                    current_token.push(char);
                    continue
                };

                if next_char != '&' {
                    current_token.push(char);
                    current_token.push(next_char);
                    continue
                }

                if !current_token.is_empty() {
                    let token = std::mem::take(&mut current_token);
                    if next_is_conditional {
                        next_is_conditional = true;
                        return Some(ArgRunStatus::Conditional(token))
                    } else {
                        next_is_conditional = true;
                        return Some(ArgRunStatus::UnConditional(token))
                    }
                }
            } else if char == ';' {

                if !current_token.is_empty() {
                    let token = std::mem::take(&mut current_token);
                    if next_is_conditional {
                        next_is_conditional = false;
                        return Some(ArgRunStatus::Conditional(token))
                    } else {
                        next_is_conditional = false;
                        return Some(ArgRunStatus::UnConditional(token))
                    }
                }

            } else if char == '|' {

                if !current_token.is_empty() {
                    let token = std::mem::take(&mut current_token);
                    
                    next_is_conditional = true;
                    return Some(ArgRunStatus::Pipeline(token))
                }

            } else {
                current_token.push(char);
            }
        }

        if !current_token.is_empty() {
            let token = std::mem::take(&mut current_token);
            if next_is_conditional {
                next_is_conditional = true;
                return Some(ArgRunStatus::Conditional(token))
            } else {
                next_is_conditional = true;
                return Some(ArgRunStatus::UnConditional(token))
            }
        }

        None
    })
}

const ERROR_STRING_ENDS_IN_BACKSLASH: Error = Error {
    error_type: ErrorType::InvalidToken,
    error_message: "Cannot end with a backslash as backslash is the escape character"
};

const ERROR_NO_INPUT: Error = Error {
    error_type: ErrorType::EmptyString,
    error_message: "Input field must not be empty"
};

const ERROR_HAS_TWO_COMMANDS: HintError = HintError {
    error_type: ErrorType::AlreadyExists,
    error_message: "Cannot have two commands in one argument",
    hint_message: "Consider seperating commands with '&&' or ';'. Use --help for more details"
};