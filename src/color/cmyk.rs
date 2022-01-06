use crate::error::Error;

use super::utils::color_string_splitter;

pub fn is_cmyk_string(color: &String) -> bool {
    match CMYKColor::from(color) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[derive(Debug)]
pub struct CMYKColor {
    pub c: u8,
    pub m: u8,
    pub y: u8,
    pub k: u8,
}

impl CMYKColor {
    pub fn from(color: &String) -> Result<Self, Error> {
        let mut color = color.trim().to_string();
        if color.starts_with("cmyk(") && color.ends_with(")") {
            color = String::from(&color[5..color.len()-1]);
        }

        let mut cmyk = vec![0; 4];
        let splitter = color_string_splitter(&color);
        if splitter.len() == 4 {
            for (i, value) in splitter.into_iter().enumerate() {
                let mut value = value;
                if value.ends_with("%") {
                    value = &value[0..value.len() - 1];
                }
                let parse = value.trim().parse::<u8>();
                match parse {
                    Ok(parse) => {
                        if parse <= 100 {
                            cmyk[i] = parse
                        } else {
                            return Err(Error::new("Can't parse CMYK string"));
                        }
                    }
                    Err(_) => return Err(Error::new("Can't parse CMYK string")),
                }
            }
            return Ok(Self {
                c: cmyk[0],
                m: cmyk[1],
                y: cmyk[2],
                k: cmyk[3],
            });
        }
        return Err(Error::new("Can't parse CMYK string"));
    }

    pub fn print_out(&self) {
        println!(
            "CMYK: {}%, {}%, {}%, {}%",
            &self.c, &self.m, &self.y, &self.k
        )
    }
}
