use lazy_static::lazy_static;
use serde::Serialize;
use std::fs;
use std::io;
use std::path::PathBuf;

lazy_static! {
    static ref WORKING_DIR: PathBuf =
        std::env::current_dir().expect("Failed to get working directory");
}

#[derive(Serialize, Debug)]
pub enum FileEntryType {
    File,
    Directory,
    Symlink,
    Unknown,
}

#[derive(Serialize, Debug)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub type_: FileEntryType,
}

fn escape_path(relative_path: &str) -> PathBuf {
    if relative_path.starts_with('/') {
        WORKING_DIR.join(&relative_path[1..])
    } else if relative_path.contains("..") {
        println!("[warn] Attempted use of \"..\" blocked");
        escape_path(&relative_path.replace("../", "").replace("..", ""))
    } else {
        WORKING_DIR.join(relative_path)
    }
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

        entries.push(FileEntry {
            name: entry.file_name().into_string().unwrap_or_default(),
            size: metadata.len(),
            type_: entry_type,
        });
    }

    Ok(entries)
}
