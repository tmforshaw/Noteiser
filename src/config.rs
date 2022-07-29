use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;

use crate::commands::{run_command, run_editor};
use crate::error;
use crate::getters::{home, verify_filename};

use clap::Subcommand;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct TomlConf {
    pub dev: String,
    pub editor: Option<String>,
    pub editor_backup: Option<String>,
    pub note: Option<String>,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Get your config file in struct notation
    Get,
    /// Open up the config file
    Open,
    /// Setup the config file
    Setup,
}

const CONF_DIR: &str = ".config/noteiser/config.toml";

pub fn config() -> Result<TomlConf, String> {
    match File::open(config_dir().as_str()) {
        Ok(mut file) => {
            let mut contents = String::new();

            match file.read_to_string(&mut contents) {
                Ok(_) => match toml::from_str::<TomlConf>(&contents) {
                    Ok(config) => Ok(config),
                    Err(e) => error!("Could not read contents to string: {e}"),
                },
                Err(e) => error!("Config file contents not found: {e}"),
            }
        }
        Err(e) => Err(format!("Config file not found: {e}")),
    }
}

pub fn config_dir() -> String {
    let path_string = format!("{}/{CONF_DIR}", home());

    // Check if config location exists
    match verify_filename(path_string.as_str()) {
        Some(_) => return path_string.trim().to_string(),
        None => error!("Couldn't find config directory"),
    }
}

pub fn config_open() {
    run_editor(config_dir().as_str());
}

pub fn match_config(command_maybe: &Option<ConfigCommands>) {
    match command_maybe {
        Some(command) => match command {
            ConfigCommands::Open => {
                config_open();
            }
            ConfigCommands::Get => match config() {
                Ok(config) => println!("{:#?}", config),
                Err(e) => error!("{e}"),
            },
            ConfigCommands::Setup => match config() {
                Ok(_) => error!("Config file already exists"),
                Err(_) => {
                    let config_path_string = format!("{}/.config/noteiser", home());

                    run_command("mkdir", &vec!["-p", config_path_string.as_str()]);

                    run_editor(config_path_string.as_str());
                }
            },
        },
        None => config_open(),
    }
}
