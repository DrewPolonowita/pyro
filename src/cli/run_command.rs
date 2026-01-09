use crate::cli::format_input::{Command, Flag, Args};
use crate::cli::help::{command_help, flag_help};

use crate::fs::create_files::{create_directory, remove_directory};
use crate::fs::change_directory::command_cd;

use std::path::{PathBuf, Path};

use crate::fs::display_dir::{read_dir_entries, build_entry_table};
use crate::Result;

pub fn run_command(args: Args, curr_path: &mut PathBuf) -> Result<()> {
    if args.flags.contains(&Flag::Help) {
        match args.command {
            Some(command) => flag_help(command),
            None => command_help()
        }
        return Ok(())
    }

    match args.command {
        Some(command) => {
            match command {
                Command::Cd(value) => {
                    *curr_path = command_cd(value, &curr_path)?;
                },
                Command::Ls => {
                    let mut is_long = false;
                    let mut is_human = false;

                    if args.flags.contains(&Flag::Long) {
                        is_long = true;
                    }
                    if args.flags.contains(&Flag::Human) {
                        is_human = true;
                    }

                    let _ = command_ls(curr_path, is_long, is_human);
                },
                Command::New(dir_name) => {
                    let _ = create_directory(&curr_path, dir_name)?;
                },
                Command::Del(dir_name) => {
                    let mut is_recursive = false;

                    if args.flags.contains(&Flag::Recursive) {
                        is_recursive = true;
                    }

                    let _ = remove_directory(&curr_path, dir_name, is_recursive)?;
                }
            }
        },

        None => todo!()
    }
    Ok(())
}

fn command_ls(curr_path: &Path, is_long: bool, is_human: bool) -> Result<()> {

    let dir_entries = read_dir_entries(&curr_path)?;

    let table = build_entry_table(dir_entries, is_long, is_human)?;

    println!("{}", table);

    Ok(())
}