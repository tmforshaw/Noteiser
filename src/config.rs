use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

use crate::commands::{run_command, run_editor};
use crate::error;
use crate::getters::{home, verify_filename};

use clap::Subcommand;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Toml {
    pub dev: String,
    pub editor: Option<String>,
    pub editor_backup: Option<String>,
    pub note: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get your config file in struct notation
    Get,
    /// Open up the config file
    Open,
    /// Setup the config file
    Setup,
}

const CONF_DIR: &str = ".config/noteiser/config.toml";

/// # Errors
///
/// Will return `Err` if config file does not exist or can't be read
pub fn get() -> Result<Toml, String> {
    match File::open(get_dir().as_str()) {
        Ok(mut file) => {
            let mut contents = String::new();

            match file.read_to_string(&mut contents) {
                Ok(_) => match toml::from_str::<Toml>(&contents) {
                    Ok(config) => Ok(config),
                    Err(e) => error!("Could not read contents to string: {e}"),
                },
                Err(e) => error!("Config file contents not found: {e}"),
            }
        }
        // No config was found
        Err(e) => Err(format!("Config file not found: {e}")),
    }
}

#[must_use]
pub fn get_dir() -> String {
    let path_string = format!("{}/{CONF_DIR}", home());

    // Check if config location exists
    match verify_filename(path_string.as_str()) {
        Some(_) => return path_string.trim().to_string(),
        None => error!("Couldn't find config directory"),
    }
}

pub fn open_in_editor() {
    run_editor(get_dir().as_str());
}

pub fn parse_command(command_maybe: &Option<Commands>) {
    match command_maybe {
        Some(command) => match command {
            Commands::Open => {
                open_in_editor();
            }
            Commands::Get => match get() {
                Ok(config) => println!("{:#?}", config),
                Err(e) => error!("{e}"),
            },
            Commands::Setup => {
                if get().is_ok() {
                    error!("Config file already exists")
                } else {
                    let config_path_string = format!("{}/.config/noteiser", home());

                    run_command("mkdir", &vec!["-p", config_path_string.as_str()]);

                    run_editor(config_path_string.as_str());
                }
            }
        },
        None => open_in_editor(),
    }
}
