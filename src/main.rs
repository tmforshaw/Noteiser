use clap::{Parser, Subcommand};

// Most linux distros ship with vi
const EDIT_BIN: &str = "/usr/bin/vi";
const CONF_DIR: &str = ".config/noteiser/config.toml";

// TODO get this from a config file
const DEV_DIR: &str = "/home/tmforshaw/Development";

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true )]
struct Cli {
    /// Specify a specific editor
    #[clap(short, long, value_parser)]
    editor: Option<String>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Open a file in your editor
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
    /// Edit config
    Config,
}

#[derive(Subcommand)]
enum RustCommands {
    /// Create a new rust project
    New {
        /// Name of project to create
        #[clap(value_parser)]
        project_name: String,
    },
}

fn run_command(command: &str, args: Vec<&str>) -> Result<std::process::ExitStatus, std::io::Error> {
    std::process::Command::new(command).args(args).status()
}

fn main() {
    let cli = Cli::parse();

    let editor = if let Some(com_editor) = cli.editor {
        com_editor
    } else if let Ok(env_editor) = std::env::var("EDITOR") {
        env_editor
    } else {
        EDITOR_CONST.to_string()
    };

    match &cli.command {
        Commands::Open { file_name } => {
            run_command(editor.as_str(), vec![file_name]).unwrap();
        }
        Commands::Rust { command } => match &command {
            RustCommands::New { project_name } => {
                let location = format!("{DEV_DIR}/Rust/{project_name}");

                // Check if project already exists
                if run_command("ls", vec![location.as_str()]).is_err() {
                    run_command("cargo", vec!["new", location.clone().as_str()]).unwrap();

                    run_command(
                        editor.as_str(),
                        vec![format!("{location}/src/main.rs").as_str()],
                    )
                    .unwrap();
                } else {
                    println!("Project already exists");
                    std::process::exit(0x1000);
                }
            }
        },
        Commands::Config => {
            // Open the config file
        }
    }
}
