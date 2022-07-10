const EDITOR_CONST: &str = "/usr/bin/helix";
const DEV_DIR: &str = "/home/tmforshaw/Development/";

fn run_command(editor: &str, args: Vec<&str>) -> Result<std::process::ExitStatus, std::io::Error> {
    std::process::Command::new(editor).args(args).status()
}

fn get_args() -> Vec<String> {
    let mut args = std::env::args();
    args.next();

    args.collect()
}

fn process_args(args: Vec<String>) {
    let editor = if let Some(x) = args.iter().enumerate().find(|(_i, x)| x.as_str() == "-e") {
        if x.0 < args.len() - 1 {
            args[x.0 + 1].clone()
        } else {
            println!("Please enter an editor when using the editor command");
            std::process::exit(0x1000);
        }
    } else if let Ok(editor_maybe) = std::env::var("EDITOR") {
        editor_maybe
    } else {
        EDITOR_CONST.to_string()
    };

    for i in 0..args.len() {
        match args[i].as_str() {
            "open" | "-o" => {
                if i < args.len() - 1 {
                    if let Err(x) = run_command(
                        editor.clone().as_str(),
                        vec![format!("{}{}", DEV_DIR, args[i + 1]).as_str()],
                    ) {
                        println!("Error: {}", x);
                        std::process::exit(0x1000);
                    }
                } else {
                    println!("Please enter a file to open");
                    std::process::exit(0x1000);
                }
            }
            _ => {}
        }
    }
}

fn main() {
    process_args(get_args());
}
