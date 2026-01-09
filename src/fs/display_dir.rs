use chrono::{DateTime, Local};

use std::path::Path;
use std::fs::{read_dir};
use std::iter::Iterator;

use std::fmt;

use tabled::{builder::Builder, settings::Style, Table};

use crate::Result;

const BITES_PER_NEXT_UNIT: f64 = 1024.0;
const HEADERS: [&str; 3] = [
    "Name",
    "File Type",
    "Date Modified"
];
const HEADERS_LONG: [&str; 6] = [
    "Name",
    "File Type",
    "File Size",
    "Date Modified",
    "Date Created",
    "Accessed"
];


#[derive(Debug)]
pub enum FileType {
    Dir,
    File
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            FileType::Dir => "Directory",
            FileType::File => "File",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct FileEntry {
    pub filename: String,
    pub file_type: FileType,
    pub file_size: u64,
    //pub permissions: Permissions,
    pub date_modified: DateTime<Local>,
    pub date_created: DateTime<Local>,
    pub date_accessed: DateTime<Local>,
}

pub fn read_dir_entries(dir_addr: &Path) -> Result<impl Iterator<Item = Result<FileEntry>>> {
    // From a relative directory address, gives the files and directories founds at that address
    //
    // #Arguments
    // *dir_addr A relative directory address from the script as a reference to a PathBuf
    //
    // #Returns
    // Returns result enum with a vector containing type FileEntry or a std io error

    let entries = read_dir(dir_addr)?;

    Ok(entries.map(|entry| {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let file_type = entry.file_type()?;
        let date_modified: DateTime<Local> = metadata.modified()?.into();
        let date_created: DateTime<Local> = metadata.created()?.into();
        let date_accessed: DateTime<Local> = metadata.accessed()?.into();

        Ok(FileEntry {
            filename: entry.file_name().to_string_lossy().into_owned(),
            file_type: if file_type.is_dir() {
                FileType::Dir
            } else {
                FileType::File
            },
            file_size: metadata.len(),
            //permissions: metadata.permissions(),
            date_modified: date_modified,
            date_created: date_created,
            date_accessed: date_accessed
        })
    }))
}

pub fn build_entry_table(dir_iter: impl Iterator<Item = Result<FileEntry>>, is_long: bool, is_human: bool) -> Result<Table> {
    // Returns a table that tabulates and displays information about each entry in a Directory
    //
    // #Arguments
    // *dir_iter Is an iterator containing Results of FileEntries in a directory
    //
    // #Returns
    // Returns an option with a table or error message

    let mut builder = Builder::new();
    if is_long {
        builder.push_record(HEADERS_LONG);
    } else {
        builder.push_record(HEADERS);
    }

    if is_long {
        dir_iter
        .filter_map(Result::ok)
        .for_each(|entry| {
            let mut size = format!("{}", entry.file_size);

            if is_human {
                size = format_size(entry.file_size);
            };

            builder.push_record([
                entry.filename,
                format!("{}", entry.file_type),
                size,
                format!("{}", entry.date_modified.format("%d-%m-%Y %H:%M:%S")),
                format!("{}", entry.date_created.format("%d-%m-%Y %H:%M:%S")),
                format!("{}", entry.date_accessed.format("%d-%m-%Y %H:%M:%S"))
            ]);
        });
    } else {
        dir_iter
        .filter_map(Result::ok)
        .for_each(|entry| {
            builder.push_record([
                entry.filename,
                format!("{}", entry.file_type),
                format!("{}", entry.date_modified.format("%d-%m-%Y %H:%M:%S"))
            ]);
        });
    }

    let mut table = builder.build();
    table.with(Style::sharp());

    Ok(table)
}


fn format_size(size: u64) -> String {
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
        format!("{:.2} PB", size/1000000.0)
    }
}