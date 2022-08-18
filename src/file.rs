use std::fs;
use std::path::Path;

use crate::error;

pub fn create(path_str: &String) {
    let filename = match Path::new(path_str.as_str()).file_name() {
        Some(name) => match name.to_str() {
            Some(name_str) => name_str,
            None => error!("Could not parse '{name:#?}' into string"),
        },
        None => error!("Could not get file name from '{path_str}'"),
    };

    let dir_path = path_str.trim_end_matches(filename.trim_end_matches('/'));

    if let Some(e) = fs::create_dir_all(dir_path).err() {
        error!("Could not create directory '{dir_path}: {e}");
    }

    if let Some(e) = fs::File::options()
        .read(true)
        .write(true)
        .create(true)
        .open(&path_str)
        .err()
    {
        error!("Could not create file '{path_str}': {e}");
    }
}
