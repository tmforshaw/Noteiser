use clap::Subcommand;

use crate::config::{match_config, ConfigCommands};
use crate::getters::{editor, verify_filename, Cli};
use crate::rust::{match_rust, RustCommands};

#[derive(Subcommand)]
pub enum Commands {
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

pub fn run_command(command: &str, args: Vec<&str>) {
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

pub fn run_editor(filepath: &str) {
    match verify_filename(filepath) {
        Some(name) => run_command(editor().clone().as_str(), vec![name]),
        None => {
            println!("Editor could not find file '{filepath}'");
            std::process::exit(0x1000);
        }
    }
}

pub fn match_command(cli: Cli) {
    match &cli.command {
        Commands::Open { file_name } => {
            run_editor(file_name.as_str());
            std::process::exit(0x1000);
        }
        Commands::Rust { command } => match_rust(command),
        Commands::Config { command } => match_config(command),
    }
}
