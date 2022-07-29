pub mod commands;
pub mod config;
pub mod getters;
pub mod rust;

use commands::match_command;
use getters::cli;

fn main() {
    let cli = cli();

    match_command(cli);
}
