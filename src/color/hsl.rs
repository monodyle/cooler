use anyhow::Result;
use regex::Regex;

use crate::error::Error;

use super::utils::color_string_splitter;

#[derive(Debug)]
pub struct HSLColor {
    pub h: u16,
    pub s: f64,
    pub l: f64,
}

pub fn is_hsl_string(color: &String) -> bool {
    match HSLColor::from(color) {
        Ok(_) => true,
        Err(_) => false,
    }
}

impl HSLColor {
    pub fn from(color: &String) -> Result<Self, Error> {
        let mut color = color.trim().to_string();
        if color.starts_with("hsl(") && color.ends_with(")") {
            color = String::from(&color[4..color.len() - 1]);
        }

        let splitter = color_string_splitter(&color);
        if splitter.len() == 3 {
            let (mut h, mut s, mut l): (u16, f64, f64) = (0, 0.0, 0.0);
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
                    h = hue
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
                let p = p_str.parse::<f64>();
                if let Ok(p) = p {
                    if p <= 100.0 {
                        if i == 1 {
                            s = p
                        } else {
                            l = p
                        }
                    } else {
                        return Err(Error::new("Can't parse HSL string"));
                    }
                } else {
                    return Err(Error::new("Can't parse HSL string"));
                }
            }
            return Ok(Self { h, s, l });
        }
        return Err(Error::new("Can't parse HSL string"));
    }

    pub fn print(&self) {
        println!("HSL: {}°, {:.0}%, {:.0}%", &self.h, &self.s, &self.l)
    }

    pub fn print_others(&self) {
        self.print();
    }
}
