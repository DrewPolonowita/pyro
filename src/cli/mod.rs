pub mod error;
pub mod format_input;
pub mod args;
pub mod run_command;

mod help;

use crate::cli::format_input::{format_input, parse_pipe_args};
use crate::cli::run_command::run_command;
use crate::cli::args::ArgRunStatus;

use std::path::PathBuf;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use crate::Result;

struct StdStream {
    stdin: String,
    stdout: String
}

impl StdStream {
    pub fn write(&mut self, arg: String) {
        self.stdout += &arg;
    }
    fn clear_out(&mut self) {
        self.stdout = String::new();
    }
    fn clear_in(&mut self) {
        self.stdin = String::new();
    }
    fn pipe(&mut self) {
        self.stdin = self.stdout.clone();
        self.stdout = String::new();
    }
}

pub fn repl() -> Result<()>{
<<<<<<< HEAD
    let mut curr_path = env::current_dir()?;
    //let mut curr_path = PathBuf::from(r"C:\Users\Drew\Documents\Rust Apps\pyro\test");
=======
    let mut curr_path = std::env::current_dir()?;
    // let _ = curr_path.pop();
>>>>>>> 30387090dd207a1b6c84ce439902c0a50b388602
    let mut std_stream = StdStream {
        stdin: String::new(),
        stdout: String::new()
    };

    loop {
        let mut input_string = String::new();
        print!("{} >>> ", curr_path.display());
        stdout().flush().unwrap();

        stdin()
            .read_line(&mut input_string)?;

        let mut input_strings_by_pipes = parse_pipe_args(&mut input_string);
        let mut last_operation_failed = false;

        while let Some(input) = input_strings_by_pipes.next() {
            last_operation_failed = match input {
                ArgRunStatus::Conditional(input) => {

                    if !last_operation_failed {
                        let temp = run_args(&mut curr_path, &input, &mut std_stream);
                        std_stream.clear_in();
                        temp
                    } else {
                        true
                    }
                },
                ArgRunStatus::UnConditional(input) => {
                    let temp = run_args(&mut curr_path, &input, &mut std_stream);
                    std_stream.clear_in();
                    temp
                },
                ArgRunStatus::Pipeline(input) => {
                    let temp = run_args(&mut curr_path, &input, &mut std_stream);
                    std_stream.pipe();
                    temp
                }
            };


            if !std_stream.stdout.is_empty() {
                println!("{}", std_stream.stdout);
                std_stream.clear_out();
            }
        }
    }
}

fn run_args(curr_path: &mut PathBuf, input_string: &String, std_stream: &mut StdStream) -> bool {
    match format_input(input_string) {

        Ok(input) => {
            match run_command(input, curr_path, std_stream) {

                Ok(_) => {
                    return false;
                },

                Err(e) => {
                    eprintln!("{}", e);
                    return true;
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            return true;
        }
    }
}