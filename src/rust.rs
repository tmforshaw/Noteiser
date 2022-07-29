use std::path::Path;

use clap::Subcommand;

use crate::commands::{run_command, run_editor};
use crate::config::{config, TomlConfig};
use crate::error;
use crate::getters::verify_filename;

// Show project source directory

#[derive(Subcommand)]
pub enum RustCommands {
    /// Create a new rust project
    New {
        /// Name of project to create
        #[clap(value_parser)]
        project_name: String,
    },
    /// Open a previous rust project or file using shortened-notation
    Open {
        /// Name of project to open
        #[clap(value_parser)]
        project_name: String,

        /// Project file to open
        #[clap(value_parser)]
        file_name: Option<String>,
    },
}

fn project_dir(config: &TomlConfig, project_name: String) -> String {
    format!("{}/Rust/{project_name}", config.dev)
}

fn rust_new(project_name: &String) {
    match config() {
        Ok(config) => {
            let filename = project_dir(&config, project_name.clone());

            match verify_filename(&filename.clone()) {
                Some(name) => {
                    run_command("cargo", vec!["-q", "new", name]);

                    println!("Project '{project_name}' created successfully");

                    run_editor(format!("{name}/src/main.rs").as_str());
                }
                None => error!("Project '{project_name}' already exists"),
            }
        }
        Err(e) => error!("Config file not found: {e}"),
    }
}

fn rust_open(project_name: &String, file_name: &Option<String>) {
    match config() {
        Ok(config) => {
            let filename = match file_name {
                Some(file) => {
                    format!(
                        "{project_name}/src/{}{}",
                        file,
                        match Path::new(file).extension() {
                            Some(_) => "",
                            None => ".rs",
                        }
                    )
                }
                None => project_name.clone(),
            };

            let full_filename = project_dir(&config, filename.clone());

            match verify_filename(&full_filename.clone()) {
                Some(name) => run_editor(name),
                None => error!("Project or File not found: '{full_filename}'"),
            }
        }
        Err(e) => error!("Config file not found: {e}"),
    }
}

pub fn match_rust(command: &RustCommands) {
    match &command {
        RustCommands::New { project_name } => rust_new(project_name),
        RustCommands::Open {
            project_name,
            file_name,
        } => rust_open(project_name, file_name),
    }
}
