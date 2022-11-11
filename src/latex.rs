use std::fs::remove_dir_all;
use std::path::Path;

use clap::Subcommand;

use crate::commands::{run_command, run_editor};
use crate::config;
use crate::{confirm, error, verify_filename};

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new latex file
    New {
        /// Name of file to create
        #[clap(value_parser)]
        file_name: String,
    },
    /// Open a previous latex file using shortened-notation
    Open {
        /// Latex file to open
        #[clap(value_parser)]
        file_name: String,
    },
    // /// List files in project dir (or leave blank to list projects in latex directory)
    // List {
    //     /// Show all files
    //     #[clap(short, long)]
    //     all: bool,

    //     /// Project to list files of
    //     project_name: Option<String>,
    // },
    /// Remove a latex file
    Rm {
        /// Name of file to remove
        #[clap(value_parser)]
        project_name: String,
    },
}

fn latex_new(file_name: &String) {
    match config::get() {
        Ok(config) => {
            let filename = format!("{}/Rust/{file_name}", config.dev);

            match verify_filename(&filename) {
                Some(_) => error!("File '{file_name}' already exists"),
                None => {
                    run_command("cargo", &vec!["-q", "new", filename.as_str()]);

                    println!("File '{file_name}' created successfully");

                    run_editor(format!("{filename}/src/main.rs").as_str());
                }
            }
        }
        Err(e) => error!("{e}"),
    }
}

fn latex_open(file_name: &String) {
    match config::get() {
        Ok(config) => {
            let path_name = format!(
                "{file_name}{}",
                match Path::new(file_name).extension() {
                    Some(_) => "",
                    None => ".tex",
                }
            );

            let full_filename = format!("{}/{}", config.doc, path_name);

            match verify_filename(&full_filename) {
                Some(name) => {
                    run_editor(name);
                    run_latex_preview(name);
                }
                None => error!("Latex file not found: '{full_filename}'"),
            }
        }
        Err(e) => error!("{e}"),
    }
}

fn latex_remove(file_name: &String) {
    match config::get() {
        Ok(config) => {
            let filename = format!("{}/{file_name}", config.doc);

            match verify_filename(&filename) {
                Some(name) => {
                    if confirm!("remove {file_name}") {
                        match remove_dir_all(name) {
                            Ok(_) => println!("Successfully deleted {name}"),
                            Err(e) => error!("Could not remove latex file '{name}': {e}"),
                        }
                    } else {
                        // User denies confirmation
                        std::process::exit(0x1001);
                    }
                }
                None => error!("Project not found: '{filename}'"),
            }
        }
        Err(e) => error!("{e}"),
    }
}

pub fn parse_command(command: &Commands) {
    match &command {
        Commands::New { file_name } => latex_new(file_name),
        Commands::Open { file_name } => latex_open(file_name),
        Commands::Rm { project_name } => latex_remove(project_name),
    }
}
