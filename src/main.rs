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
    editor: Option<String>,
    editor_backup: Option<String>,
    dev: String,
}

#[derive(Subcommand)]
enum Commands {
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
    /// Edit config
    Config {
        #[clap(subcommand)]
        command: Option<ConfigCommands>,
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
    /// Open up the config file
    Open,
    Get,
    // /// Setup the config file
    // Setup,
}

struct Noteiser {}

impl Noteiser {
    fn editor() -> String {
        if let Some(cli_editor) = Self::cli().editor {
            cli_editor
        } else if let Some(conf_editor) = Self::config().editor {
            conf_editor
        } else if let Some(conf_editor) = Self::config().editor_backup {
            conf_editor
        } else if let Ok(env_editor) = std::env::var("EDITOR") {
            env_editor
        } else {
            println!("No available editors to use");
            std::process::exit(0x1000);
        }
    }

    fn cli() -> Cli {
        Cli::parse()
    }

    fn home() -> String {
        if let Ok(home) = std::env::var("HOME") {
            home
        } else {
            println!("Couldn't find home directory");
            std::process::exit(0x1000);
        }
    }

    fn start(cli: Cli) -> ! {
        Self::match_command(cli);

        loop {}
    }

    fn run_command(
        command: &str,
        args: Vec<&str>,
    ) -> Result<std::process::ExitStatus, std::io::Error> {
        std::process::Command::new(command).args(args).status()
    }

    fn run_editor(filepath: &str) -> Result<std::process::ExitStatus, std::io::Error> {
        if let Ok(path) = Path::new(&filepath).canonicalize() {
            std::process::Command::new(Self::editor().clone())
                .args(vec![path])
                .status()
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Filename was invalid '{}'", filepath).as_str(),
            ))
        }
    }

    fn config_open() {
        // Open in editor
        match Self::run_editor(Self::config_dir().as_str()) {
            Ok(_) => {}
            Err(e) => {
                println!("Editor failed to run: {e}");
                std::process::exit(0x1000);
            }
        };
    }

    fn config_dir() -> String {
        let path_string = format!("{}/{CONF_DIR}", Self::home());
        let path = Path::new(path_string.as_str());

        // Check if config location exists
        match path.clone().canonicalize() {
            Ok(_) => return path_string,
            Err(_) => {
                println!("Couldn't find config directory");
                std::process::exit(0x1000);
            }
        }
    }

    fn config() -> TomlConfig {
        match File::open(Self::config_dir().as_str()) {
            Ok(mut file) => {
                let mut contents = String::new();

                match file.read_to_string(&mut contents) {
                    Ok(_) => match toml::from_str::<TomlConfig>(&contents) {
                        Ok(config) => return config,
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

            Err(e) => {
                println!("Config file not found: {e}");
                std::process::exit(0x1000);
            }
        };
    }

    fn match_command(cli: Cli) {
        match &cli.command {
            Commands::Open { file_name } => match Self::run_editor(file_name) {
                Ok(_) => {}
                Err(e) => {
                    println!("Command 'rust open' failed: {e}");
                    std::process::exit(0x1000);
                }
            },
            Commands::Rust { command } => match &command {
                RustCommands::New { project_name } => {
                    let location_string = format!("{}/Rust/{project_name}", Self::config().dev);

                    if let Err(_) = Path::new(&location_string.clone()).canonicalize() {
                        match Self::run_command(
                            "cargo",
                            vec!["-q", "new", location_string.clone().as_str()],
                        ) {
                            Ok(_) => println!("Project '{project_name}' created successfully"),
                            Err(e) => {
                                println!("Cargo error: {e}");
                                std::process::exit(0x1000);
                            }
                        }

                        match Self::run_editor(
                            format!("{}/src/main.rs", location_string.clone()).as_str(),
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Command 'rust new' failed: {e}");
                                std::process::exit(0x1000);
                            }
                        }
                    } else {
                        println!("Project '{project_name}' already exists");
                        std::process::exit(0x1000);
                    }
                }
            },
            Commands::Config {
                command: command_maybe,
            } => {
                match command_maybe {
                    Some(command) => {
                        match command {
                            ConfigCommands::Open => {
                                Self::config_open();
                            }
                            ConfigCommands::Get => {
                                println!("{:?}", Self::config());
                            } // ConfigCommands::Setup => {
                              //     // TODO fix this

                              //     if canon_path.is_err() {
                              //         Self::run_command(
                              //             "mkdir",
                              //             vec![
                              //                 "-p",
                              //                 format!("{}/.config/noteiser", self.home.clone().unwrap())
                              //                     .as_str(),
                              //             ],
                              //         )
                              //         .unwrap();

                              //         Self::run_command(
                              //             self.editor.clone().as_str(),
                              //             vec![config_path_str.as_str()],
                              //         )
                              //         .unwrap();
                              //     } else {
                              //         println!("Config already created");
                              //         std::process::exit(0x1000);
                              //     }
                              // }
                        }
                    }
                    None => Self::config_open(),
                }
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    println!("{} {}", Noteiser::home(), Noteiser::config_dir());

    Noteiser::start(cli);
}
