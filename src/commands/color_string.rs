use crate::color::{
    cmyk::{is_cmyk_string, CMYKColor},
    hex::{is_hex_string, HexColor},
    hsl::{is_hsl_string, HSLColor},
    rgb::{is_rgb_string, RGBColor},
};

pub fn run(color: &String) {
    if is_hex_string(color) {
        HexColor::from(color).unwrap().print_others()
    } else if is_rgb_string(color) {
        RGBColor::from(color).unwrap().print_others()
    } else if is_hsl_string(color) {
        HSLColor::from(color).unwrap().print_others()
    } else if is_cmyk_string(color) {
        CMYKColor::from(color).unwrap().print_others()
    } else {
        println!("Invalid color string");
    }
}
