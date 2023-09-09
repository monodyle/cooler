use lazy_static::lazy_static;
use regex::Regex;

use crate::{color::utils, error::Error};

use super::utils::safe_value;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RGBColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

lazy_static! {
    static ref REDUCED_HEX_RE: Regex =
        Regex::new("^#(?P<r>[a-f0-9])(?P<g>[a-f0-9])(?P<b>[a-f0-9])(?P<a>[a-f0-9])?$").unwrap();
    static ref HEX_RE: Regex =
        Regex::new("^#(?P<r>[a-f0-9]{2})(?P<g>[a-f0-9]{2})(?P<b>[a-f0-9]{2})(?P<a>[a-f0-9]{2})?$")
            .unwrap();
    static ref RGB_RE: Regex = Regex::new(
        r"^rgba?\(\s*(?P<r>\d+)\s*,\s*(?P<g>\d+)\s*,\s*(?P<b>\d+)\s*(?:,\s*(?P<a>[\d.]+))?\s*\)$"
    )
    .unwrap();
}

impl RGBColor {
    pub fn new(r: u8, g: u8, b: u8, a: Option<f32>) -> Self {
        Self {
            r: safe_value(0, 255, r),
            g: safe_value(0, 255, g),
            b: safe_value(0, 255, b),
            a: safe_value(0.0, 1.0, a.unwrap_or(1.0)),
        }
    }

    pub fn parse(color: &String) -> Result<Self, Error> {
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
                a: (utils::hex_to_u8(alpha_hex.to_owned())? as f32) / 255.0,
            });
        }

        if HEX_RE.is_match(&color) {
            let Some(values) = HEX_RE.captures(&color) else {
                return Err(Error::new("Invalid color string"))
            };
            let extracted_alpha = values.name("a");
            let alpha_hex = if extracted_alpha.is_some() {
                &values["a"]
            } else {
                "ff"
            };
            return Ok(Self {
                r: utils::hex_to_u8(String::from(&values["r"]))?,
                g: utils::hex_to_u8(String::from(&values["g"]))?,
                b: utils::hex_to_u8(String::from(&values["b"]))?,
                a: (utils::hex_to_u8(alpha_hex.to_owned())? as f32) / 255.0,
            });
        }

        if RGB_RE.is_match(&color) {
            let Some(values) = RGB_RE.captures(&color) else {
                return Err(Error::new("Invalid color string"))
            };
            let extracted_alpha = values.name("a");
            let alpha = if extracted_alpha.is_some() {
                values["a"]
                    .parse::<f32>()
                    .or(Err(Error::new("Invalid color string")))?
            } else {
                1.0
            };

            return Ok(Self {
                r: values["r"]
                    .parse::<u8>()
                    .or(Err(Error::new("Invalid color string")))?,
                g: values["g"]
                    .parse::<u8>()
                    .or(Err(Error::new("Invalid color string")))?,
                b: values["b"]
                    .parse::<u8>()
                    .or(Err(Error::new("Invalid color string")))?,
                a: alpha,
            });
        }

        return Err(Error::new("Invalid color string"));
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.a = safe_value(0.0, 1.0, alpha);
    }

    pub fn is_rgb_string(color: &String) -> bool {
        match RGBColor::parse(color) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn print(&self) {
        println!(
            "RGB Color:\nRed: {}\nGreen: {}\nBlue: {}\nAlpha: {}",
            &self.r, &self.g, &self.b, &self.a
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::color::rgb::RGBColor;

    #[test]
    fn transparent_string() {
        assert_eq!(
            RGBColor::parse(&String::from("transparent")).unwrap(),
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
            RGBColor::parse(&String::from("#fff")).unwrap(),
            RGBColor {
                r: 255,
                g: 255,
                b: 255,
                a: 1.0
            }
        );
    }

    #[test]
    fn reduced_hex_with_alpha_string() {
        assert_eq!(
            RGBColor::parse(&String::from("#fff0")).unwrap(),
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
        let result_1 = RGBColor::parse(&String::from("#zzz")).map_err(|e| e.message);
        assert_eq!(result_1, Err(String::from("Invalid color string")));

        let result_2 = RGBColor::parse(&String::from("#ffag")).map_err(|e| e.message);
        assert_eq!(result_2, Err(String::from("Invalid color string")));
    }

    #[test]
    fn hex_string() {
        assert_eq!(
            RGBColor::parse(&String::from("#000000")).unwrap(),
            RGBColor {
                r: 0,
                g: 0,
                b: 0,
                a: 1.0
            }
        );
    }

    #[test]
    fn hex_with_alpha_string() {
        assert_eq!(
            RGBColor::parse(&String::from("#000000ff")).unwrap(),
            RGBColor {
                r: 0,
                g: 0,
                b: 0,
                a: 1.0
            }
        );
    }

    #[test]
    fn invalid_hex_string() {
        let result_1 = RGBColor::parse(&String::from("#abcdefgh")).map_err(|e| e.message);
        assert_eq!(result_1, Err(String::from("Invalid color string")));

        let result_2 = RGBColor::parse(&String::from("#iklmno")).map_err(|e| e.message);
        assert_eq!(result_2, Err(String::from("Invalid color string")));
    }

    #[test]
    fn rgb_string() {
        assert_eq!(
            RGBColor::parse(&String::from("rgb(0, 0, 0)")).unwrap(),
            RGBColor {
                r: 0,
                g: 0,
                b: 0,
                a: 1.0
            }
        );
    }

    #[test]
    fn rgba_string() {
        assert_eq!(
            RGBColor::parse(&String::from("rgba( 255, 255, 255, 0)")).unwrap(),
            RGBColor {
                r: 255,
                g: 255,
                b: 255,
                a: 0.0
            }
        );
    }
}
