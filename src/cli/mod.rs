mod format_input;
mod commands;
mod run_command;
mod help;

use crate::cli::format_input::format_input;
use crate::cli::run_command::run_command;

use std::path::PathBuf;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
//use std::env;

use crate::Result;

pub fn repl() -> Result<()>{
    //let mut curr_path = env::current_exe()?;
    let mut curr_path = PathBuf::from(r"C:\\Users\Drew\Documents\Rust Programs\CLI File Explorer");
    loop {
        let mut input_string = String::new();
        print!("{} >>> ", curr_path.display());
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut input_string)?;

        match format_input(&input_string) {

            Ok(input) => {
                match run_command(input, &mut curr_path) {

                    Ok(_) => {},

                    Err(err) => eprintln!("{}", err)

                }
            }

            Err(err) => eprintln!("{}", err)
        };
    }
}