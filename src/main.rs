use utils::check_command;

mod color;
mod commands;
mod error;
mod utils;

fn main() {
    let color = std::env::args().nth(1).unwrap_or("help".to_string());
    match color.as_ref() {
        "help" => commands::help::run(),
        _ => check_command(&color).unwrap_or_default(),
    }
}
