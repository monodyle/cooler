use crate::error::Error;

use super::{hsl::HSLColor, utils::color_string_splitter, hex::HexColor};

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
            color = String::from(&color[4..color.len() - 1]);
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

    pub fn print(&self) {
        println!("RGB: {}, {}, {}", &self.r, &self.g, &self.b);
    }

    pub fn to_hsl(&self) -> HSLColor {
        let (r, g, b) = (
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
        );
        let cmin = f64::min(f64::min(r, g), b);
        let cmax = f64::max(f64::max(r, g), b);
        let delta = cmax - cmin;

        let (mut h, mut s, mut l): (f64, f64, f64) = (0.0, 0.0, 0.0);

        if delta == 0.0 {
            h = 0.0;
        } else if cmax == r {
            h = ((g - b) as f64 / delta as f64) % 6.0;
        } else if cmax == g {
            h = (b - r) as f64 / delta as f64 + 2.0;
        } else {
            h = (r - g) as f64 / delta as f64 + 4.0;
        }
        h = (h * 60.0).ceil();
        if h < 0.0 {
            h += 360.0
        }

        l = (cmax + cmin) as f64 / 2.0;
        s = if delta == 0.0 {
            0.0
        } else {
            delta as f64 / (1.0 - f64::abs(2.0 * l - 1.0))
        };

        s = s * 100.0;
        l = l * 100.0;

        HSLColor { h: h as u16, s, l }
    }

    pub fn to_hex(&self) -> HexColor {
        let hex = hex::encode(vec![self.r, self.g, self.b]);
        HexColor::from(&hex).unwrap()
    }

    pub fn print_others(&self) {
        self.print();
        self.to_hsl().print();
        self.to_hex().print();
    }
}
