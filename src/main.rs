use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use clap::{Parser, Subcommand};
use serde_derive::Deserialize;

const CONF_DIR: &str = ".config/noteiser/config.toml";

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true )]
struct Cli {
    /// Specify a specific editor
    #[clap(short, long, value_parser)]
    editor: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct TomlConfig {
    dev: String,
    editor: Option<String>,
    editor_backup: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Edit config
    Config {
        #[clap(subcommand)]
        command: Option<ConfigCommands>,
    },
    /// Open a file in your editor
    Open {
        /// File to open
        #[clap(value_parser)]
        file_name: String,
    },
    /// Rust specific functions
    Rust {
        #[clap(subcommand)]
        command: RustCommands,
    },
}

#[derive(Subcommand)]
enum RustCommands {
    /// Create a new rust project
    New {
        /// Name of project to create
        #[clap(value_parser)]
        project_name: String,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Get your config file in struct notation
    Get,
    /// Open up the config file
    Open,
    /// Setup the config file
    Setup,
}

struct Noteiser {}

impl Noteiser {
    fn cli() -> Cli {
        Cli::parse()
    }

    fn config() -> Result<TomlConfig, String> {
        match File::open(Self::config_dir().as_str()) {
            Ok(mut file) => {
                let mut contents = String::new();

                match file.read_to_string(&mut contents) {
                    Ok(_) => match toml::from_str::<TomlConfig>(&contents) {
                        Ok(config) => return Ok(config),
                        Err(e) => {
                            println!("Could not read contents to string: {e}");
                            std::process::exit(0x1000);
                        }
                    },
                    Err(e) => {
                        println!("Config file contents not found: {e}");
                        std::process::exit(0x1000);
                    }
                }
            }

            Err(e) => Err(format!("Config file not found: {e}")),
        }
    }

    fn config_dir() -> String {
        let path_string = format!("{}/{CONF_DIR}", Self::home());

        // Check if config location exists
        match Self::verify_filename(path_string.as_str()) {
            Some(_) => return path_string.trim().to_string(),
            None => {
                println!("Couldn't find config directory");
                std::process::exit(0x1000);
            }
        }
    }

    fn config_open() {
        Self::run_editor(Self::config_dir().as_str());
    }

    fn editor() -> String {
        // TODO add resilliance to this to check if they are valid editors

        if let Some(cli_editor) = Self::cli().editor {
            return cli_editor;
        }

        if let Ok(config) = Self::config() {
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

    fn home() -> String {
        match std::env::var("HOME") {
            Ok(home) => home,
            Err(e) => {
                println!("Couldn't find home directory: {e}");
                std::process::exit(0x1000);
            }
        }
    }

    fn match_command(cli: Cli) {
        match &cli.command {
            Commands::Open { file_name } => {
                Self::run_editor(file_name);
                std::process::exit(0x1000);
            }
            Commands::Rust { command } => match &command {
                RustCommands::New { project_name } => match Self::config() {
                    Ok(config) => {
                        let filename_string = format!("{}/Rust/{project_name}", config.dev);

                        match Self::verify_filename(&filename_string.clone()) {
                            Some(name) => {
                                Self::run_command("cargo", vec!["-q", "new", name]);

                                println!("Project '{project_name}' created successfully");

                                Self::run_editor(format!("{name}/src/main.rs").as_str());
                            }
                            None => {
                                println!("Project '{project_name}' already exists");
                                std::process::exit(0x1000);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Config file not found: {e}");
                        std::process::exit(0x1000);
                    }
                },
            },
            Commands::Config {
                command: command_maybe,
            } => match command_maybe {
                Some(command) => match command {
                    ConfigCommands::Open => {
                        Self::config_open();
                    }
                    ConfigCommands::Get => match Self::config() {
                        Ok(config) => println!("{:#?}", config),
                        Err(e) => {
                            println!("{e}");
                            std::process::exit(0x1000);
                        }
                    },
                    ConfigCommands::Setup => match Self::config() {
                        Ok(_) => {
                            println!("Config file already exists");
                            std::process::exit(0x1000);
                        }
                        Err(_) => {
                            let config_path_string = format!("{}/.config/noteiser", Self::home());

                            Self::run_command("mkdir", vec!["-p", config_path_string.as_str()]);

                            Self::run_editor(config_path_string.as_str());
                        }
                    },
                },
                None => Self::config_open(),
            },
        }
    }

    fn run_command(command: &str, args: Vec<&str>) {
        match std::process::Command::new(command)
            .args(args.clone())
            .status()
        {
            Ok(_) => {}
            Err(e) => {
                println!(
                    "Could not run command: '{command}' with args: {:?}\n Error: {e}",
                    args
                );
                std::process::exit(0x1000);
            }
        }
    }

    fn run_editor(filepath: &str) {
        match Self::verify_filename(filepath) {
            Some(name) => Self::run_command(Self::editor().clone().as_str(), vec![name]),
            None => {
                println!("Editor could not find file '{filepath}'");
                std::process::exit(0x1000);
            }
        }
    }

    fn start(cli: Cli) {
        Self::match_command(cli);
    }

    fn verify_filename<'a>(filename: &'a str) -> Option<&'a str> {
        // Check if config location exists
        match Path::new(filename).canonicalize() {
            Ok(_) => Some(filename.trim()),
            Err(_) => None,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    Noteiser::start(cli);
}
