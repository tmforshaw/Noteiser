use clap::Subcommand;

use crate::config::{match_config, ConfigCommands};
use crate::error;
use crate::getters::{editor, verify_filename, Cli};
use crate::rust::{match_rust, RustCommands};

#[derive(Subcommand)]
pub enum Commands {
    /// Edit config
    Config {
        #[clap(subcommand)]
        command: Option<ConfigCommands>,
    },
    /// Open a file in your editor using non shortned-notation
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
        Err(e) => error!(
            "Could not run command: '{command}' with args: {:?}\n Error: {e}",
            args
        ),
    }
}

pub fn run_editor(filepath: &str) {
    match verify_filename(filepath) {
        Some(name) => run_command(editor().clone().as_str(), vec![name]),
        None => error!("Editor could not find file '{filepath}'"),
    }
}

pub fn match_command(cli: Cli) {
    match &cli.command {
        Commands::Open { file_name } => run_editor(file_name.as_str()),
        Commands::Rust { command } => match_rust(command),
        Commands::Config { command } => match_config(command),
    }
}

#[macro_export]
macro_rules! error {
        ($($arg:tt)*) => {
        $crate::commands::error_func(std::format!("{}", std::format_args!($($arg)*)).as_str())
    };
}

pub fn error_func<'a>(message: &'a str) -> ! {
    println!("{message}");
    std::process::exit(0x1000);
}
