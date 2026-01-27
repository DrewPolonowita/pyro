use crate::cli::args::{Command, Args, Flags};
use crate::cli::help::{command_help, flag_help};

use crate::fs::create_files::{create_from_path, remove_directory};
use crate::fs::change_directory::command_cd;
use crate::fs::file_output::{command_echo, command_cat, command_pwd};
use crate::fs::move_directory::move_directory;

use std::path::PathBuf;
use crate::cli::error::{AppError, Error, ErrorType};

use crate::fs::display_dir::command_ls;
use crate::Result;
use crate::cli::StdStream;

pub fn run_command(args: Args, curr_path: &mut PathBuf, std_stream: &mut StdStream) -> Result<()> {
    // Runs the required command with the flag arguments for a given command
    //
    // #arguments
    // *args An Args enum containing the command and the given flags for the querry
    // *curr_path The current working directory path of the file explorer
    //
    // #returns
    // This function returns an empty Result type, this function can return errors but the Ok value is discarded

    let flags = args.flags;

    if flags.help {
        std_stream.write(match args.command {
            Some(command) => flag_help(command),
            None => command_help()
        });
        return Ok(())
    }

    match args.command {
        Some(command) => {
            match command {
                Command::Cd => {
                    *curr_path = command_cd(&args.argv, &curr_path, flags)?;
                },
                Command::Ls => {
                    std_stream.write(command_ls(curr_path, flags, &args.argv)?);
                },
                Command::New => {
                    let _ = create_from_path(&curr_path, &args.argv, flags)?;
                },
                Command::Del => {
                    let _ = remove_directory(&curr_path, &args.argv, flags)?;
                },
                Command::Echo => {
                    std_stream.write(command_echo(&args.argv, flags)?);
                },
                Command::Cat => {
                    std_stream.write(command_cat(&args.argv, &std_stream.stdin, &curr_path, flags)?);
                },
                Command::Info => {
                    println!("info")
                },
                Command::Pwd => {
                    std_stream.write(command_pwd(&curr_path, &args.argv, flags)?);
                },
                Command::Move => {
                    //
                    let _ = move_directory(&curr_path, &args.argv, &flags)?;
                }
            }
        },

        None => return Err(AppError::from(ERROR_MISSING_COMMAND))
    }
    Ok(())
}

pub fn flag_error(command: &str, cmd_flags: Flags, user_flags: &Flags) -> Result<()> {
    // Matches the flags the command takes vs what the user suppled, if a command is given that isnt valid for the command
    // this throws an error
    //
    // #arguments
    // *command: A string ref containing the name of the command
    // *cmd_flags: A flags struct containing the valid flags for the command
    // *user_flags: A flags struct containing the flags supplied by the user
    //
    // #returns
    // Returns a empty result enum which returns an error when

    if !cmd_flags.long && user_flags.long {
        return Err(AppError::from(
            Error {
                error_type: ErrorType::InvalidToken,
                error_message: String::leak(format!("Flag 'long' is not supported for command '{}'", command))
            }
            ));
    } else if !cmd_flags.human && user_flags.human {
        return Err(AppError::from(
            Error {
                error_type: ErrorType::InvalidToken,
                error_message: String::leak(format!("Flag 'human' is not supported for command '{}'", command))
            }
            ));
    } else if !cmd_flags.recursive && user_flags.recursive {
        return Err(AppError::from(
            Error {
                error_type: ErrorType::InvalidToken,
                error_message: String::leak(format!("Flag 'recursive' is not supported for command '{}'", command))
            }
            ));
    } else if !cmd_flags.dir && user_flags.dir {
        return Err(AppError::from(
            Error {
                error_type: ErrorType::InvalidToken,
                error_message: String::leak(format!("Flag 'dir' is not supported for command '{}'", command))
            }
            ));
    } else if !cmd_flags.force && user_flags.force {
        return Err(AppError::from(
            Error {
                error_type: ErrorType::InvalidToken,
                error_message: String::leak(format!("Flag 'force' is not supported for command '{}'", command))
            }
            ));
    } else if !cmd_flags.time && user_flags.time {
        return Err(AppError::from(
            Error {
                error_type: ErrorType::InvalidToken,
                error_message: String::leak(format!("Flag 'time' is not supported for command '{}'", command))
            }
            ));
    } else if !cmd_flags.all && user_flags.all {
        return Err(AppError::from(
            Error {
                error_type: ErrorType::InvalidToken,
                error_message: String::leak(format!("Flag 'all' is not supported for command '{}'", command))
            }
            ));
    } else if !cmd_flags.reverse && user_flags.reverse {
        return Err(AppError::from(
            Error {
                error_type: ErrorType::InvalidToken,
                error_message: String::leak(format!("Flag 'reverse' is not supported for command '{}'", command))
            }
            ));
    }

    Ok(())
}

const ERROR_MISSING_COMMAND: Error = Error {
    error_type: ErrorType::NotFound,
    error_message: "Missing command"
};