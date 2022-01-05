use crate::error::Error;

pub fn is_rgb_string(color: &String) -> bool {
    match RGBColor::from(color) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[derive(Debug)]
pub struct RGBColor {
    r: u8,
    g: u8,
    b: u8,
}

impl RGBColor {
    pub fn from(code: &String) -> Result<Self, Error> {
        let mut rgb = vec![0; 3];
        if code.contains(",") {
            let splitter = code.trim().split(",");
            if splitter.clone().count() == 3 {
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
        } else {
            let splitter = code.trim().split_ascii_whitespace();
            if splitter.clone().count() == 3 {
                for (i, value) in splitter.into_iter().enumerate() {
                    let parse = value.parse::<u8>();
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
    }

    pub fn print_out(&self) {
        println!("RGB: ({}, {}, {})", &self.r, &self.g, &self.b)
    }
}
