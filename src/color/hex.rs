use anyhow::Result;
use colored::Colorize;
use regex::Regex;

use crate::error::Error;

use super::rgb::RGBColor;
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
        let pattern = Regex::new(r"^[a-zA-Z0-9]{6}$").unwrap();
        if color.len() == 6 {
            if pattern.is_match(color) {
                return Ok(Self(color.to_string()));
            }
            return Err(Error::new("Can't parse Hex string"));
        }
        if color.len() == 7 {
            if color.starts_with("#") {
                let color = &color[1..];
                if pattern.is_match(color) {
                    return Ok(Self(color.to_string()));
                } else {
                    return Err(Error::new("Can't parse Hex string"));
                }
            }
            return Err(Error::new("Can't parse Hex string"));
        }
        Err(Error::new("Can't parse Hex string"))
    }

    pub fn print_out(&self) {
        println!("Hex: #{}", self.0.bold());
        self.to_rgb().print_out()
    }

    pub fn to_rgb(&self) -> RGBColor {
        let decode_hex = <[u8; 3]>::from_hex(&self.0).unwrap();
        RGBColor {
            r: decode_hex[0],
            g: decode_hex[1],
            b: decode_hex[2],
        }
    }
}
