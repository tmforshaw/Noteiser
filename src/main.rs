#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

pub mod commands;
pub mod config;
pub mod note;
pub mod rust;
pub mod temp_control;

use std::fs;
use std::path::Path;

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true )]
pub struct Cli {
    /// Specify a specific editor
    #[clap(short, long, value_parser)]
    pub editor: Option<String>,

    #[clap(subcommand)]
    pub command: commands::Commands,
}

#[must_use]
pub fn cli() -> Cli {
    Cli::parse()
}

#[must_use]
pub fn get_editor() -> String {
    // TODO add resilliance to this to check if they are valid editors

    if let Some(cli_editor) = cli().editor {
        return cli_editor;
    }

    if let Ok(config) = config::get() {
        if let Some(conf_editor) = config.editor {
            return conf_editor;
        } else if let Some(conf_editor_backup) = config.editor_backup {
            return conf_editor_backup;
        } else if let Ok(env_editor) = std::env::var("EDITOR") {
            return env_editor;
        }
    }

    if let Ok(env_editor) = std::env::var("EDITOR") {
        env_editor
    } else {
        error!("No available editors to use");
    }
}

#[must_use]
pub fn get_files_vec(directory: &String) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    let dir_paths = match fs::read_dir(directory) {
        Ok(read_dir_path) => read_dir_path,
        Err(e) => error!("Error while finding files: {e}"),
    };

    for path in dir_paths {
        match path {
            Ok(file) => {
                let file_path = file.path();

                let file_name = match file_path.strip_prefix(directory) {
                    Ok(stripped_file_name) => stripped_file_name,
                    Err(e) => error!(
                        "Could not get file name from path '{}': {e}",
                        file_path.display()
                    ),
                };

                let file_name_string = match file_name.to_str() {
                    Some(string) => string,
                    None => error!("Could not parse file name to string {:?}", file_name),
                };

                files.push(file_name_string.to_string());
            }
            Err(e) => error!("Could not display file: {e}"),
        }
    }

    files
}

#[must_use]
pub fn list_files(directory: &String) -> String {
    list_matching_files(directory, r".*")
}

#[must_use]
pub fn list_matching_files(directory: &String, regex_string: &str) -> String {
    let mut message = format!("Contents of '{directory}':\n");

    let files = get_files_vec(directory);

    let regex = match Regex::new(regex_string) {
        Ok(re) => re,
        Err(e) => error!("Error with list regex: {e}"), // User should not receive this message
    };

    let filtered_files_iter = files.iter().filter(|f| regex.is_match(f));

    for file in filtered_files_iter {
        message.push_str(format!("\t{}\n", file).as_str());
    }

    message.trim().to_string()
}

#[must_use]
pub fn get_home() -> String {
    match std::env::var("HOME") {
        Ok(home) => home,
        Err(e) => error!("Couldn't find home directory: {e}"),
    }
}

#[must_use]
pub fn verify_filename(filename: &str) -> Option<&str> {
    // Check if config location exists
    match Path::new(filename).canonicalize() {
        Ok(_) => Some(filename.trim()),
        Err(_) => None,
    }
}

/// # Errors
/// Returns an error if the file is not found, if directory is not found it panics
pub fn verify_file_and_dir(filename: &str, directory: &str) -> Result<String, String> {
    let full_path = format!("{directory}/{filename}");

    match verify_filename(directory) {
        Some(_) => match verify_filename(full_path.as_str()) {
            Some(_) => Ok(full_path),
            None => Err("File not found".to_string()),
        },
        None => error!("Directory not found"), // TODO allow this to be rectified
    }
}

fn main() {
    let cli = cli();

    commands::match_command(&cli);
}
