use clap::Subcommand;

use crate::{
    commands::{run_command, run_editor},
    config::config,
    error,
    getters::verify_filename,
};

#[derive(Subcommand)]
pub enum NoteCommands {
    /// Create a new note
    New {
        #[clap(value_parser)]
        file_name: String,
    },
}

// const NOTE_DIR: &str = "/Documents/Notes";

fn note_new(filename: &String) {
    match config() {
        Ok(config) => {
            let unshortened_path = match config.note {
                Some(path) => path,
                None => error!("No notes directory set"),
            };

            // Directory exists
            match verify_filename(unshortened_path.as_str()) {
                Some(path) => {
                    let full_path = format!("{path}{filename}");
                    match verify_filename(full_path.as_str()) {
                        Some(_) => error!("Note '{filename}' already exists"),
                        None => {
                            run_command("touch", &vec![full_path.as_str()]);

                            run_editor(full_path.as_str());
                        }
                    }
                }
                None => error!("Note directory does not exist"),
            }
        }
        Err(e) => error!("{e}"),
    }
}

pub fn match_note(command: &NoteCommands) {
    match command {
        NoteCommands::New { file_name } => note_new(file_name),
    };
}
