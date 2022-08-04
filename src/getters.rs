use std::fs;
use std::path::Path;

use clap::Parser;

use crate::commands::Commands;
use crate::config::config;
use crate::error;

// TODO fix naming system

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true )]
pub struct Cli {
    /// Specify a specific editor
    #[clap(short, long, value_parser)]
    pub editor: Option<String>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[must_use]
pub fn cli() -> Cli {
    Cli::parse()
}

#[must_use]
pub fn editor() -> String {
    // TODO add resilliance to this to check if they are valid editors

    if let Some(cli_editor) = cli().editor {
        return cli_editor;
    }

    if let Ok(config) = config() {
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
pub fn list_files(directory: &String) -> String {
    let dir_paths = match fs::read_dir(directory) {
        Ok(read_dir_path) => read_dir_path,
        Err(e) => error!("Error while finding files: {e}"),
    };

    let mut message = format!("Contents of '{directory}':\n");

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

                message.push_str(format!("\t{}\n", file_name_string).as_str());
            }
            Err(e) => error!("Could not display file: {e}"),
        }
    }

    message.trim().to_string()
}

#[must_use]
pub fn home() -> String {
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
