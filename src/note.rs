use clap::Subcommand;
use std::path::Path;

use crate::{
    commands::run_editor,
    config, error, file, {get_files, verify_file_and_dir},
};

#[derive(Subcommand)]
pub enum Commands {
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
    match config::get() {
        Ok(config) => {
            let dir_path = match config.note {
                Some(path) => path,
                None => error!("No notes directory set"),
            };

            let checked_filename = check_extension(filename);
            let full_path = format!("{dir_path}/{checked_filename}");

            if let Ok(path) = verify_file_and_dir(checked_filename.as_str(), dir_path.as_str()) {
                println!("Note '{path}' already exists...\nOpening note");

                run_editor(&full_path);
            } else {
                file::create(&full_path);

                run_editor(full_path.as_str());
            }
        }
        Err(e) => error!("{e}"),
    }
}

fn note_open(filename: &String) {
    match config::get() {
        Ok(config) => {
            let dir_path = match config.note {
                Some(path) => path,
                None => error!("No notes directory set"),
            };

            let checked_filename = check_extension(filename);

            match verify_file_and_dir(checked_filename.as_str(), dir_path.as_str()) {
                Ok(path) => run_editor(&path),
                Err(e) => error!("Note error: {e}"),
            }
        }
        Err(e) => error!("{e}"),
    }
}

fn note_list() {
    match config::get() {
        Ok(config) => {
            let notes_dir = match config.note {
                Some(path) => path,
                None => error!("No notes directory set"),
            };

            let files = get_files(&notes_dir);

            let shortened_notes_dir = match Path::new(&notes_dir).file_name() {
                Some(dir) => match dir.to_str() {
                    Some(string) => string,
                    None => error!("Could not parse notes directory name into string"),
                },
                None => error!("Could not get name of notes directory"),
            };

            println!("{shortened_notes_dir}/");

            for file in files {
                println!("\t{file}");
            }
        }
        Err(e) => error!("{e}"),
    }
}

pub fn parse_command(command: &Commands) {
    match command {
        Commands::New { file_name } => note_new(file_name),
        Commands::Open { file_name } => note_open(file_name),
        Commands::List => note_list(),
    };
}
