use crate::error::Error;

use super::utils::color_string_splitter;

pub fn is_rgb_string(color: &String) -> bool {
    match RGBColor::from(color) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[derive(Debug)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGBColor {
    pub fn from(color: &String) -> Result<Self, Error> {
        let mut color = color.trim().to_string();
        if color.starts_with("rgb(") && color.ends_with(")") {
            color = String::from(&color[4..color.len()-1]);
        }

        let mut rgb = vec![0; 3];
        let splitter = color_string_splitter(&color);
        if splitter.len() == 3 {
            for (i, value) in splitter.into_iter().enumerate() {
                let parse = value.trim().parse::<u8>();
                match parse {
                    Ok(parse) => rgb[i] = parse,
                    Err(_) => return Err(Error::new("Can't parse RGB string")),
                }
            }
            return Ok(Self {
                r: rgb[0],
                g: rgb[1],
                b: rgb[2],
            });
        }
        return Err(Error::new("Can't parse RGB string"));
    }

    pub fn print_out(&self) {
        println!("RGB: {}, {}, {}", &self.r, &self.g, &self.b)
    }
}
