use anyhow::Result;

use crate::{color::hex::{is_hex_string, HexColor}, error::Error};

pub fn check_command(color: &String) -> Result<(), Error> {
    if is_hex_string(color) {
        HexColor::from(color)?.output()
    }
    println!("not match");
    Ok(())
}
