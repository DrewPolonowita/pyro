use std::path::Path;
use std::fs::{read_dir};
use std::iter::Iterator;

use tabled::{builder::Builder, settings::Style, Table};

use crate::cli::error::Result;

use crate::cli::run_command::flag_error;
use crate::cli::args::{FlagsBuilder, Flags};

use crate::fs::files::{FileType, OptionDate, FileEntry, HEADERS, HEADERS_LONG};
use crate::cli::error::{AppError, Error, ErrorType};

const BITES_PER_NEXT_UNIT: f64 = 1024.0;
const FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002;

#[cfg(windows)]
fn is_file_hidden(metadata: std::fs::Metadata, _filename: std::ffi::OsString) -> bool {
    // Checks if the file is hidden by checking the hidden attribute on windows

    use std::os::windows::fs::MetadataExt;
    metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0
}

#[cfg(unix)]
fn is_file_hidden(_metadata: std::fs::Metadata, filename: std::ffi::OsString) -> bool {
    // Checks if the file is hidden on unix by checking the first byte in the file name is a .

    if let Some(&first_byte) = filename.as_encoded_bytes().get(0) {
        first_byte == 0x2E
    } else {
        false
    }
}

pub fn command_ls(curr_path: &Path, flags: Flags, argv: &[String]) -> Result<String> {
    // Lists all files in a given directory and displays them using a table with optional user Flags
    //
    // #arguments
    // curr_path: A reference to the directory to displays
    // flags: A Flags struct with the user input Flags
    //
    // #returns
    // Returns an ok result enum with a formatted string containing the table to be printed to the console
    // Returns an err result enum when the path doesn't exist, the flags aren't supported, there is an argument given etc.
    //

    // Causes an error if a not supported flag is given
    let _ = flag_error("ls", {
        FlagsBuilder::new()
        .long()
        .human()
        .all()
        .time()
        .dir()
        .reverse()
        .build()
    }, &flags)?;

    // Causes an error if the user supplies an argument
    if let Some(_) = argv.get(0) {
        return Err(AppError::from(ERROR_PROVIDED_ARGS));
    }

    let dir_entries = read_dir_entries(&curr_path, &flags)?;
    let mut dir_entries: Vec<FileEntry> = dir_entries.collect();

    if flags.time {
        dir_entries.sort_by_key(|item| item.date_modified.clone());
    }
    if flags.dir {
        dir_entries.sort_by_key(|item| item.file_type.clone());
    }
    if flags.reverse {
        dir_entries.reverse();
    }

    let table = build_entry_table(dir_entries, &flags)?;

    Ok(format!("{}", table))
}

fn read_dir_entries(dir_addr: &Path, flags: &Flags) -> Result<impl Iterator<Item = FileEntry>> {
    // From a relative directory address, gives the files and directories founds at that address
    //
    // #Arguments
    // *dir_addr A relative directory address from the script as a reference to a PathBuf
    //
    // #Returns
    // Returns result enum with a vector containing type FileEntry or a std io error

    let entries = read_dir(dir_addr)?;

    Ok(entries.filter_map(|entry| {
        let Ok(entry) = entry else {
            return None;
        };

        let Ok(metadata) = entry.metadata() else {
            return None;
        };

        let filename = entry.file_name().to_string_lossy().into_owned();
        let file_type = FileType::from(metadata.file_type());
        let file_size = metadata.len();

        let date_modified = OptionDate::from(metadata.modified());
        let date_created = OptionDate::from(metadata.created());
        let date_accessed = OptionDate::from(metadata.accessed());

        if !flags.all && is_file_hidden(metadata, entry.file_name()) {
            return None
        }

        Some(FileEntry {
            filename: filename,
            file_type: file_type,
            file_size: file_size,
            date_modified: date_modified,
            date_created: date_created,
            date_accessed: date_accessed,
        })
    }))
}

fn build_entry_table(dir_iter: Vec<FileEntry>, flags: &Flags) -> Result<Table> {
    // Returns a table that tabulates and displays information about each entry in a Directory
    //
    // #Arguments
    // *dir_iter Is an iterator containing Results of FileEntries in a directory
    //
    // #Returns
    // Returns an option with a table or error message

    let mut builder = Builder::new();
    if flags.long {
        builder.push_record(HEADERS_LONG);
    } else {
        builder.push_record(HEADERS);
    }

    for entry in dir_iter {
        let record = if flags.long {
            let mut size = format!("{}", entry.file_size);

            if flags.human {
                size = format_size(entry.file_size);
            };

            vec![
                entry.filename,
                format!("{}", entry.file_type),
                size,
                format!("{}", entry.date_modified),
                format!("{}", entry.date_created),
                format!("{}", entry.date_accessed),
            ]
        } else {
            vec![
                entry.filename,
                format!("{}", entry.file_type),
                format!("{}", entry.date_modified)
            ]
        };

        builder.push_record(record);
    };

    let mut table = builder.build();
    table.with(Style::sharp());

    Ok(table)
}


fn format_size(size: u64) -> String {
    // Formats a u64 representing the size in bits into a string of form xxx unit

    let size = size as f64;

    if size < 1024.0 {
        format!("{} b", size)
    } else if size < BITES_PER_NEXT_UNIT.powf(2.0) {
        format!("{:.2} KB", size/BITES_PER_NEXT_UNIT)
    } else if size < BITES_PER_NEXT_UNIT.powf(3.0) {
        format!("{:.2} MB", size/BITES_PER_NEXT_UNIT.powf(2.0))
    } else if size < BITES_PER_NEXT_UNIT.powf(4.0) {
        format!("{:.2} GB", size/BITES_PER_NEXT_UNIT.powf(3.0))
    } else if size < BITES_PER_NEXT_UNIT.powf(5.0) {
        format!("{:.2} TB", size/BITES_PER_NEXT_UNIT.powf(4.0))
    } else {
        format!("{:.2} PB", size/BITES_PER_NEXT_UNIT.powf(5.0))
    }
}

const ERROR_PROVIDED_ARGS: Error = Error {
    error_type: ErrorType::InvalidToken,
    error_message: "Command 'ls' does not take any arguments"
};