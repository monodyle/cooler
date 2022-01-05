use anyhow::Result;

use crate::{color::{hex::{is_hex_string, HexColor}, rgb::{is_rgb_string, RGBColor}}, error::Error};

pub fn check_command(color: &String) -> Result<(), Error> {
    if is_hex_string(color) {
        HexColor::from(color)?.print_out()
    } else if is_rgb_string(color) {
        RGBColor::from(color)?.print_out()
    } else {
        println!("Invalid color string");
    }
    Ok(())
}
