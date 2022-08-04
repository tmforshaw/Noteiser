use clap::Subcommand;
use std::path::Path;

use crate::{
    commands::{run_command, run_editor},
    config::config,
    error,
    getters::{list_files, verify_filename},
};

#[derive(Subcommand)]
pub enum NoteCommands {
    /// Create a new note
    New {
        #[clap(value_parser)]
        file_name: String,
    },
    /// Open a note
    Open {
        #[clap(value_parser)]
        file_name: String,
    },
    /// List files in notes directory
    List,
}

fn check_extension(filename: &String) -> String {
    let path = Path::new(filename);

    match path.extension() {
        Some(_) => filename.clone(),
        None => format!("{filename}.txt"),
    }
}

fn note_new(filename: &String) {
    match config() {
        Ok(config) => {
            let dir_path = match config.note {
                Some(path) => path,
                None => error!("No notes directory set"),
            };

            // Directory exists
            match verify_filename(dir_path.as_str()) {
                Some(path) => {
                    let checked_filename = check_extension(filename);

                    let full_path = format!("{path}/{checked_filename}");

                    match verify_filename(full_path.as_str()) {
                        Some(_) => error!("Note '{checked_filename}' already exists"),
                        None => {
                            run_command("touch", &vec![full_path.as_str()]);

                            run_editor(full_path.as_str());
                        }
                    }
                }
                None => error!("Note directory '{dir_path}' does not exist"),
            }
        }
        Err(e) => error!("{e}"),
    }
}

fn note_open(filename: &String) {
    match config() {
        Ok(config) => {
            let dir_path = match config.note {
                Some(path) => path,
                None => error!("No notes directory set"),
            };

            // Directory exists
            match verify_filename(dir_path.as_str()) {
                Some(path) => {
                    let checked_filename = check_extension(filename);

                    let full_path = format!("{path}/{checked_filename}");

                    match verify_filename(full_path.as_str()) {
                        Some(path) => run_editor(path),
                        None => error!("Note '{checked_filename}' does not exist"),
                    }
                }
                None => error!("Note directory '{dir_path}' does not exist"),
            }
        }
        Err(e) => error!("{e}"),
    }
}

fn note_list() {
    match config() {
        Ok(config) => {
            let notes_dir = match config.note {
                Some(path) => path,
                None => error!("No notes directory set"),
            };

            println!("{}", list_files(notes_dir));
        }
        Err(e) => error!("{e}"),
    }
}

pub fn match_note(command: &NoteCommands) {
    match command {
        NoteCommands::New { file_name } => note_new(file_name),
        NoteCommands::Open { file_name } => note_open(file_name),
        NoteCommands::List => note_list(),
    };
}
