use crate::error::Error;

use super::{rgb::RGBColor, utils::safe_value};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct HSLColor {
    pub h: u16,
    pub s: u8,
    pub l: u8,
    pub a: f32,
}

impl HSLColor {
    pub fn new(h: u16, s: u8, l: u8, a: Option<f32>) -> Self {
        Self {
            h: safe_value(0, 360, h),
            s: safe_value(0, 100, s),
            l: safe_value(0, 100, l),
            a: safe_value(0.0, 1.0, a.unwrap_or(1.0)),
        }
    }

    pub fn to_rgb(&self) -> RGBColor {
        let (h, mut s, mut l) = (self.h as f64, self.s as f64, self.l as f64);
        s /= 100.0;
        l /= 100.0;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;
        let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);

        if 0.0 <= h && h < 60.0 {
            r = c;
            g = x;
            b = 0.0;
        } else if 60.0 <= h && h < 120.0 {
            r = x;
            g = c;
            b = 0.0;
        } else if 120.0 <= h && h < 180.0 {
            r = 0.0;
            g = c;
            b = x;
        } else if 180.0 <= h && h < 240.0 {
            r = 0.0;
            g = x;
            b = c;
        } else if 240.0 <= h && h < 300.0 {
            r = x;
            g = 0.0;
            b = c;
        } else if 300.0 <= h && h < 360.0 {
            r = c;
            g = 0.0;
            b = x;
        }

        r = ((r + m) * 255.0).round();
        g = ((g + m) * 255.0).round();
        b = ((b + m) * 255.0).round();

        RGBColor { r: r as u8, g: g as u8, b: b as u8, a: self.a }
    }

    pub fn parse(color: &String) -> Result<Self, Error> {
        let rgb = RGBColor::parse(color)?;
        Ok(rgb.to_hsl())
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.a = safe_value(0.0, 1.0, alpha);
    }

    pub fn is_hsl_string(color: &String) -> bool {
        match Self::parse(color) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn print(&self) {
        println!(
            "HSL Color:\nHue: {}\nSaturation: {}\nLightness: {}\nAlpha: {}",
            &self.h, &self.s, &self.l, &self.a
        );
    }
}
