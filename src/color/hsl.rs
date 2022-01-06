use anyhow::Result;
use regex::Regex;

use crate::error::Error;

use super::utils::color_string_splitter;

#[derive(Debug)]
pub struct HSLColor {
    h: u16,
    s: u8,
    l: u8,
}

pub fn is_hsl_string(color: &String) -> bool {
    match HSLColor::from(color) {
        Ok(_) => true,
        Err(_) => false,
    }
}

impl HSLColor {
    pub fn from(code: &String) -> Result<Self, Error> {
        let mut hsl = vec![0; 3];
        let splitter = color_string_splitter(code);
        if splitter.len() == 3 {
            let pattern = Regex::new(r"^(\d+)").unwrap();
            // hue
            let hue_str = splitter[0].trim();
            if !pattern.is_match(hue_str) {
                return Err(Error::new("Can't parse HSL string"));
            }

            let matched = pattern.find(hue_str).unwrap().as_str();
            let hue = matched.parse::<u16>();
            if let Ok(hue) = hue {
                if hue <= 360 {
                    hsl[0] = hue
                } else {
                    return Err(Error::new("Can't parse HSL string"));
                }
            } else {
                return Err(Error::new("Can't parse HSL string"));
            }
            // saturation & lightness
            for i in 1..2 {
                let mut p_str = splitter[i].trim();
                if p_str.ends_with("%") {
                    p_str = &p_str[0..p_str.len() - 1];
                }
                let p = p_str.parse::<u16>();
                if let Ok(p) = p {
                    if p <= 100 {
                        hsl[i] = p
                    } else {
                        return Err(Error::new("Can't parse HSL string"));
                    }
                } else {
                    return Err(Error::new("Can't parse HSL string"));
                }
            }
            return Ok(Self {
                h: hsl[0],
                s: hsl[1] as u8,
                l: hsl[2] as u8,
            });
        }
        return Err(Error::new("Can't parse HSL string"));
    }

    pub fn print_out(&self) {
        println!("HSL: {}°, {}%, {}%", &self.h, &self.s, &self.l)
    }
}
