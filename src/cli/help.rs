use crate::cli::format_input::Command;

pub fn flag_help(command: Command) {
    match command {
        Command::Ls => {
            println!("\nCommand 'ls' short for list:");
            println!("\tThis command will list all files and directories within the current working directory in alphabetical order");
            println!("Flags:");
            println!("\t--long: Gives more metadata information for each given file");
            println!("\t--human: Gives a more human readable representation of the data (WIP)");
            println!("\t--all: Lists all directories and files including hidden files (WIP)");
            println!("\t--time: Sorts the data based on the last time accessed (WIP)");
            println!("\t--reverse: Reverses the current sorting pattern (WIP)");
            println!("")
        },
        Command::Cd(_) => {
            println!("Help information for change directories command (WIP)")
        },
        Command::New(_) => {
            println!("Help information for make dir command (WIP)")
        },
        Command::Del(_) => {
            println!("\nCommand 'del' <key> short for delete:");
            println!("\tThis command will remove a file or directory from the current directory with the name <key>");
            println!("Flags:");
            println!("\t--recursive: Removes all file contents of a directory recursively, only works on a directory");
            println!("")
        }
    }
}

pub fn command_help() {
    println!("big helper right here");
}