use std::path::Path;

use clap::{Parser, Subcommand};

// Most linux distros ship with vi
const EDIT_BIN: &str = "/usr/bin/vi";
// const CONF_DIR: &str = ".config/noteiser/config.toml";

// TODO get this from a config file
const DEV_DIR: &str = "/home/tmforshaw/Development";

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true )]
struct Cli {
    /// Specify a specific editor
    #[clap(short, long, value_parser)]
    editor: Option<String>,

    #[clap(subcommand)]
    command: Commands,
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
    Config,
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

struct Noteiser {
    cli: Cli,
    editor: String,
}

impl Noteiser {
    fn new() -> Self {
        let cli = Cli::parse();

        let editor = if let Some(com_editor) = cli.editor.clone() {
            com_editor
        } else if let Ok(env_editor) = std::env::var("EDITOR") {
            env_editor
        } else {
            EDIT_BIN.to_string()
        };

        Self { cli, editor }
    }

    fn run_command(
        command: &str,
        args: Vec<&str>,
    ) -> Result<std::process::ExitStatus, std::io::Error> {
        std::process::Command::new(command).args(args).status()
    }

    fn run_editor(self: &Self, filepath: &str) -> Result<std::process::ExitStatus, std::io::Error> {
        if let Ok(path) = Path::new(&filepath).canonicalize() {
            std::process::Command::new(self.editor.clone())
                .args(vec![path])
                .status()
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Filename was invalid '{}'", filepath).as_str(),
            ))
        }
    }

    fn match_command(&self) {
        match &self.cli.command {
            Commands::Open { file_name } => match self.run_editor(file_name) {
                Ok(_) => {}
                Err(e) => println!("Command 'open' failed: {e}"),
            },
            Commands::Rust { command } => match &command {
                RustCommands::New { project_name } => {
                    let location_string = format!("{DEV_DIR}/Rust/{project_name}");

                    if let Err(_) = Path::new(&location_string.clone()).canonicalize() {
                        match Self::run_command(
                            "cargo",
                            vec!["-q", "new", location_string.clone().as_str()],
                        ) {
                            Ok(_) => println!("Project '{project_name}' created successfully"),
                            Err(e) => println!("Cargo error: {e}"),
                        }

                        match self
                            .run_editor(format!("{}/src/main.rs", location_string.clone()).as_str())
                        {
                            Ok(_) => {}
                            Err(e) => println!("Command 'new' failed: {e}"),
                        }
                    } else {
                        println!("Project '{project_name}' already exists");
                        std::process::exit(0x1000);
                    }
                }
            },
            Commands::Config => {
                // Open the config file
            }
        }
    }
}

fn main() {
    let n = Noteiser::new();

    n.match_command();
}
