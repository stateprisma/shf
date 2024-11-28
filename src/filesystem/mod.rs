pub mod query;
pub mod video;

use std::path::PathBuf;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref WORKING_DIR: PathBuf =
        std::env::current_dir().expect("Failed to get working directory");
}
