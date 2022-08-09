use std::fs::remove_dir_all;
use std::path::Path;

use clap::Subcommand;

use crate::commands::{run_command, run_editor};
use crate::config;
use crate::{confirm, error, get_files, get_matching_files, verify_filename};

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
    /// List files in project dir (or leave blank to list projects in rust directory)
    List {
        /// Show all files
        #[clap(short, long)]
        all: bool,

        /// Project to list files of
        project_name: Option<String>,
    },
    /// Remove a rust project
    Rm {
        /// Name of project to remove
        #[clap(value_parser)]
        project_name: String,
    },
}

fn rust_new(project_name: &String) {
    match config::get() {
        Ok(config) => {
            let filename = format!("{}/Rust/{project_name}", config.dev);

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

            let full_filename = format!("{}/Rust/{}", config.dev, path_name);

            match verify_filename(&full_filename) {
                Some(name) => run_editor(name),
                None => error!("Project or File not found: '{full_filename}'"),
            }
        }
        Err(e) => error!("{e}"),
    }
}

fn rust_list(project_name: &Option<String>, show_all: bool) {
    match config::get() {
        Ok(config) => {
            match project_name {
                Some(name) => {
                    let dev_dir = format!("{}/Rust/{}", config.dev, &name);

                    match verify_filename(&dev_dir) {
                        Some(directory) => {
                            // Remove hidden files, 'target', and 'src' (if show_all doesn't exist)
                            let root_files = if show_all {
                                get_matching_files(&directory.to_string(), ".*")
                            } else {
                                get_matching_files(&directory.to_string(), r"^[a-zA-Z]")
                                    .iter()
                                    .filter_map(|f| {
                                        if f.as_str().eq("target") || f.as_str().eq("src") {
                                            None
                                        } else {
                                            Some(f.clone())
                                        }
                                    })
                                    .collect::<Vec<String>>()
                            };

                            println!("{name}/");

                            for file in root_files {
                                println!("\t{file}");
                            }

                            let src_files = get_files(&format!("{directory}/src"));

                            println!("\n{name}/src/");

                            for file in src_files {
                                println!("\t{file}");
                            }
                        }
                        None => error!("Development directory not found: '{dev_dir}'"),
                    }
                }
                None => {
                    let directory = format!("{}/Rust", config.dev);

                    let root_files = get_matching_files(&directory, r"^[a-zA-Z]")
                        .iter()
                        .filter_map(|f| {
                            if f.as_str().eq("Scripts") {
                                None
                            } else {
                                Some(f.clone())
                            }
                        })
                        .collect::<Vec<String>>();

                    println!("Rust/");

                    for file in root_files {
                        println!("\t{file}");
                    }

                    let script_files = get_matching_files(
                        &format!("{directory}/Scripts"),
                        if show_all { ".*" } else { r"^[a-zA-Z]" },
                    );

                    println!("\nRust/Scripts");

                    for file in script_files {
                        println!("\t{file}");
                    }
                }
            }
        }
        Err(e) => error!("{e}"),
    };
}

fn rust_remove(project_name: &String) {
    match config::get() {
        Ok(config) => {
            let filename = format!("{}/Rust/{project_name}", config.dev);

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
        Commands::List { project_name, all } => rust_list(project_name, *all),
        Commands::Rm { project_name } => rust_remove(project_name),
    }
}
