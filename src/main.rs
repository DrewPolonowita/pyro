use ctrlc;

mod cli;
mod fs;

use crate::cli::repl;
use std::process::exit;

use crate::cli::error::Result;

fn main() -> Result<()> {
    /*
    let test_dir = PathBuf::from(".");

    let entry_iter = read_dir_entries(&test_dir)?;
    let table = build_entry_table(entry_iter)?;

    println!("{}", table);
    */

    ctrlc::set_handler(move || {

        // Add proper exit codes //

        exit(0);
    })
    .expect("Error setting Ctrl-C handler");




    let _ = repl();

    Ok(())
}