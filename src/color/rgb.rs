use lazy_static::lazy_static;
use regex::Regex;

use crate::{error::Error, color::utils};

pub fn is_rgb_string(color: &String) -> bool {
    match RGBColor::from(color) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f64,
}

lazy_static! {
    static ref REDUCED_HEX_RE: Regex =
        Regex::new(r"^#(?P<r>[a-f0-9])(?P<g>[a-f0-9])(?P<b>[a-f0-9])(?P<a>[a-f0-9])?$").unwrap();
    static ref HEX_RE: Regex =
        Regex::new(r"^#([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})([a-f0-9]{2})?$").unwrap();
    static ref RGB_RE: Regex =
        Regex::new(r"^(?i:rgb)?\\(\\s*(\\d+)\\s*,\\s*(\\d+)\\s*,\\s*(\\d+)\\s*\\)$").unwrap();
    static ref RGBA_RE: Regex = Regex::new(
        r"^(?i:rgba)?\\(\\s*(\\d+)\\s*,\\s*(\\d+)\\s*,\\s*(\\d+)\\s*(?:,\\s*([\\d.]+))?\\s*\\)$"
    )
    .unwrap();
}

impl RGBColor {
    pub fn from(color: &String) -> Result<Self, Error> {
        let color: String = color.trim().to_string();
        if color == "transparent" {
            return Ok(Self {
                r: 0,
                g: 0,
                b: 0,
                a: 0.0,
            });
        }

        if REDUCED_HEX_RE.is_match(&color) {
            let Some(values) = REDUCED_HEX_RE.captures(&color) else {
                return Err(Error::new("Invalid color string"))
            };

            let extracted_alpha = values.name("a");
            let alpha_hex = if extracted_alpha.is_some() {
                &values["a"]
            } else {
                "f"
            };
            return Ok(Self {
                r: utils::hex_to_u8(String::from(&values["r"]))?,
                g: utils::hex_to_u8(String::from(&values["g"]))?,
                b: utils::hex_to_u8(String::from(&values["b"]))?,
                a: (utils::hex_to_u8(alpha_hex.to_owned())? as f64) / 255.0,
            })
        }

        return Err(Error::new("Invalid color string"));
    }

    pub fn print(&self) {
        println!("RGB: {}, {}, {}", &self.r, &self.g, &self.b);
    }

    pub fn print_others(&self) {
        self.print();
    }
}

#[cfg(test)]
mod tests {
    use crate::color::rgb::RGBColor;

    #[test]
    fn transparent_string() {
        assert_eq!(
            RGBColor::from(&String::from("transparent")).unwrap(),
            RGBColor {
                r: 0,
                g: 0,
                b: 0,
                a: 0.0
            }
        );
    }

    #[test]
    fn reduced_hex_string() {
        assert_eq!(
            RGBColor::from(&String::from("#fff")).unwrap(),
            RGBColor {
                r: 255,
                g: 255,
                b: 255,
                a: 1.0
            }
        );
    }

    #[test]
    fn reduced_hex_walpha_string() {
        assert_eq!(
            RGBColor::from(&String::from("#fff0")).unwrap(),
            RGBColor {
                r: 255,
                g: 255,
                b: 255,
                a: 0.0
            }
        );
    }

    #[test]
    fn invalid_reduced_hex_string() {
        let result_1 = RGBColor::from(&String::from("#zzz")).map_err(|e| e.message);
        assert_eq!(result_1, Err(String::from("Invalid color string")));

        let result_2 = RGBColor::from(&String::from("#ffag")).map_err(|e| e.message);
        assert_eq!(result_2, Err(String::from("Invalid color string")));
    }

    // #[test]
    // fn rgb_white_string() {
    //     assert_eq!(
    //         RGBColor::from(&String::from("rgb(255,255,255)")).unwrap(),
    //         RGBColor {
    //             r: 255,
    //             g: 255,
    //             b: 255,
    //             a: 1
    //         }
    //     );
    // }

    // #[test]
    // fn rgb_black_string() {
    //     assert_eq!(
    //         RGBColor::from(&String::from("rgb(0,0,0)")).unwrap(),
    //         RGBColor {
    //             r: 0,
    //             g: 0,
    //             b: 0,
    //             a: 1
    //         }
    //     );
    // }

    // #[test]
    // fn invalid_rgb_string() {
    //     let result = RGBColor::from(&String::from("rgb(255,266,288)")).map_err(|e| e.message);
    //     assert_eq!(result, Err(String::from("Invalid color string")));
    // }
}
