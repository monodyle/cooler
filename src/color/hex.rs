use anyhow::Result;
use regex::Regex;

use crate::error::Error;

use super::{hsl::HSLColor, rgb::RGBColor};
use hex::FromHex;

#[derive(Debug)]
pub struct HexColor(String);

pub fn is_hex_string(color: &String) -> bool {
    if let Ok(_) = HexColor::from(color) {
        return true;
    }
    false
}

impl HexColor {
    pub fn from(color: &String) -> Result<Self, Error> {
        let mut color = color.trim().to_string();
        if color.starts_with("#") {
            color = String::from(&color[1..]);
        }

        if color.len() == 6 {
            let pattern = Regex::new(r"^[a-zA-Z0-9]{6}$").unwrap();
            if pattern.is_match(&color) {
                return Ok(Self(color.to_string()));
            }
        }
        if color.len() == 3 {
            let pattern = Regex::new(r"^[a-zA-Z0-9]{3}$").unwrap();
            if pattern.is_match(&color) {
                let chars = &color
                    .chars()
                    .map(|c| c.to_string().repeat(2))
                    .collect::<Vec<String>>()
                    .concat();
                return Ok(Self(chars.to_string()));
            }
        }
        Err(Error::new("Can't parse Hex string"))
    }

    pub fn print(&self) {
        println!("Hex: #{}", self.0);
    }

    pub fn to_rgb(&self) -> RGBColor {
        let decode_hex = <[u8; 3]>::from_hex(&self.0).unwrap();
        RGBColor {
            r: decode_hex[0],
            g: decode_hex[1],
            b: decode_hex[2],
        }
    }

    pub fn to_hsl(&self) -> HSLColor {
        self.to_rgb().to_hsl()
    }

    pub fn print_others(&self) {
        self.print();
        self.to_rgb().print();
        self.to_hsl().print();
    }
}
