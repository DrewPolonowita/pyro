use crate::Result;

use crate::cli::commands::COMMANDS;
use crate::cli::commands::FLAGS;

use std::iter::Peekable;
use std::str::SplitWhitespace;

#[derive(Debug)]
pub enum Command {
    Cd(String),
    Ls,
    New(String),
    Del(String)

}

#[derive(Debug, PartialEq)]
pub enum Flag {
    Help,
    Long,
    Human,
    Recursive
}

pub struct Args {
    pub command: Option<Command>,
    pub flags: Vec<Flag>
}

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
        return Err("Empty string".into())
    }

    let mut tokens = user_input.split_whitespace().peekable();
    let mut flags: Vec<Flag> = Vec::new();
    let mut main_command: Option<Command> = None;

    while let Some(token) = tokens.next() {

        if COMMANDS.contains(&token) {

            // makes sure there isn't two commands
            if !matches!(main_command, None) {
                return Err(format!(
                    "Cannot have two commands"
                    ).into());
            }

            // creates the command
            main_command = Some(create_command(&mut tokens, token)?);

        } else if FLAGS.contains(&token) {
            let flag = create_flag(token)?;
            flags.push(flag);

        } else {

            return Err(format!(
                "Unknown command {}", token)
                .into());
        }
    }

    Ok(Args {
        command: main_command,
        flags: flags
    })
}

fn create_command(tokens: &mut Peekable<SplitWhitespace<'_>>, token: &str) -> Result<Command> {
    // Returns an option enum with a command enum as a value or no value varient depending on the next token

    match token {
        "cd" => {
            let next_token = create_next_token(tokens)
            .ok_or("Command 'cd' must contain a directory value")?;

            Ok(Command::Cd(format!("{}", next_token)))
        },
        "ls" => {
            Ok(Command::Ls)
        },
        "new" => {
            let next_token = create_next_token(tokens)
            .ok_or("Command 'new' must contain a directory name")?;

            Ok(Command::New(format!("{}", next_token)))
        },
        "del" => {
            let next_token = create_next_token(tokens)
            .ok_or("Command 'del' must contain a directory or file name")?;

            Ok(Command::Del(format!("{}", next_token)))
        }
        _ => unreachable!()
    }
}

fn create_flag(token: &str) -> Result<Flag> {
    match token {
        "--long" | "l" => {
            Ok(Flag::Long)
        },
        "--help" | "h" => {
            Ok(Flag::Help)
        },
        "--human" => {
            Ok(Flag::Human)
        },
        "--recursive" => {
            Ok(Flag::Recursive)
        }
        flag => {
            Err(format!(
                "{} is not a valid flag", flag
                ).into())
        }
    }
}

fn create_next_token(tokens: &mut Peekable<SplitWhitespace<'_>>) -> Option<String> {
    let mut token = String::new();

    loop {
        let next = tokens.next_if(|t| is_next_token_a_value(t));

        let Some(next) = next else {
            break
        };

        token.push_str(&format!(" {}", next));
    }

    Some(token.trim_start().to_string())
}

fn is_next_token_a_value(token: &str) -> bool {
    // Returns true if the next token is a command or flag

    !(COMMANDS.contains(&token) || FLAGS.contains(&token))
}