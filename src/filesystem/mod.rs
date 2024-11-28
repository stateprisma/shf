pub mod query;
pub mod video;

use std::path::PathBuf;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref WORKING_DIR: PathBuf =
        std::env::current_dir().expect("Failed to get working directory");
}

pub fn escape_path(relative_path: &str) -> PathBuf {
    if relative_path.starts_with('/') {
        WORKING_DIR.join(&relative_path[1..])
    } else if relative_path.contains("..") {
        println!("[warn] Attempted use of \"..\" blocked");
        escape_path(&relative_path.replace("../", "").replace("..", ""))
    } else {
        WORKING_DIR.join(relative_path)
    }
}