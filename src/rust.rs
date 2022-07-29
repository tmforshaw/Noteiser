use clap::Subcommand;

use crate::commands::{run_command, run_editor};
use crate::config::config;
use crate::getters::verify_filename;

#[derive(Subcommand)]
pub enum RustCommands {
    /// Create a new rust project
    New {
        /// Name of project to create
        #[clap(value_parser)]
        project_name: String,
    },
}

pub fn match_rust(command: &RustCommands) {
    match &command {
        RustCommands::New { project_name } => match config() {
            Ok(config) => {
                let filename_string = format!("{}/Rust/{project_name}", config.dev);

                match verify_filename(&filename_string.clone()) {
                    Some(name) => {
                        run_command("cargo", vec!["-q", "new", name]);

                        println!("Project '{project_name}' created successfully");

                        run_editor(format!("{name}/src/main.rs").as_str());
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
    }
}
