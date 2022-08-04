use std::fs::remove_dir_all;
use std::path::Path;

use clap::Subcommand;

use crate::commands::{run_command, run_editor};
use crate::config;
use crate::getters::{list_files, verify_filename};
use crate::{confirm, error};

#[derive(Subcommand)]
pub enum Commands {
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
    /// List the projects in your rust directory
    List,
    /// Remove a rust project
    Rm {
        /// Name of project to remove
        #[clap(value_parser)]
        project_name: String,
    },
}

fn project_dir(config: &config::Toml, project_name: &String) -> String {
    format!("{}/Rust/{project_name}", config.dev)
}

fn rust_new(project_name: &String) {
    match config::get() {
        Ok(config) => {
            let filename = project_dir(&config, project_name);

            match verify_filename(&filename) {
                Some(_) => error!("Project '{project_name}' already exists"),
                None => {
                    run_command("cargo", &vec!["-q", "new", filename.as_str()]);

                    println!("Project '{project_name}' created successfully");

                    run_editor(format!("{filename}/src/main.rs").as_str());
                }
            }
        }
        Err(e) => error!("{e}"),
    }
}

fn rust_open(project_name: &String, file_name: &Option<String>) {
    match config::get() {
        Ok(config) => {
            let path_name = match file_name {
                Some(path) => {
                    format!(
                        "{project_name}/src/{}{}",
                        path,
                        match Path::new(path).extension() {
                            Some(_) => "",
                            None => ".rs",
                        }
                    )
                }
                None => project_name.clone(),
            };

            let full_filename = project_dir(&config, &path_name);

            match verify_filename(&full_filename) {
                Some(name) => run_editor(name),
                None => error!("Project or File not found: '{full_filename}'"),
            }
        }
        Err(e) => error!("{e}"),
    }
}

fn rust_list() {
    match config::get() {
        Ok(config) => {
            let dev_dir = project_dir(&config, &String::new());

            match verify_filename(&dev_dir) {
                Some(directory) => println!("{}", list_files(&directory.to_string())),
                None => error!("Development directory not found: '{dev_dir}'"),
            }
        }
        Err(e) => error!("{e}"),
    };
}

fn rust_remove(project_name: &String) {
    match config::get() {
        Ok(config) => {
            let filename = project_dir(&config, project_name);

            match verify_filename(&filename) {
                Some(name) => {
                    if confirm!("remove {project_name}") {
                        match remove_dir_all(name) {
                            Ok(_) => println!("Successfully deleted {name}"),
                            Err(e) => error!("Could not remove project '{name}': {e}"),
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
        Commands::New { project_name } => rust_new(project_name),
        Commands::Open {
            project_name,
            file_name,
        } => rust_open(project_name, file_name),
        Commands::List => rust_list(),
        Commands::Rm { project_name } => rust_remove(project_name),
    }
}
