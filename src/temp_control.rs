use crate::error;
use crate::{get_home, verify_file_and_dir};

use std::fs;

const DEFAULT_FILE_DIR: &str = ".cache/ntsr";
const DEFAULT_FILE_NAME: &str = "temp.toml";

#[must_use]
pub fn get_file() -> fs::File {
    let directory = format!("{}/{DEFAULT_FILE_DIR}", get_home());

    match verify_file_and_dir(DEFAULT_FILE_NAME, directory.as_str()) {
        Ok(verified_path) => match fs::File::create(verified_path) {
            Ok(file) => file,
            Err(e) => error!("{e}"),
        },
        Err(e) => error!("Temp file error: {e}"),
    }
}
