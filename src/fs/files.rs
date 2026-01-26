use chrono::{DateTime, Local};
use std::cmp::Ordering;

pub const HEADERS: [&str; 3] = [
    "Name",
    "File Type",
    "Date Modified"
];
pub const HEADERS_LONG: [&str; 6] = [
    "Name",
    "File Type",
    "File Size",
    "Date Modified",
    "Date Created",
    "Accessed"
];

#[derive(Clone)]
pub enum FileType {
    Dir,
    File,
    Symlink
}

impl From<std::fs::FileType> for FileType {
    fn from(file_type: std::fs::FileType) -> FileType {
        if file_type.is_dir() {
            FileType::Dir
        } else if file_type.is_file() {
            FileType::File
        } else {
            FileType::Symlink
        }
    }
}

impl PartialEq for FileType {
    fn eq(&self, other: &FileType) -> bool {
        match (self, other) {
            (FileType::Dir, FileType::Dir) => true,
            (FileType::Dir, FileType::File) => false,
            (FileType::Dir, FileType::Symlink) => false,
            (FileType::File, FileType::Dir) => false,
            (FileType::File, FileType::File) => true,
            (FileType::File, FileType::Symlink) => false,
            (FileType::Symlink, FileType::Dir) => false,
            (FileType::Symlink, FileType::File) => false,
            (FileType::Symlink, FileType::Symlink) => true,
        }
    }
}

impl Eq for FileType {}

impl PartialOrd for FileType {
    fn partial_cmp(&self, other: &FileType) -> Option<Ordering> {
        match (self, other) {
            (FileType::Dir, FileType::Dir) => Some(Ordering::Equal),
            (FileType::Dir, FileType::File) => Some(Ordering::Less),
            (FileType::Dir, FileType::Symlink) => Some(Ordering::Less),
            (FileType::File, FileType::Dir) => Some(Ordering::Greater),
            (FileType::File, FileType::File) => Some(Ordering::Equal),
            (FileType::File, FileType::Symlink) => Some(Ordering::Greater),
            (FileType::Symlink, FileType::Dir) => Some(Ordering::Greater),
            (FileType::Symlink, FileType::File) => Some(Ordering::Less),
            (FileType::Symlink, FileType::Symlink) => Some(Ordering::Equal),
        }
    }
}

impl Ord for FileType {
    fn cmp(&self, other: &Self) -> Ordering {
        // delegate to your partial_cmp (unwrap because you know it's total)
        self.partial_cmp(other).unwrap()
    }
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            FileType::Dir => "Directory",
            FileType::File => "File",
            FileType::Symlink => "Symlink"
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone)]
pub enum OptionDate {
    Some(DateTime<Local>),
    None
}

impl PartialEq for OptionDate {
    fn eq(&self, other: &OptionDate) -> bool {
        match (self, other) {
            (OptionDate::Some(self_time), OptionDate::Some(other_time)) => self_time == other_time,
            (OptionDate::Some(_), OptionDate::None) => false,
            (OptionDate::None, OptionDate::Some(_)) => false,
            (OptionDate::None, OptionDate::None) => true
        }
    }
}

impl Eq for OptionDate {}

impl PartialOrd for OptionDate {
    fn partial_cmp(&self, other: &OptionDate) -> Option<Ordering> {
        match (self, other) {
            (OptionDate::Some(self_time), OptionDate::Some(other_time)) => Some(self_time.cmp(other_time)),
            (OptionDate::Some(_), OptionDate::None) => Some(Ordering::Greater),
            (OptionDate::None, OptionDate::Some(_)) => Some(Ordering::Less),
            (OptionDate::None, OptionDate::None) => Some(Ordering::Equal)
        }
    }
}

impl Ord for OptionDate {
    fn cmp(&self, other: &Self) -> Ordering {
        // delegate to your partial_cmp (unwrap because you know it's total)
        self.partial_cmp(other).unwrap()
    }
}

impl From<std::result::Result<std::time::SystemTime, std::io::Error>> for OptionDate {
    fn from(date: std::result::Result<std::time::SystemTime, std::io::Error>) -> OptionDate {
        match date {
            Ok(date) => {
                OptionDate::Some(date.into())
            },
            Err(_) => OptionDate::None
        }
    }
}
impl std::fmt::Display for OptionDate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OptionDate::Some(date_time) => write!(f, "{}", date_time.format("%d-%m-%Y %H:%M:%S")),
            OptionDate::None => write!(f, ""),
        }
    }
}

pub struct FileEntry {
    pub filename: String,
    pub file_type: FileType,
    pub file_size: u64,
    pub date_modified: OptionDate,
    pub date_created: OptionDate,
    pub date_accessed: OptionDate,
}