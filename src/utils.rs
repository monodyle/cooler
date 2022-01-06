use anyhow::Result;

use crate::{color::{hex::{is_hex_string, HexColor}, rgb::{is_rgb_string, RGBColor}, hsl::{is_hsl_string, HSLColor}, cmyk::{is_cmyk_string, CMYKColor}}, error::Error};

pub fn check_command(color: &String) -> Result<(), Error> {
    if is_hex_string(color) {
        HexColor::from(color)?.print_others()
    } else if is_rgb_string(color) {
        RGBColor::from(color)?.print_others()
    } else if is_hsl_string(color) {
        HSLColor::from(color)?.print_others()
    } else if is_cmyk_string(color) {
        CMYKColor::from(color)?.print_others()
    } else {
        println!("Invalid color string");
    }
    Ok(())
}
