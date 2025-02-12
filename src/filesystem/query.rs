use serde::Serialize;
use serde_repr::Serialize_repr;
use std::fs;
use std::io;
use std::time::UNIX_EPOCH;

use super::escape_path;
use super::WORKING_DIR;

#[derive(Serialize_repr, Debug)]
#[repr(u8)]
pub enum FileEntryType {
    Unknown = 0,
    File = 1,
    Directory = 2,
    Symlink = 3,
}

#[derive(Serialize, Debug)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub type_: FileEntryType,
    pub last_modified: u64,
}

pub fn list_directory(relative_path: &str) -> io::Result<Vec<FileEntry>> {
    let target_path = escape_path(relative_path);

    if !target_path.starts_with(&*WORKING_DIR) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Attempt to traverse outside of the working directory.",
        ));
    }

    let mut entries = Vec::new();

    for entry in fs::read_dir(target_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        let entry_type = if metadata.is_file() {
            FileEntryType::File
        } else if metadata.is_dir() {
            FileEntryType::Directory
        } else if metadata.file_type().is_symlink() {
            FileEntryType::Symlink
        } else {
            FileEntryType::Unknown
        };

        let last_modified = metadata
            .modified()
            .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_secs())
            .unwrap_or(0);

        entries.push(FileEntry {
            name: entry.file_name().into_string().unwrap_or_default(),
            size: metadata.len(),
            type_: entry_type,
            last_modified,
        });
    }

    Ok(entries)
}
