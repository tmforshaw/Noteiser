#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]


pub mod commands;
pub mod config;
pub mod getters;
pub mod note;
pub mod rust;

use commands::match_command;
use getters::cli;

fn main() {
    let cli = cli();

    match_command(&cli);
}
