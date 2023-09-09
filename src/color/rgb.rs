use lazy_static::lazy_static;
use regex::Regex;

use crate::{color::utils, error::Error};

use super::{hsl::HSLColor, utils::safe_value};

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
        r"^(?i:rgba?)\(\s*(?P<r>\d+)\s*,\s*(?P<g>\d+)\s*,\s*(?P<b>\d+)\s*(?:,\s*(?P<a>[\d.]+))?\s*\)$"
    )
    .unwrap();
    static ref HSL_RE: Regex = Regex::new(
        r"^(?i:hsla?)\(\s*(?P<h>[\d.]+)\s*,\s*(?P<s>[\d.]+)%\s*,\s*(?P<l>[\d.]+)%(?:\s*,\s*(?P<a>[\d.]+))?\s*\)$"
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

    pub fn to_hsl(&self) -> HSLColor {
        let (r, g, b) = (
            self.r as f64 / 255.0,
            self.g as f64 / 255.0,
            self.b as f64 / 255.0,
        );
        let cmin = f64::min(f64::min(r, g), b);
        let cmax = f64::max(f64::max(r, g), b);
        let delta = cmax - cmin;

        let (mut h, mut s, mut l): (f64, f64, f64);

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

        s = (s * 100.0).ceil();
        l = (l * 100.0).ceil();

        HSLColor { h: h as u16, s: s as u8, l: l as u8, a: self.a }
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

        if HSL_RE.is_match(&color) {
            let Some(values) = HSL_RE.captures(&color) else {
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

            let h = values["h"]
                .parse::<u16>()
                .or(Err(Error::new("Invalid color string")))?;
            let s = values["s"]
                .parse::<u8>()
                .or(Err(Error::new("Invalid color string")))?;
            let l = values["l"]
                .parse::<u8>()
                .or(Err(Error::new("Invalid color string")))?;

            if safe_value(0, 100, s) != s || safe_value(0, 100, l) != l {
                return Err(Error::new("Invalid color string"));
            }

            let hsl = HSLColor::new(h, s, l, Some(alpha));
            return Ok(hsl.to_rgb());
        }

        Err(Error::new("Invalid color string"))
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.a = safe_value(0.0, 1.0, alpha);
    }

    pub fn is_rgb_string(color: &String) -> bool {
        match Self::parse(color) {
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

    #[test]
    fn hsl_string() {
        assert_eq!(RGBColor::parse(&String::from("hsl(136, 32%, 62%)")).unwrap(),
        RGBColor {
            r: 127,
            g: 189,
            b: 144,
            a: 1.0
        })
    }
}
