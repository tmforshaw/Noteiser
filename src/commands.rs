use clap::Subcommand;

use crate::config;
use crate::error;
use crate::latex;
use crate::note;
use crate::rust;
use crate::{get_editor, verify_filename, Cli};

#[derive(Subcommand)]
pub enum Commands {
    /// Edit config
    Config {
        #[clap(subcommand)]
        command: Option<config::Commands>,
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
        command: rust::Commands,
    },
    /// Opening and displaying of latex files
    Latex {
        #[clap(subcommand)]
        command: latex::Commands,
    },
    /// Note taking functions
    Note {
        #[clap(subcommand)]
        command: note::Commands,
    },
}

pub fn run_command(command: &str, args: &Vec<&str>) {
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
        Some(name) => run_command(get_editor().as_str(), &vec![name]),
        None => error!("Editor could not find file '{filepath}'"),
    }
}

pub fn match_command(cli: &Cli) {
    match &cli.command {
        Commands::Open { file_name } => run_editor(file_name.as_str()),
        Commands::Rust { command } => rust::parse_command(command),
        Commands::Config { command } => config::parse_command(command),
        Commands::Latex { command } => latex::parse_command(command),
        Commands::Note { command } => note::parse_command(command),
    }
}

pub fn error_fn(message: &str) -> ! {
    println!("{message}");
    std::process::exit(0x1000);
}

#[macro_export]
macro_rules! error {
        ($($arg:tt)*) => {
        $crate::commands::error_fn(std::format!("{}", std::format_args!($($arg)*)).as_str())
    };
}

#[must_use]
pub fn confirm_fn(message: &str) -> bool {
    println!("Are you sure you want to {message}? [y/N]");

    // Get user input
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => matches!(input.to_string().trim().to_lowercase().as_str(), "y"),
        Err(e) => error!("Could not parse input: {e}"),
    }
}

#[macro_export]
macro_rules! confirm{
        ($($arg:tt)*) => {
        $crate::commands::confirm_fn(std::format!("{}", std::format_args!($($arg)*)).as_str())
    };
}
