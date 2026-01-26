use crate::cli::args::Command;

pub fn flag_help(command: Command) -> String {
    match command {
        Command::Ls => {
            format!(
                "{}{}{}{}{}{}{}{}{}",
                "\nCommand 'ls' short for list:\n",
                "\tLists all files and directories in the current working directory; by default sorts alphabetically and doesn't list hidden files/folders\n",
                "Flags:\n",
                "\t--long; -l: Gives more metadata information for each given file\n",
                "\t--human; -h: Displays file sizes in human-readable format (e.g., KB, MB, GB)\n",
                "\t--all; -a: Displays all hidden files/folders in the working directory\n",
                "\t--time; -t: Sorts the files based on the last date modified\n",
                "\t--dir; -d: Lists directories first, then symlinks, then files\n",
                "\t--reverse; -R: Reverses the listing order\n"
            )
        },
        Command::Cd => {
            format!(
                "{}{}",
                "\nCommand 'cd' <key> short for change directory:\n",
                "\tThis command will move into the directory <key> from the current directory\n"
            )
        },
        Command::New => {
            format!(
                "{}{}{}{}{}{}",
                "\nCommand 'new' <key>:\n",
                "\tThis command will create a new file by default in the target directory\n",
                "Flags:\n",
                "\t--dir; -d: Creates a directory with the specified name <key>\n",
                "\t--recursive; -r: Recursively creates all parent directories required for the given path\n",
                "\t--force; -f: Replaces a given file or directory if it exists (will not replace files with folders and vise versa)\n"
            )
        },
        Command::Del => {
            format!(
                "{}{}{}{}",
                "\nCommand 'del' <key> short for delete:\n",
                "\tThis command will remove a file or directory from the current directory with the name <key>\n",
                "Flags:\n",
                "\t--recursive; -r: Removes all file contents of a directory recursively, only works on a directory\n"
                    )
        },
        Command::Echo => {
            format!(
                "{}{}",
                "\nCommand 'echo' <key>\n",
                "\tThis command will print to stdout with the value of <key>\n"
            )
        },
        Command::Cat => {
            format!(
                "{}{}",
                "\nCommand 'cat' <key> <key> ...\n",
                "\tThis command will get the file contents of <key> and send the result to stdout. Can have any number of keys. If no arguments are given then cat will send the result of stdin to stdout\n",
            )
        },
        Command::Info => {
            format!("")
        },
        Command::Pwd => {
            format!(
                "{}{}",
                "\nCommand 'pwd' short for print working directory\n",
                "\tThis command will output the current working directory to stdout\n",
            )
        },
        Command::Move => {
            format!("")
        }
    }
}

pub fn command_help() -> String {
    format!("{}", "big helper right here")
}