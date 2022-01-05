use anyhow::Result;
use colored::Colorize;
use regex::Regex;

use crate::error::Error;

#[derive(Debug)]
pub struct HexColor(String);

pub fn is_hex_string(color: &String) -> bool {
    let pattern = Regex::new(r"^[a-zA-Z0-9]{6}$").unwrap();
    if color.len() == 6 {
        return pattern.is_match(color);
    }
    if color.len() == 7 {
        if color.starts_with("#") {
            let color = &color[1..];
            return pattern.is_match(color);
        }
        return false;
    }
    false
}

impl HexColor {
    pub fn from(code: &String) -> Result<Self, Error> {
        if is_hex_string(&code) {
            Ok(Self(code.to_owned()))
        } else {
            Err(Error::new("Hello"))
        }
    }

    pub fn print_out(&self) {
        println!("Hex: {}", self.0.bold())
    }
}
