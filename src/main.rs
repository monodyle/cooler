mod color;
mod commands;
mod error;

fn main() {
    let color = std::env::args().nth(1).unwrap_or("help".to_string());
    match color.as_ref() {
        "help" => commands::help::run(),
        _ => commands::color_string::run(&color),
    }
}
