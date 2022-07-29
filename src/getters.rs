use std::path::Path;

use clap::Parser;

use crate::commands::Commands;
use crate::config::config;

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true )]
pub struct Cli {
    /// Specify a specific editor
    #[clap(short, long, value_parser)]
    pub editor: Option<String>,

    #[clap(subcommand)]
    pub command: Commands,
}

pub fn cli() -> Cli {
    Cli::parse()
}

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
        println!("No available editors to use");
        std::process::exit(0x1000);
    }
}

pub fn home() -> String {
    match std::env::var("HOME") {
        Ok(home) => home,
        Err(e) => {
            println!("Couldn't find home directory: {e}");
            std::process::exit(0x1000);
        }
    }
}

pub fn verify_filename<'a>(filename: &'a str) -> Option<&'a str> {
    // Check if config location exists
    match Path::new(filename).canonicalize() {
        Ok(_) => Some(filename.trim()),
        Err(_) => None,
    }
}