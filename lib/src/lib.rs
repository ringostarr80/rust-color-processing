//! # Color crate
//!
//! This `color_processing` crate contains functions/methods to handle color values,
//! like converting a rgb color to another colorspace (cmyk, hsl, hwb, ...),
//! parse different kinds of color-strings or
//! modifying colors (inverting, grayscale, colorize, ...).
//!
//! It's not intended for image manipulation, just for parsing and processing single colors.
//!
//! # Examples
//!
//! ```
//! use color_processing::Color;
//!
//! let red = Color::new_rgb(255, 0, 0);
//! assert_eq!(255, red.red);
//! assert_eq!(0, red.green);
//! assert_eq!(0, red.blue);
//!
//! let grayscaled_red = red.grayscale();
//! assert_eq!(76, grayscaled_red.red);
//! assert_eq!(76, grayscaled_red.green);
//! assert_eq!(76, grayscaled_red.blue);
//!
//! assert_eq!("#4C4C4C", grayscaled_red.to_hex_string());
//! assert_eq!("rgb(76, 76, 76)", grayscaled_red.to_rgb_string());
//! assert_eq!("cmyk(0%, 0%, 0%, 70%)", grayscaled_red.to_cmyk_string());
//! assert_eq!("hsl(0, 0%, 29.8%)", grayscaled_red.to_hsl_string());
//!
//! // for colorizing:
//! let colorized_blue = grayscaled_red.colorize_string("blue").unwrap();
//! assert_eq!("rgb(0, 0, 76)", colorized_blue.to_rgb_string());
//!
//! // To get the raw values of a specific colorspace:
//! // The ranges go from 0.0 (0%) to 1.0 (100%).
//! let raw_rgba = red.get_rgba();
//! assert_eq!(1.0, raw_rgba.0); // red value
//! assert_eq!(0.0, raw_rgba.1); // green value
//! assert_eq!(0.0, raw_rgba.2); // blue value
//! assert_eq!(1.0, raw_rgba.3); // alpha value
//!
//! let raw_cmyk = red.get_cmyk();
//! assert_eq!(0.0, raw_cmyk.0); // cyan value
//! assert_eq!(1.0, raw_cmyk.1); // magenta value
//! assert_eq!(1.0, raw_cmyk.2); // yellow value
//! assert_eq!(0.0, raw_cmyk.3); // key (black) value
//!
//! // several ways of parsing strings is also possible:
//! let green = Color::new_string("green").unwrap();
//! let blue = Color::new_string("rgb(0, 0, 255)").unwrap();
//! let cyan = Color::new_string("cmyk(100%, 0%, 0%, 0%)").unwrap();
//! let yellow: Color = "yellow".parse().unwrap();
//! let magenta = "yellow".parse::<Color>().unwrap();
//! ```
//!
//! Now, you should have a notion of what this library can do and if it is the right thing for you!
//!
//! For all the available functionality, please lookout for the [Color](struct.Color.html)-struct.

#[macro_use]
extern crate lazy_static;

extern crate regex;

use self::regex::Regex;
use std::f64::consts::PI;
use std::str::FromStr;

fn round_with_precision(number: f64, precision: u8) -> f64 {
    let multiplier = 10_f64.powi(precision as i32);
    (number * multiplier).round() / multiplier
}

#[derive(Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
    original_string: String,
}

impl Color {
    const LAB_CONSTANT_T0: f64 = 0.137931034; // 4 / 29
    const LAB_CONSTANT_T1: f64 = 0.206896552; // 6 / 29
    const LAB_CONSTANT_T2: f64 = 0.12841855; // 3 * t1 * t1
    const LAB_CONSTANT_T3: f64 = 0.008856452; // t1 * t1 * t1
                                              // Corresponds roughly to RGB brighter/darker
    const LAB_CONSTANT_KN: f64 = 18.0;
    // D65 standard referent
    const LAB_CONSTANT_XN: f64 = 0.950470;
    const LAB_CONSTANT_YN: f64 = 1.0;
    const LAB_CONSTANT_ZN: f64 = 1.088830;

    const RAD2DEG: f64 = 180.0 / PI;
    const DEG2RAD: f64 = PI / 180.0;

    /// Gets a new Color struct, that represents the "black"-color.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let black = Color::new();
    ///
    /// assert_eq!(0, black.red);
    /// assert_eq!(0, black.green);
    /// assert_eq!(0, black.blue);
    /// assert_eq!(255, black.alpha);
    /// ```
    pub fn new() -> Color {
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 255,
            original_string: String::new(),
        }
    }

    /// Gets a new Color struct, that represents a color with the given cyan, magenta, yellow and key (black) values.
    ///
    /// * The value range of cyan, magenta, yellow and key (black) is from 0.0 to 1.0 represents the intensity from 0% to 100%.
    /// * If a value is out of this range, it will be automatically clipped, e.g.: -0.123 becomes 0.0 and 231.31 becomes 1.0!
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_cmyk(0.0, 1.0, 1.0, 0.0);
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(255, red.alpha);
    /// ```
    pub fn new_cmyk(cyan: f64, magenta: f64, yellow: f64, key: f64) -> Color {
        let rgb = Color::get_rgb_from_cmyk(cyan, magenta, yellow, key);

        Color::new_rgb(rgb.0, rgb.1, rgb.2)
    }

    /// Gets a new Color struct, that represents a color with the given KnownColors-enum values.
    ///
    /// * The names and values are equal from the [www.w3.org](https://www.w3.org/TR/css-color-4/#named-colors) Website for the css named colors.
    ///
    /// # Example
    /// ```
    /// use color_processing::{Color, KnownColors};
    ///
    /// let red = Color::new_enum(KnownColors::Red);
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(255, red.alpha);
    /// ```
    pub fn new_enum(known_color: KnownColors) -> Color {
        match known_color {
            KnownColors::AliceBlue => Color {
                red: 0xF0,
                green: 0xF8,
                blue: 0xFF,
                ..Default::default()
            },
            KnownColors::AntiqueWhite => Color {
                red: 0xFA,
                green: 0xEB,
                blue: 0xD7,
                ..Default::default()
            },
            KnownColors::Aqua => Color {
                red: 0x00,
                green: 0xFF,
                blue: 0xFF,
                ..Default::default()
            },
            KnownColors::AquaMarine => Color {
                red: 0x7F,
                green: 0xFF,
                blue: 0xD4,
                ..Default::default()
            },
            KnownColors::Azure => Color {
                red: 0xF0,
                green: 0xFF,
                blue: 0xFF,
                ..Default::default()
            },
            KnownColors::Beige => Color {
                red: 0xF5,
                green: 0xF5,
                blue: 0xDC,
                ..Default::default()
            },
            KnownColors::Bisque => Color {
                red: 0xFF,
                green: 0xE4,
                blue: 0xC4,
                ..Default::default()
            },
            KnownColors::Black => Color {
                red: 0x00,
                green: 0x00,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::BlanchedAlmond => Color {
                red: 0xFF,
                green: 0xEB,
                blue: 0xCD,
                ..Default::default()
            },
            KnownColors::Blue => Color {
                red: 0x00,
                green: 0x00,
                blue: 0xFF,
                ..Default::default()
            },
            KnownColors::BlueViolet => Color {
                red: 0x8A,
                green: 0x2B,
                blue: 0xE2,
                ..Default::default()
            },
            KnownColors::Brown => Color {
                red: 0xA5,
                green: 0x2A,
                blue: 0x2A,
                ..Default::default()
            },
            KnownColors::BurlyWood => Color {
                red: 0xDE,
                green: 0xB8,
                blue: 0x87,
                ..Default::default()
            },
            KnownColors::CadetBlue => Color {
                red: 0x5F,
                green: 0x9E,
                blue: 0xA0,
                ..Default::default()
            },
            KnownColors::Chartreuse => Color {
                red: 0x7F,
                green: 0xFF,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::Chocolate => Color {
                red: 0xD2,
                green: 0x69,
                blue: 0x1E,
                ..Default::default()
            },
            KnownColors::Coral => Color {
                red: 0xFF,
                green: 0x7F,
                blue: 0x50,
                ..Default::default()
            },
            KnownColors::CornflowerBlue => Color {
                red: 0x64,
                green: 0x95,
                blue: 0xED,
                ..Default::default()
            },
            KnownColors::Cornsilk => Color {
                red: 0xFF,
                green: 0xF8,
                blue: 0xDC,
                ..Default::default()
            },
            KnownColors::Crimson => Color {
                red: 0xDC,
                green: 0x14,
                blue: 0x3C,
                ..Default::default()
            },
            KnownColors::Cyan => Color {
                red: 0x00,
                green: 0xFF,
                blue: 0xFF,
                ..Default::default()
            },
            KnownColors::DarkBlue => Color {
                red: 0x00,
                green: 0x00,
                blue: 0x8B,
                ..Default::default()
            },
            KnownColors::DarkCyan => Color {
                red: 0x00,
                green: 0x8B,
                blue: 0x8B,
                ..Default::default()
            },
            KnownColors::DarkGoldenrod => Color {
                red: 0xB8,
                green: 0x86,
                blue: 0x0B,
                ..Default::default()
            },
            KnownColors::DarkGray => Color {
                red: 0xA9,
                green: 0xA9,
                blue: 0xA9,
                ..Default::default()
            },
            KnownColors::DarkGreen => Color {
                red: 0x00,
                green: 0x64,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::DarkKhaki => Color {
                red: 0xBD,
                green: 0xB7,
                blue: 0x6B,
                ..Default::default()
            },
            KnownColors::DarkMagenta => Color {
                red: 0x8B,
                green: 0x00,
                blue: 0x8B,
                ..Default::default()
            },
            KnownColors::DarkOliveGreen => Color {
                red: 0x55,
                green: 0x6B,
                blue: 0x2F,
                ..Default::default()
            },
            KnownColors::DarkOrange => Color {
                red: 0xFF,
                green: 0x8C,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::DarkOrchid => Color {
                red: 0x99,
                green: 0x32,
                blue: 0xCC,
                ..Default::default()
            },
            KnownColors::DarkRed => Color {
                red: 0x8B,
                green: 0x00,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::DarkSalmon => Color {
                red: 0xE9,
                green: 0x96,
                blue: 0x7A,
                ..Default::default()
            },
            KnownColors::DarkSeaGreen => Color {
                red: 0x8F,
                green: 0xBC,
                blue: 0x8B,
                ..Default::default()
            },
            KnownColors::DarkSlateBlue => Color {
                red: 0x48,
                green: 0x3D,
                blue: 0x8B,
                ..Default::default()
            },
            KnownColors::DarkSlateGray => Color {
                red: 0x2F,
                green: 0x4F,
                blue: 0x4F,
                ..Default::default()
            },
            KnownColors::DarkTurquoise => Color {
                red: 0x00,
                green: 0xCE,
                blue: 0xD1,
                ..Default::default()
            },
            KnownColors::DarkViolet => Color {
                red: 0x94,
                green: 0x00,
                blue: 0xD3,
                ..Default::default()
            },
            KnownColors::DeepPink => Color {
                red: 0xFF,
                green: 0x14,
                blue: 0x93,
                ..Default::default()
            },
            KnownColors::DeepSkyBlue => Color {
                red: 0x00,
                green: 0xBF,
                blue: 0xFF,
                ..Default::default()
            },
            KnownColors::DimGray => Color {
                red: 0x69,
                green: 0x69,
                blue: 0x69,
                ..Default::default()
            },
            KnownColors::DodgerBlue => Color {
                red: 0x1E,
                green: 0x90,
                blue: 0xFF,
                ..Default::default()
            },
            KnownColors::Firebrick => Color {
                red: 0xB2,
                green: 0x22,
                blue: 0x22,
                ..Default::default()
            },
            KnownColors::FloralWhite => Color {
                red: 0xFF,
                green: 0xFA,
                blue: 0xF0,
                ..Default::default()
            },
            KnownColors::ForestGreen => Color {
                red: 0x22,
                green: 0x8B,
                blue: 0x22,
                ..Default::default()
            },
            KnownColors::Fuchsia => Color {
                red: 0xFF,
                green: 0x00,
                blue: 0xFF,
                ..Default::default()
            },
            KnownColors::Gainsboro => Color {
                red: 0xDC,
                green: 0xDC,
                blue: 0xDC,
                ..Default::default()
            },
            KnownColors::GhostWhite => Color {
                red: 0xF8,
                green: 0xF8,
                blue: 0xFF,
                ..Default::default()
            },
            KnownColors::Gold => Color {
                red: 0xFF,
                green: 0xD7,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::Goldenrod => Color {
                red: 0xDA,
                green: 0xA5,
                blue: 0x20,
                ..Default::default()
            },
            KnownColors::Gray => Color {
                red: 0x80,
                green: 0x80,
                blue: 0x80,
                ..Default::default()
            },
            KnownColors::Green => Color {
                red: 0x00,
                green: 0x80,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::GreenYellow => Color {
                red: 0xAD,
                green: 0xFF,
                blue: 0x2F,
                ..Default::default()
            },
            KnownColors::Honeydew => Color {
                red: 0xF0,
                green: 0xFF,
                blue: 0xF0,
                ..Default::default()
            },
            KnownColors::HotPink => Color {
                red: 0xFF,
                green: 0x69,
                blue: 0xB4,
                ..Default::default()
            },
            KnownColors::IndianRed => Color {
                red: 0xCD,
                green: 0x5C,
                blue: 0x5C,
                ..Default::default()
            },
            KnownColors::Indigo => Color {
                red: 0x4B,
                green: 0x00,
                blue: 0x82,
                ..Default::default()
            },
            KnownColors::Ivory => Color {
                red: 0xFF,
                green: 0xFF,
                blue: 0xF0,
                ..Default::default()
            },
            KnownColors::Khaki => Color {
                red: 0xF0,
                green: 0xE6,
                blue: 0x8C,
                ..Default::default()
            },
            KnownColors::Lavender => Color {
                red: 0xE6,
                green: 0xE6,
                blue: 0xFA,
                ..Default::default()
            },
            KnownColors::LavenderBlush => Color {
                red: 0xFF,
                green: 0xF0,
                blue: 0xF5,
                ..Default::default()
            },
            KnownColors::LawnGreen => Color {
                red: 0x7C,
                green: 0xFC,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::LemonChiffon => Color {
                red: 0xFF,
                green: 0xFA,
                blue: 0xCD,
                ..Default::default()
            },
            KnownColors::LightBlue => Color {
                red: 0xAD,
                green: 0xD8,
                blue: 0xE6,
                ..Default::default()
            },
            KnownColors::LightCoral => Color {
                red: 0xF0,
                green: 0x80,
                blue: 0x80,
                ..Default::default()
            },
            KnownColors::LightCyan => Color {
                red: 0xE0,
                green: 0xFF,
                blue: 0xFF,
                ..Default::default()
            },
            KnownColors::LightGoldenrodYellow => Color {
                red: 0xFA,
                green: 0xFA,
                blue: 0xD2,
                ..Default::default()
            },
            KnownColors::LightGray => Color {
                red: 0xD3,
                green: 0xD3,
                blue: 0xD3,
                ..Default::default()
            },
            KnownColors::LightGreen => Color {
                red: 0x90,
                green: 0xEE,
                blue: 0x90,
                ..Default::default()
            },
            KnownColors::LightPink => Color {
                red: 0xFF,
                green: 0xB6,
                blue: 0xC1,
                ..Default::default()
            },
            KnownColors::LightSalmon => Color {
                red: 0xFF,
                green: 0xA0,
                blue: 0x7A,
                ..Default::default()
            },
            KnownColors::LightSeaGreen => Color {
                red: 0x20,
                green: 0xB2,
                blue: 0xAA,
                ..Default::default()
            },
            KnownColors::LightSkyBlue => Color {
                red: 0x87,
                green: 0xCE,
                blue: 0xFA,
                ..Default::default()
            },
            KnownColors::LightSlateGray => Color {
                red: 0x77,
                green: 0x88,
                blue: 0x99,
                ..Default::default()
            },
            KnownColors::LightSteelBlue => Color {
                red: 0xB0,
                green: 0xC4,
                blue: 0xDE,
                ..Default::default()
            },
            KnownColors::LightYellow => Color {
                red: 0xFF,
                green: 0xFF,
                blue: 0xE0,
                ..Default::default()
            },
            KnownColors::Lime => Color {
                red: 0x00,
                green: 0xFF,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::LimeGreen => Color {
                red: 0x32,
                green: 0xCD,
                blue: 0x32,
                ..Default::default()
            },
            KnownColors::Linen => Color {
                red: 0xFA,
                green: 0xF0,
                blue: 0xE6,
                ..Default::default()
            },
            KnownColors::Magenta => Color {
                red: 0xFF,
                green: 0x00,
                blue: 0xFF,
                ..Default::default()
            },
            KnownColors::Maroon => Color {
                red: 0x80,
                green: 0x00,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::MediumAquaMarine => Color {
                red: 0x66,
                green: 0xCD,
                blue: 0xAA,
                ..Default::default()
            },
            KnownColors::MediumBlue => Color {
                red: 0x00,
                green: 0x00,
                blue: 0xCD,
                ..Default::default()
            },
            KnownColors::MediumOrchid => Color {
                red: 0xBA,
                green: 0x55,
                blue: 0xD3,
                ..Default::default()
            },
            KnownColors::MediumPurple => Color {
                red: 0x93,
                green: 0x70,
                blue: 0xDB,
                ..Default::default()
            },
            KnownColors::MediumSeaGreen => Color {
                red: 0x3C,
                green: 0xB3,
                blue: 0x71,
                ..Default::default()
            },
            KnownColors::MediumSlateBlue => Color {
                red: 0x7B,
                green: 0x68,
                blue: 0xEE,
                ..Default::default()
            },
            KnownColors::MediumSpringGreen => Color {
                red: 0x00,
                green: 0xFA,
                blue: 0x9A,
                ..Default::default()
            },
            KnownColors::MediumTurquoise => Color {
                red: 0x48,
                green: 0xD1,
                blue: 0xCC,
                ..Default::default()
            },
            KnownColors::MediumVioletRed => Color {
                red: 0xC7,
                green: 0x15,
                blue: 0x85,
                ..Default::default()
            },
            KnownColors::MidnightBlue => Color {
                red: 0x19,
                green: 0x19,
                blue: 0x70,
                ..Default::default()
            },
            KnownColors::MintCream => Color {
                red: 0xF5,
                green: 0xFF,
                blue: 0xFA,
                ..Default::default()
            },
            KnownColors::MistyRose => Color {
                red: 0xFF,
                green: 0xE4,
                blue: 0xE1,
                ..Default::default()
            },
            KnownColors::Moccasin => Color {
                red: 0xFF,
                green: 0xE4,
                blue: 0xB5,
                ..Default::default()
            },
            KnownColors::NavajoWhite => Color {
                red: 0xFF,
                green: 0xDE,
                blue: 0xAD,
                ..Default::default()
            },
            KnownColors::Navy => Color {
                red: 0x00,
                green: 0x00,
                blue: 0x80,
                ..Default::default()
            },
            KnownColors::OldLace => Color {
                red: 0xFD,
                green: 0xF5,
                blue: 0xE6,
                ..Default::default()
            },
            KnownColors::Olive => Color {
                red: 0x80,
                green: 0x80,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::OliveDrab => Color {
                red: 0x6B,
                green: 0x8E,
                blue: 0x23,
                ..Default::default()
            },
            KnownColors::Orange => Color {
                red: 0xFF,
                green: 0xA5,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::OrangeRed => Color {
                red: 0xFF,
                green: 0x45,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::Orchid => Color {
                red: 0xDA,
                green: 0x70,
                blue: 0xD6,
                ..Default::default()
            },
            KnownColors::PaleGoldenrod => Color {
                red: 0xEE,
                green: 0xE8,
                blue: 0xAA,
                ..Default::default()
            },
            KnownColors::PaleGreen => Color {
                red: 0x98,
                green: 0xFB,
                blue: 0x98,
                ..Default::default()
            },
            KnownColors::PaleTurquoise => Color {
                red: 0xAF,
                green: 0xEE,
                blue: 0xEE,
                ..Default::default()
            },
            KnownColors::PaleVioletRed => Color {
                red: 0xDB,
                green: 0x70,
                blue: 0x93,
                ..Default::default()
            },
            KnownColors::PapayaWhip => Color {
                red: 0xFF,
                green: 0xEF,
                blue: 0xD5,
                ..Default::default()
            },
            KnownColors::PeachPuff => Color {
                red: 0xFF,
                green: 0xDA,
                blue: 0xB9,
                ..Default::default()
            },
            KnownColors::Peru => Color {
                red: 0xCD,
                green: 0x85,
                blue: 0x3F,
                ..Default::default()
            },
            KnownColors::Pink => Color {
                red: 0xFF,
                green: 0xC0,
                blue: 0xCB,
                ..Default::default()
            },
            KnownColors::Plum => Color {
                red: 0xDD,
                green: 0xA0,
                blue: 0xDD,
                ..Default::default()
            },
            KnownColors::PowderBlue => Color {
                red: 0xB0,
                green: 0xE0,
                blue: 0xE6,
                ..Default::default()
            },
            KnownColors::Purple => Color {
                red: 0x80,
                green: 0x00,
                blue: 0x80,
                ..Default::default()
            },
            KnownColors::Red => Color {
                red: 0xFF,
                green: 0x00,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::RosyBrown => Color {
                red: 0xBC,
                green: 0x8F,
                blue: 0x8F,
                ..Default::default()
            },
            KnownColors::RoyalBlue => Color {
                red: 0x41,
                green: 0x69,
                blue: 0xE1,
                ..Default::default()
            },
            KnownColors::SaddleBrown => Color {
                red: 0x8B,
                green: 0x45,
                blue: 0x13,
                ..Default::default()
            },
            KnownColors::Salmon => Color {
                red: 0xFA,
                green: 0x80,
                blue: 0x72,
                ..Default::default()
            },
            KnownColors::SandyBrown => Color {
                red: 0xF4,
                green: 0xA4,
                blue: 0x60,
                ..Default::default()
            },
            KnownColors::SeaGreen => Color {
                red: 0x2E,
                green: 0x8B,
                blue: 0x57,
                ..Default::default()
            },
            KnownColors::SeaShell => Color {
                red: 0xFF,
                green: 0xF5,
                blue: 0xEE,
                ..Default::default()
            },
            KnownColors::Sienna => Color {
                red: 0xA0,
                green: 0x52,
                blue: 0x2D,
                ..Default::default()
            },
            KnownColors::Silver => Color {
                red: 0xC0,
                green: 0xC0,
                blue: 0xC0,
                ..Default::default()
            },
            KnownColors::SkyBlue => Color {
                red: 0x87,
                green: 0xCE,
                blue: 0xEB,
                ..Default::default()
            },
            KnownColors::SlateBlue => Color {
                red: 0x6A,
                green: 0x5A,
                blue: 0xCD,
                ..Default::default()
            },
            KnownColors::SlateGray => Color {
                red: 0x70,
                green: 0x80,
                blue: 0x90,
                ..Default::default()
            },
            KnownColors::Snow => Color {
                red: 0xFF,
                green: 0xFA,
                blue: 0xFA,
                ..Default::default()
            },
            KnownColors::SpringGreen => Color {
                red: 0x00,
                green: 0xFF,
                blue: 0x7F,
                ..Default::default()
            },
            KnownColors::SteelBlue => Color {
                red: 0x46,
                green: 0x82,
                blue: 0xB4,
                ..Default::default()
            },
            KnownColors::Tan => Color {
                red: 0xD2,
                green: 0xB4,
                blue: 0x8C,
                ..Default::default()
            },
            KnownColors::Teal => Color {
                red: 0x00,
                green: 0x80,
                blue: 0x80,
                ..Default::default()
            },
            KnownColors::Thistle => Color {
                red: 0xD8,
                green: 0xBF,
                blue: 0xD8,
                ..Default::default()
            },
            KnownColors::Tomato => Color {
                red: 0xFF,
                green: 0x63,
                blue: 0x47,
                ..Default::default()
            },
            KnownColors::Transparent => Color {
                red: 0x00,
                green: 0x00,
                blue: 0x00,
                alpha: 0x00,
                ..Default::default()
            },
            KnownColors::Turquoise => Color {
                red: 0x40,
                green: 0xE0,
                blue: 0xD0,
                ..Default::default()
            },
            KnownColors::Violet => Color {
                red: 0xEE,
                green: 0x82,
                blue: 0xEE,
                ..Default::default()
            },
            KnownColors::Wheat => Color {
                red: 0xF5,
                green: 0xDE,
                blue: 0xB3,
                ..Default::default()
            },
            KnownColors::White => Color {
                red: 0xFF,
                green: 0xFF,
                blue: 0xFF,
                ..Default::default()
            },
            KnownColors::WhiteSmoke => Color {
                red: 0xF5,
                green: 0xF5,
                blue: 0xF5,
                ..Default::default()
            },
            KnownColors::Yellow => Color {
                red: 0xFF,
                green: 0xFF,
                blue: 0x00,
                ..Default::default()
            },
            KnownColors::YellowGreen => Color {
                red: 0x9A,
                green: 0xCD,
                blue: 0x32,
                ..Default::default()
            },
        }
    }

    /// Gets a new Color struct, that represents a color with a gray value.
    ///
    /// * The value range of gray is from 0 to 255.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let gray = Color::new_gray(100);
    ///
    /// assert_eq!(100, gray.red);
    /// assert_eq!(100, gray.green);
    /// assert_eq!(100, gray.blue);
    /// assert_eq!(255, gray.alpha);
    /// ```
    pub fn new_gray(gray: u8) -> Color {
        Color {
            red: gray,
            green: gray,
            blue: gray,
            ..Default::default()
        }
    }

    /// Gets a new Color struct, that represents a color with the hue, saturation and lightness values.
    ///
    /// * The value range of hue is from 0.0 to 360.0 in degrees.
    /// * If the value of the hue is out of range, it will be normalized. e.g.: 420.0 becomes 60.0 and -40.0 becomes 320.0.
    /// * The value range of saturation and lightness is from 0.0 to 1.0 represents the intensity from 0% to 100%.
    /// * If the value of the saturation or lightness is out of range, it will be automatically clipped, e.g.: -0.123 becomes 0.0 and 231.31 becomes 1.0!
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_hsl(0.0, 1.0, 0.5);
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(255, red.alpha);
    /// ```
    pub fn new_hsl(hue: f64, saturation: f64, lightness: f64) -> Color {
        Color::new_hsla(hue, saturation, lightness, 1.0)
    }

    /// Gets a new Color struct, that represents a color with the hue, saturation, lightness and alpha values.
    ///
    /// * The value range of hue is from 0.0 to 360.0 in degrees.
    /// * If the value of the hue is out of range, it will be normalized. e.g.: 420.0 becomes 60.0 and -40.0 becomes 320.0.
    /// * The value range of saturation, lightness and alpha is from 0.0 to 1.0 represents the intensity from 0% to 100%.
    /// * If the value of the saturation, lightness or alpha is out of range, it will be automatically clipped, e.g.: -0.123 becomes 0.0 and 231.31 becomes 1.0!
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_hsla(0.0, 1.0, 0.5, 0.5);
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(128, red.alpha);
    /// ```
    pub fn new_hsla(hue: f64, saturation: f64, lightness: f64, alpha: f64) -> Color {
        let a = if alpha < 0.0 {
            0
        } else if alpha > 1.0 {
            255
        } else {
            (alpha * 255.0).round() as u8
        };

        let rgb = Color::get_rgb_from_hsl(hue, saturation, lightness);

        Color::new_rgba(rgb.0, rgb.1, rgb.2, a)
    }

    /// Gets a new Color struct, that represents a color with the hue, saturation and value values.
    ///
    /// * The value range of hue is from 0.0 to 360.0 in degrees.
    /// * If the value of the hue is out of range, it will be normalized. e.g.: 420.0 becomes 60.0 and -40.0 becomes 320.0.
    /// * The value range of saturation and value is from 0.0 to 1.0 represents the intensity from 0% to 100%.
    /// * If the value of the saturation or value is out of range, it will be automatically clipped, e.g.: -0.123 becomes 0.0 and 231.31 becomes 1.0!
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_hsv(0.0, 1.0, 1.0);
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(255, red.alpha);
    /// ```
    pub fn new_hsv(hue: f64, saturation: f64, value: f64) -> Color {
        let rgb = Color::get_rgb_from_hsv(hue, saturation, value);

        Color::new_rgb(rgb.0, rgb.1, rgb.2)
    }

    /// Gets a new Color struct, that represents a color with the hue, saturation, value and alpha values.
    ///
    /// * The value range of hue is from 0.0 to 360.0 in degrees.
    /// * If the value of the hue is out of range, it will be normalized. e.g.: 420.0 becomes 60.0 and -40.0 becomes 320.0.
    /// * The value range of saturation, value and alpha is from 0.0 to 1.0 represents the intensity from 0% to 100%.
    /// * If the value of the saturation, value or alpha is out of range, it will be automatically clipped, e.g.: -0.123 becomes 0.0 and 231.31 becomes 1.0!
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_hsva(0.0, 1.0, 1.0, 0.5);
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(128, red.alpha);
    /// ```
    pub fn new_hsva(hue: f64, saturation: f64, value: f64, alpha: f64) -> Color {
        let a = if alpha < 0.0 {
            0
        } else if alpha > 1.0 {
            255
        } else {
            (alpha * 255.0).round() as u8
        };

        let rgb = Color::get_rgb_from_hsv(hue, saturation, value);

        Color::new_rgba(rgb.0, rgb.1, rgb.2, a)
    }

    /// Gets a new Color struct, that represents a color with the hue, whiteness and blackness values.
    ///
    /// * The value range of hue is from 0.0 to 360.0 in degrees.
    /// * If the value of the hue is out of range, it will be normalized. e.g.: 420.0 becomes 60.0 and -40.0 becomes 320.0.
    /// * The value range of whiteness and blackness is from 0.0 to 1.0 represents the intensity from 0% to 100%.
    /// * If the value of the whiteness or blackness is out of range, it will be automatically clipped, e.g.: -0.123 becomes 0.0 and 231.31 becomes 1.0!
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_hwb(0.0, 0.0, 0.0);
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(255, red.alpha);
    /// ```
    pub fn new_hwb(hue: f64, whiteness: f64, blackness: f64) -> Color {
        let rgb = Color::get_rgb_from_hwb(hue, whiteness, blackness);

        Color::new_rgb(rgb.0, rgb.1, rgb.2)
    }

    /// Gets a new Color struct, that represents a color with the hue, whiteness, blackness and alpha values.
    ///
    /// * The value range of hue is from 0.0 to 360.0 in degrees.
    /// * If the value of the hue is out of range, it will be normalized. e.g.: 420.0 becomes 60.0 and -40.0 becomes 320.0.
    /// * The value range of whiteness, blackness and alpha is from 0.0 to 1.0 represents the intensity from 0% to 100%.
    /// * If the value of the whiteness, blackness or alpha is out of range, it will be automatically clipped, e.g.: -0.123 becomes 0.0 and 231.31 becomes 1.0!
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_hwba(0.0, 0.0, 0.0, 0.5);
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(128, red.alpha);
    /// ```
    pub fn new_hwba(hue: f64, whiteness: f64, blackness: f64, alpha: f64) -> Color {
        let a = if alpha < 0.0 {
            0
        } else if alpha > 1.0 {
            255
        } else {
            (alpha * 255.0).round() as u8
        };

        let rgb = Color::get_rgb_from_hwb(hue, whiteness, blackness);

        Color::new_rgba(rgb.0, rgb.1, rgb.2, a)
    }

    /// Gets a new Color struct, that represents a color with the lightness, a and b values.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let black_lab = Color::new_lab(0.0, 0.0, 0.0);
    /// assert_eq!(black_lab.to_hex_string(), "#000000");
    ///
    /// let white_lab = Color::new_lab(100.0, 0.0, 0.0);
    /// assert_eq!(white_lab.to_hex_string(), "#FFFFFF");
    ///
    /// let gray_lab = Color::new_lab(53.59, 0.0, 0.0);
    /// assert_eq!(gray_lab.to_hex_string(), "#808080");
    ///
    /// let red_lab = Color::new_lab(53.24, 80.09, 67.2);
    /// assert_eq!(red_lab.to_hex_string(), "#FF0000");
    ///
    /// let yellow_lab = Color::new_lab(97.14, -21.55, 94.48);
    /// assert_eq!(yellow_lab.to_hex_string(), "#FFFF00");
    ///
    /// let green_lab = Color::new_lab(87.73, -86.18, 83.18);
    /// assert_eq!(green_lab.to_hex_string(), "#00FF00");
    ///
    /// let cyan_lab = Color::new_lab(91.11, -48.09, -14.13);
    /// assert_eq!(cyan_lab.to_hex_string(), "#00FFFF");
    ///
    /// let blue_lab = Color::new_lab(32.3, 79.19, -107.86);
    /// assert_eq!(blue_lab.to_hex_string(), "#0000FF");
    /// ```
    pub fn new_lab(l: f64, a: f64, b: f64) -> Color {
        Color::new_laba(l, a, b, 1.0)
    }

    /// Gets a new Color struct, that represents a color with the lightness, a, b and alpha values.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let black_lab = Color::new_laba(0.0, 0.0, 0.0, 1.0);
    /// assert_eq!(black_lab.to_hex_string(), "#000000");
    ///
    /// let white_lab = Color::new_laba(100.0, 0.0, 0.0, 0.5);
    /// assert_eq!(white_lab.to_hex_string(), "#FFFFFF80");
    ///
    /// let gray_lab = Color::new_laba(53.59, 0.0, 0.0, 0.5);
    /// assert_eq!(gray_lab.to_hex_string(), "#80808080");
    ///
    /// let red_lab = Color::new_laba(53.24, 80.09, 67.2, 1.0);
    /// assert_eq!(red_lab.to_hex_string(), "#FF0000");
    ///
    /// let yellow_lab = Color::new_laba(97.14, -21.55, 94.48, 0.0);
    /// assert_eq!(yellow_lab.to_hex_string(), "#FFFF0000");
    ///
    /// let green_lab = Color::new_laba(87.73, -86.18, 83.18, 1.0);
    /// assert_eq!(green_lab.to_hex_string(), "#00FF00");
    ///
    /// let cyan_lab = Color::new_laba(91.11, -48.09, -14.13, 1.0);
    /// assert_eq!(cyan_lab.to_hex_string(), "#00FFFF");
    ///
    /// let blue_lab = Color::new_laba(32.3, 79.19, -107.86, 1.0);
    /// assert_eq!(blue_lab.to_hex_string(), "#0000FF");
    /// ```
    pub fn new_laba(l: f64, a: f64, b: f64, alpha: f64) -> Color {
        let alpha = if alpha < 0.0 {
            0
        } else if alpha > 1.0 {
            255
        } else {
            (alpha * 255.0).round() as u8
        };

        let rgb = Color::lab_2_rgb(l, a, b);

        Color::new_rgba(
            rgb.0.round() as u8,
            rgb.1.round() as u8,
            rgb.2.round() as u8,
            alpha,
        )
    }

    /// Gets a new Color struct, that represents a color with the lightness, chroma and hue values.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let black_lch = Color::new_lch(0.0, 0.0, std::f64::NAN);
    /// assert_eq!(black_lch.to_rgb_string(), "rgb(0, 0, 0)");
    ///
    /// let white_lch = Color::new_lch(100.0, 0.0, std::f64::NAN);
    /// assert_eq!(white_lch.to_rgb_string(), "rgb(255, 255, 255)");
    ///
    /// let gray_lch = Color::new_lch(53.59, 0.0, std::f64::NAN);
    /// assert_eq!(gray_lch.to_rgb_string(), "rgb(128, 128, 128)");
    ///
    /// let red_lch = Color::new_lch(53.24, 104.55, 40.0);
    /// assert_eq!(red_lch.to_rgb_string(), "rgb(255, 0, 0)");
    ///
    /// let yellow_lch = Color::new_lch(97.14, 96.91, 102.85);
    /// assert_eq!(yellow_lch.to_rgb_string(), "rgb(255, 255, 0)");
    ///
    /// let green_lch = Color::new_lch(87.73, 119.78, 136.02);
    /// assert_eq!(green_lch.to_rgb_string(), "rgb(0, 255, 0)");
    ///
    /// let cyan_lch = Color::new_lch(91.11, 50.12, 196.38);
    /// assert_eq!(cyan_lch.to_rgb_string(), "rgb(0, 255, 255)");
    ///
    /// let blue_lch = Color::new_lch(32.3, 133.81, 306.28);
    /// assert_eq!(blue_lch.to_rgb_string(), "rgb(0, 0, 255)");
    ///
    /// let magenta_lch = Color::new_lch(60.32, 115.54, 328.23);
    /// assert_eq!(magenta_lch.to_rgb_string(), "rgb(255, 0, 255)");
    /// ```
    pub fn new_lch(lightness: f64, chroma: f64, hue: f64) -> Color {
        Color::new_lcha(lightness, chroma, hue, 1.0)
    }

    /// Gets a new Color struct, that represents a color with the lightness, chroma, hue and alpha values.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let black_lch = Color::new_lcha(0.0, 0.0, std::f64::NAN, 1.0);
    /// assert_eq!(black_lch.to_rgb_string(), "rgb(0, 0, 0)");
    ///
    /// let white_lch = Color::new_lcha(100.0, 0.0, std::f64::NAN, 0.0);
    /// assert_eq!(white_lch.to_rgb_string(), "rgba(255, 255, 255, 0)");
    ///
    /// let gray_lch = Color::new_lcha(53.59, 0.0, std::f64::NAN, 0.5);
    /// assert_eq!(gray_lch.to_rgb_string(), "rgba(128, 128, 128, 0.5)");
    ///
    /// let red_lch = Color::new_lcha(53.24, 104.55, 40.0, 0.5);
    /// assert_eq!(red_lch.to_rgb_string(), "rgba(255, 0, 0, 0.5)");
    ///
    /// let yellow_lch = Color::new_lcha(97.14, 96.91, 102.8, 1.0);
    /// assert_eq!(yellow_lch.to_rgb_string(), "rgb(255, 255, 0)");
    ///
    /// let green_lch = Color::new_lcha(87.73, 119.78, 136.02, 1.0);
    /// assert_eq!(green_lch.to_rgb_string(), "rgb(0, 255, 0)");
    ///
    /// let cyan_lch = Color::new_lcha(91.11, 50.12, 196.38, 1.0);
    /// assert_eq!(cyan_lch.to_rgb_string(), "rgb(0, 255, 255)");
    ///
    /// let blue_lch = Color::new_lcha(32.3, 133.81, 306.28, 1.0);
    /// assert_eq!(blue_lch.to_rgb_string(), "rgb(0, 0, 255)");
    ///
    /// let magenta_lch = Color::new_lcha(60.32, 115.54, 328.23, 1.0);
    /// assert_eq!(magenta_lch.to_rgb_string(), "rgb(255, 0, 255)");
    /// ```
    pub fn new_lcha(lightness: f64, chroma: f64, hue: f64, alpha: f64) -> Color {
        let a = if alpha < 0.0 {
            0
        } else if alpha > 1.0 {
            255
        } else {
            (alpha * 255.0).round() as u8
        };

        let lab = Color::lch_2_lab(lightness, chroma, hue);
        let rgb = Color::lab_2_rgb(lab.0, lab.1, lab.2);
        let r = if rgb.0 < 0.0 {
            0
        } else if rgb.0 > 255.0 {
            255
        } else {
            rgb.0.round() as u8
        };
        let g = if rgb.1 < 0.0 {
            0
        } else if rgb.1 > 255.0 {
            255
        } else {
            rgb.1.round() as u8
        };
        let b = if rgb.2 < 0.0 {
            0
        } else if rgb.2 > 255.0 {
            255
        } else {
            rgb.2.round() as u8
        };

        Color::new_rgba(r, g, b, a)
    }

    /// Gets a new Color struct, that represents a color with the given red, green and blue values.
    ///
    /// * The value range of red, green and blue is from 0 to 255.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_rgb(255, 0, 0);
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(255, red.alpha);
    /// ```
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {
            red,
            green,
            blue,
            ..Default::default()
        }
    }

    /// Gets a new Color struct, that represents a color with the given red, green, blue and alpha values.
    ///
    /// * The value range of red, green, blue and alpha (opacity) is from 0 to 255.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_rgba(255, 0, 0, 128);
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(128, red.alpha);
    /// ```
    pub fn new_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            red,
            green,
            blue,
            alpha,
            original_string: String::new(),
        }
    }

    /// Gets a new Option&lt;Color&gt;, that represents a color by a string.
    ///
    /// * Examples
    ///   * [known color names](#known-color-names)
    ///   * [abbreviated names](#abbreviated-names)
    ///   * [hex notation](#hex-notation)
    ///   * [rgb(a) notation](#rgb(a)-notation)
    ///   * [gray notation](#gray-notation)
    ///   * [cmyk notation](#cmyk-notation)
    ///   * [hsl(a) notation](#hsl(a)-notation)
    ///   * [hsv(a) notation](#hsv(a)-notation)
    ///   * [hwb(a) notation](#hwb(a)-notation)
    ///
    /// <a name="known-color-names"></a>
    /// # Example (known color names)
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("red").unwrap();
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(255, red.alpha);
    /// ```
    ///
    /// <a name="abbreviated-names"></a>
    /// # Example (abbreviated names)
    /// ```
    /// use color_processing::Color;
    ///
    /// let green = Color::new_string("GN").unwrap();
    ///
    /// assert_eq!(0, green.red);
    /// assert_eq!(128, green.green);
    /// assert_eq!(0, green.blue);
    /// assert_eq!(255, green.alpha);
    /// ```
    ///
    /// <a name="hex-notation"></a>
    /// # Example (hex-notation)
    /// ```
    /// use color_processing::Color;
    ///
    /// let blue = Color::new_string("#0000ff").unwrap();
    ///
    /// assert_eq!(0, blue.red);
    /// assert_eq!(0, blue.green);
    /// assert_eq!(255, blue.blue);
    /// assert_eq!(255, blue.alpha);
    ///
    /// let transparent_blue = Color::new_string("#0000ff80").unwrap();
    ///
    /// assert_eq!(0, transparent_blue.red);
    /// assert_eq!(0, transparent_blue.green);
    /// assert_eq!(255, transparent_blue.blue);
    /// assert_eq!(128, transparent_blue.alpha);
    ///
    /// let yellow = Color::new_string("#ff0").unwrap();
    ///
    /// assert_eq!(255, yellow.red);
    /// assert_eq!(255, yellow.green);
    /// assert_eq!(0, yellow.blue);
    /// assert_eq!(255, yellow.alpha);
    ///
    /// let transparent_yellow = Color::new_string("#ff07").unwrap();
    ///
    /// assert_eq!(255, transparent_yellow.red);
    /// assert_eq!(255, transparent_yellow.green);
    /// assert_eq!(0, transparent_yellow.blue);
    /// assert_eq!(119, transparent_yellow.alpha);
    /// ```
    ///
    /// <a name="rgb(a)-notation"></a>
    /// # Example (rgb(a) notation)
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("rgb(255, 0, 0)").unwrap();
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(255, red.alpha);
    ///
    /// let green = Color::new_string("rgb(0%, 100%, 0%)").unwrap();
    ///
    /// assert_eq!(0, green.red);
    /// assert_eq!(255, green.green);
    /// assert_eq!(0, green.blue);
    /// assert_eq!(255, green.alpha);
    ///
    /// let blue = Color::new_string("rgba(0, 0, 255, 0.5)").unwrap();
    ///
    /// assert_eq!(0, blue.red);
    /// assert_eq!(0, blue.green);
    /// assert_eq!(255, blue.blue);
    /// assert_eq!(128, blue.alpha);
    ///
    /// let yellow = Color::new_string("rgba(100%, 100%, 0%, 0.5)").unwrap();
    ///
    /// assert_eq!(255, yellow.red);
    /// assert_eq!(255, yellow.green);
    /// assert_eq!(0, yellow.blue);
    /// assert_eq!(128, yellow.alpha);
    /// ```
    ///
    /// <a name="gray-notation"></a>
    /// # Example (gray notation)
    /// ```
    /// use color_processing::Color;
    ///
    /// let gray = Color::new_string("gray(128)").unwrap();
    /// assert_eq!(128, gray.red);
    /// assert_eq!(128, gray.green);
    /// assert_eq!(128, gray.blue);
    /// assert_eq!(255, gray.alpha);
    ///
    /// let gray = Color::new_string("gray(50%)").unwrap();
    /// assert_eq!(128, gray.red);
    /// assert_eq!(128, gray.green);
    /// assert_eq!(128, gray.blue);
    /// assert_eq!(255, gray.alpha);
    ///
    /// let transparent_light_gray = Color::new_string("gray(50, 0.75)").unwrap();
    /// assert_eq!(50, transparent_light_gray.red);
    /// assert_eq!(50, transparent_light_gray.green);
    /// assert_eq!(50, transparent_light_gray.blue);
    /// assert_eq!(191, transparent_light_gray.alpha);
    ///
    /// let transparent_dark_gray = Color::new_string("gray(200, 50%)").unwrap();
    /// assert_eq!(200, transparent_dark_gray.red);
    /// assert_eq!(200, transparent_dark_gray.green);
    /// assert_eq!(200, transparent_dark_gray.blue);
    /// assert_eq!(128, transparent_dark_gray.alpha);
    /// ```
    ///
    /// <a name="cmyk-notation"></a>
    /// # Example (cmyk notation)
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("cmyk(0%, 100%, 100%, 0%)").unwrap();
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(255, red.alpha);
    /// ```
    ///
    /// <a name="hsl(a)-notation"></a>
    /// # Example (hsl(a) notation)
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("hsl(0, 100%, 50%)").unwrap();
    /// assert_eq!(red.red, 255);
    /// assert_eq!(red.green, 0);
    /// assert_eq!(red.blue, 0);
    /// assert_eq!(red.alpha, 255);
    ///
    /// let green = Color::new_string("hsl(120°, 100%, 50%)").unwrap();
    /// assert_eq!(green.red, 0);
    /// assert_eq!(green.green, 255);
    /// assert_eq!(green.blue, 0);
    /// assert_eq!(green.alpha, 255);
    ///
    /// let transparent_green = Color::new_string("hsla(120°, 100%, 50%, 0.5)").unwrap();
    /// assert_eq!(transparent_green.red, 0);
    /// assert_eq!(transparent_green.green, 255);
    /// assert_eq!(transparent_green.blue, 0);
    /// assert_eq!(transparent_green.alpha, 128);
    /// ```
    ///
    /// <a name="hsv(a)-notation"></a>
    /// # Example (hsv(a) notation)
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("hsv(0, 100%, 100%)").unwrap();
    /// assert_eq!(red.red, 255);
    /// assert_eq!(red.green, 0);
    /// assert_eq!(red.blue, 0);
    /// assert_eq!(red.alpha, 255);
    ///
    /// let green = Color::new_string("hsv(120°, 100%, 100%)").unwrap();
    /// assert_eq!(green.red, 0);
    /// assert_eq!(green.green, 255);
    /// assert_eq!(green.blue, 0);
    /// assert_eq!(green.alpha, 255);
    ///
    /// let transparent_green = Color::new_string("hsva(120°, 100%, 100%, 0.5)").unwrap();
    /// assert_eq!(transparent_green.red, 0);
    /// assert_eq!(transparent_green.green, 255);
    /// assert_eq!(transparent_green.blue, 0);
    /// assert_eq!(transparent_green.alpha, 128);
    /// ```
    ///
    /// <a name="hwb(a)-notation"></a>
    /// # Example (hwb(a) notation)
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("hwb(0, 0%, 0%)").unwrap();
    /// assert_eq!(red.red, 255);
    /// assert_eq!(red.green, 0);
    /// assert_eq!(red.blue, 0);
    /// assert_eq!(red.alpha, 255);
    ///
    /// let green = Color::new_string("hwb(120°, 0%, 0%)").unwrap();
    /// assert_eq!(green.red, 0);
    /// assert_eq!(green.green, 255);
    /// assert_eq!(green.blue, 0);
    /// assert_eq!(green.alpha, 255);
    ///
    /// let transparent_green = Color::new_string("hwba(120°, 0%, 0%, 0.5)").unwrap();
    /// assert_eq!(transparent_green.red, 0);
    /// assert_eq!(transparent_green.green, 255);
    /// assert_eq!(transparent_green.blue, 0);
    /// assert_eq!(transparent_green.alpha, 128);
    /// ```
    pub fn new_string<S: Into<String>>(string: S) -> Option<Color> {
        let real_string: String = string.into();
        let trimmed_str = real_string.trim();
        let normalized_string = trimmed_str.to_lowercase();
        let normalized_str = normalized_string.as_str();

        Color::try_parse_known_color(normalized_str)
            .or_else(|| Color::try_parse_abbr_color(normalized_str))
            .or_else(|| Color::try_parse_hex(normalized_str))
            .or_else(|| Color::try_parse_css_function(normalized_str))
            .and_then(|color| {
                Some(Color {
                    red: color.red,
                    green: color.green,
                    blue: color.blue,
                    alpha: color.alpha,
                    original_string: real_string,
                })
            })
    }

    /// Gets a new Color struct, that represents a color with the given temperature in kelvin.  
    /// This is based on implementation by [Neil Bartlett](https://github.com/neilbartlett/color-temperature).  
    ///
    /// The effective temperature range goes from 0 to about 30000 Kelvin.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let candle_light = Color::new_temperature(2_000);
    /// let sunset = Color::new_temperature(3_500);
    /// let daylight = Color::new_temperature(6_500);
    ///
    /// assert_eq!(candle_light.to_hex_string(), "#FF8B14");
    /// assert_eq!(sunset.to_hex_string(), "#FFC38A");
    /// assert_eq!(daylight.to_hex_string(), "#FFFAFE");
    /// ```
    pub fn new_temperature(kelvin: u16) -> Color {
        let kelvin = if kelvin > 30_000 {
            30_000.0f64
        } else {
            kelvin as f64
        };
        let temp = kelvin / 100.0;
        let rgb = if temp < 66.0 {
            let mut g = temp - 2.0;
            g = -155.25485562709179 - 0.44596950469579133 * g + 104.49216199393888 * g.ln();
            if g.is_nan() {
                g = 0.0;
            }
            g = g.min(255.0).max(0.0);
            let b = if temp < 20.0 {
                0.0
            } else {
                let mut b = temp - 10.0;
                b = -254.76935184120902 + 0.8274096064007395 * b + 115.67994401066147 * b.ln();
                if b.is_nan() {
                    b = 0.0;
                }
                b.min(255.0).max(0.0)
            };
            (255, g.round() as u8, b.round() as u8)
        } else {
            let mut r = temp - 55.0;
            r = 351.97690566805693 + 0.114206453784165 * r - 40.25366309332127 * r.ln();
            if r.is_nan() {
                r = 0.0;
            }
            r = r.min(255.0).max(0.0).round();
            let mut g = temp - 50.0;
            g = 325.4494125711974 + 0.07943456536662342 * g - 28.0852963507957 * g.ln();
            if g.is_nan() {
                g = 0.0;
            }
            g = g.min(255.0).max(0.0).round();
            (r as u8, g as u8, 255)
        };

        Color::new_rgb(rgb.0, rgb.1, rgb.2)
    }

    /// Gets the original string of the color, if it was called with new_string(...)
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("red").unwrap();
    /// let green = Color::new_string("#00ff00").unwrap();
    ///
    /// assert_eq!("red", red.get_original_string());
    /// assert_eq!("#00ff00", green.get_original_string());
    /// ```
    pub fn get_original_string(&self) -> String {
        self.original_string.clone()
    }

    /// Gets a cmyk tuple of the color.
    ///
    /// This method returns a tuple of the cmyk-components (cyan, magenta, yellow, key) of the color.  
    /// The range of each component is from 0.0 to 1.0, representing the intensity from 0% to 100%.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("red").unwrap();
    /// let red_cmyk = red.get_cmyk();
    ///
    /// assert_eq!(0.0, red_cmyk.0);
    /// assert_eq!(1.0, red_cmyk.1);
    /// assert_eq!(1.0, red_cmyk.2);
    /// assert_eq!(0.0, red_cmyk.3);
    /// ```
    pub fn get_cmyk(&self) -> (f64, f64, f64, f64) {
        let r = self.red as f64 / 255.0;
        let g = self.green as f64 / 255.0;
        let b = self.blue as f64 / 255.0;
        let mut rgb_max = r;
        if g > rgb_max {
            rgb_max = g;
        }
        if b > rgb_max {
            rgb_max = b;
        }

        let black = 1.0 - rgb_max;
        let white = 1.0 - black;
        let cyan = if white != 0.0 {
            ((1.0 - r - black) / white)
        } else {
            0.0
        };
        let magenta = if white != 0.0 {
            ((1.0 - g - black) / white)
        } else {
            0.0
        };
        let yellow = if white != 0.0 {
            ((1.0 - b - black) / white)
        } else {
            0.0
        };

        (cyan, magenta, yellow, black)
    }

    /// Gets a hsla tuple of the color.
    ///
    /// This method returns a tuple of hue, saturation, lightness and alpha of the color.  
    /// The range for hue goes from 0.0 to 360.0 degrees.  
    /// The range for saturation, lightness and alpha goes from 0.0 to 1.0, representing the intensity from 0% to 100%.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let transparent_green = Color::new_string("rgba(0, 255, 0, 0.5)").unwrap();
    /// let transparent_green_hsla = transparent_green.get_hsla();
    ///
    /// assert_eq!(120.0, transparent_green_hsla.0);
    /// assert_eq!(1.0, transparent_green_hsla.1);
    /// assert_eq!(0.5, transparent_green_hsla.2);
    /// assert_eq!(0.5, transparent_green_hsla.3);
    /// ```
    pub fn get_hsla(&self) -> (f64, f64, f64, f64) {
        let r = self.red as f64 / 255.0;
        let g = self.green as f64 / 255.0;
        let b = self.blue as f64 / 255.0;

        let mut c_max = r;
        let mut c_min = r;
        if g > c_max {
            c_max = g;
        }
        if g < c_min {
            c_min = g;
        }
        if b > c_max {
            c_max = b;
        }
        if b < c_min {
            c_min = b;
        }
        let c_delta = c_max - c_min;

        let mut h = 0.0;
        let mut s = 0.0;
        let l = (c_max + c_min) / 2.0;
        if c_delta != 0.0 {
            if c_max == r {
                h = 60.0 * (((g - b) / c_delta) % 6.0);
            } else if c_max == g {
                h = 60.0 * ((b - r) / c_delta + 2.0);
            } else if c_max == b {
                h = 60.0 * ((r - g) / c_delta + 4.0);
            }
            s = c_delta / (1.0 - (2.0 * l - 1.0).abs());
        }

        while h < 0.0 {
            h += 360.0;
        }
        while h > 360.0 {
            h -= 360.0;
        }

        let alpha = round_with_precision(self.alpha as f64 / 255.0, 2);

        (h, s, l, alpha)
    }

    /// Gets a hsva tuple of the color.
    ///
    /// This method returns a tuple of hue, saturation, value and alpha of the color.  
    /// The range for hue goes from 0.0 to 360.0 degrees.  
    /// The range for saturation, value and alpha goes from 0.0 to 1.0, representing the intensity from 0% to 100%.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let transparent_green = Color::new_string("rgba(0, 255, 0, 0.5)").unwrap();
    /// let transparent_green_hsva = transparent_green.get_hsva();
    ///
    /// assert_eq!(120.0, transparent_green_hsva.0);
    /// assert_eq!(1.0, transparent_green_hsva.1);
    /// assert_eq!(1.0, transparent_green_hsva.2);
    /// assert_eq!(0.5, transparent_green_hsva.3);
    /// ```
    pub fn get_hsva(&self) -> (f64, f64, f64, f64) {
        let mut min = 1.0;
        let mut max = 0.0;

        let red = self.red as f64 / 255.0;
        let green = self.green as f64 / 255.0;
        let blue = self.blue as f64 / 255.0;
        let alpha = round_with_precision(self.alpha as f64 / 255.0, 2);

        if red < min {
            min = red;
        }
        if green < min {
            min = green;
        }
        if blue < min {
            min = blue;
        }
        if red > max {
            max = red;
        }
        if green > max {
            max = green;
        }
        if blue > max {
            max = blue;
        }

        if max == 0.0 {
            return (0.0, 0.0, 0.0, alpha);
        }

        let v = max;
        let delta = max - min;
        let s = delta / max;
        let mut h = 0.0;
        if delta != 0.0 {
            h = if red == max {
                (green - blue) / delta
            } else if green == max {
                2.0 + (blue - red) / delta
            } else {
                4.0 + (red - green) / delta
            };

            h *= 60.0;
            if h < 0.0 {
                h += 360.0;
            }
        }

        (h, s, v, alpha)
    }

    /// Gets a hwba tuple of the color.
    ///
    /// This method returns a tuple of hue, whiteness, blackness and alpha of the color.  
    /// The range for hue goes from 0.0 to 360.0 degrees.  
    /// The range for whiteness, blackness and alpha goes from 0.0 to 1.0, representing the intensity from 0% to 100%.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let transparent_green = Color::new_string("rgba(0, 255, 0, 0.5)").unwrap();
    /// let transparent_green_hwba = transparent_green.get_hwba();
    ///
    /// assert_eq!(120.0, transparent_green_hwba.0);
    /// assert_eq!(0.0, transparent_green_hwba.1);
    /// assert_eq!(0.0, transparent_green_hwba.2);
    /// assert_eq!(0.5, transparent_green_hwba.3);
    /// ```
    pub fn get_hwba(&self) -> (f64, f64, f64, f64) {
        let r = self.red as f64 / 255.0;
        let g = self.green as f64 / 255.0;
        let b = self.blue as f64 / 255.0;

        let white = if r <= g && r <= b {
            r
        } else if g <= r && g <= b {
            g
        } else {
            b
        };
        let value = if r >= g && r >= b {
            r
        } else if g >= r && g >= b {
            g
        } else {
            b
        };
        let black = 1.0 - value;
        let f = if r == white {
            g - b
        } else if g == white {
            b - r
        } else {
            r - g
        };
        let i = if r == white {
            3.0
        } else if g == white {
            5.0
        } else {
            1.0
        };

        let mut h = if value - white != 0.0 {
            (i - f / (value - white)) * 60.0
        } else {
            0.0
        };
        if h == 360.0 {
            h = 0.0;
        }

        let alpha = round_with_precision(self.alpha as f64 / 255.0, 2);

        (h, white, black, alpha)
    }

    /// Gets a rgba tuple of the color.
    ///
    /// This method returns a tuple of red, green, blue and alpha of the color.  
    /// The range for red, green, blue and alpha goes from 0.0 to 1.0, representing the intensity from 0% to 100%.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let transparent_green = Color::new_string("rgba(0, 255, 0, 0.5)").unwrap();
    /// let transparent_green_rgba = transparent_green.get_rgba();
    ///
    /// assert_eq!(0.0, transparent_green_rgba.0);
    /// assert_eq!(1.0, transparent_green_rgba.1);
    /// assert_eq!(0.0, transparent_green_rgba.2);
    /// assert_eq!(0.5, transparent_green_rgba.3);
    /// ```
    pub fn get_rgba(&self) -> (f64, f64, f64, f64) {
        let alpha = round_with_precision(self.alpha as f64 / 255.0, 2);

        (
            self.red as f64 / 255.0,
            self.green as f64 / 255.0,
            self.blue as f64 / 255.0,
            alpha,
        )
    }

    fn get_xyz(&self) -> (f64, f64, f64) {
        let r = Color::rgb_xyz(self.red);
        let g = Color::rgb_xyz(self.green);
        let b = Color::rgb_xyz(self.blue);
        let x = Color::xyz_lab(
            (0.4124564 * r + 0.3575761 * g + 0.1804375 * b) / Color::LAB_CONSTANT_XN,
        );
        let y = Color::xyz_lab(
            (0.2126729 * r + 0.7151522 * g + 0.0721750 * b) / Color::LAB_CONSTANT_YN,
        );
        let z = Color::xyz_lab(
            (0.0193339 * r + 0.1191920 * g + 0.9503041 * b) / Color::LAB_CONSTANT_ZN,
        );

        (x, y, z)
    }

    /// Gets a laba tuple of the color.
    ///
    /// This method returns a tuple of lightness, a, b and alpha of the color.  
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let transparent_green = Color::new_string("rgba(0, 255, 0, 0.5)").unwrap();
    /// let transparent_green_laba = transparent_green.get_laba();
    ///
    /// assert_eq!(87.73, transparent_green_laba.0);
    /// assert_eq!(-86.18, transparent_green_laba.1);
    /// assert_eq!(83.18, transparent_green_laba.2);
    /// assert_eq!(0.5, transparent_green_laba.3);
    /// ```
    pub fn get_laba(&self) -> (f64, f64, f64, f64) {
        let xyz = self.get_xyz();
        let mut l = 116.0 * xyz.1 - 16.0;
        if l < 0.0 {
            l = 0.0;
        }

        l = round_with_precision(l, 2);
        let a = round_with_precision(500.0 * (xyz.0 - xyz.1), 2);
        let b = round_with_precision(200.0 * (xyz.1 - xyz.2), 2);
        let alpha = round_with_precision(self.alpha as f64 / 255.0, 2);
        (l, a, b, alpha)
    }

    /// Gets a laba tuple of the color.
    ///
    /// This method returns a tuple of lightness, chroma, hue and alpha of the color.  
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let transparent_green = Color::new_string("rgba(0, 255, 0, 0.5)").unwrap();
    /// let transparent_green_lcha = transparent_green.get_lcha();
    ///
    /// assert_eq!(87.73, transparent_green_lcha.0);
    /// assert_eq!(119.77, transparent_green_lcha.1);
    /// assert_eq!(136.01, transparent_green_lcha.2);
    /// assert_eq!(0.5, transparent_green_lcha.3);
    /// ```
    pub fn get_lcha(&self) -> (f64, f64, f64, f64) {
        let lab = self.get_laba();
        let mut c = (lab.1 * lab.1 + lab.2 * lab.2).sqrt();
        let mut h = (lab.2.atan2(lab.1) * Color::RAD2DEG + 360.0) % 360.0;
        if (c * 10_000.0).round() == 0.0 {
            h = std::f64::NAN; // NaN
        }

        let l = round_with_precision(lab.0, 2);
        c = round_with_precision(c, 2);
        h = round_with_precision(h, 2);
        let alpha = round_with_precision(self.alpha as f64 / 255.0, 2);
        (l, c, h, alpha)
    }

    fn get_rgb_from_cmyk(mut c: f64, mut m: f64, mut y: f64, mut k: f64) -> (u8, u8, u8) {
        if c < 0.0 {
            c = 0.0;
        }
        if c > 1.0 {
            c = 1.0;
        }
        if m < 0.0 {
            m = 0.0;
        }
        if m > 1.0 {
            m = 1.0;
        }
        if y < 0.0 {
            y = 0.0;
        }
        if y > 1.0 {
            y = 1.0;
        }
        if k < 0.0 {
            k = 0.0;
        }
        if k > 1.0 {
            k = 1.0;
        }

        let r = (255.0 * (1.0 - c) * (1.0 - k)).round() as u8;
        let g = (255.0 * (1.0 - m) * (1.0 - k)).round() as u8;
        let b = (255.0 * (1.0 - y) * (1.0 - k)).round() as u8;

        (r, g, b)
    }

    fn get_rgb_from_hsl(mut h: f64, mut s: f64, mut l: f64) -> (u8, u8, u8) {
        if h < 0.0 || h > 360.0 {
            h = ((h % 360.0) + 360.0) % 360.0;
        }
        if s < 0.0 {
            s = 0.0;
        } else if s > 1.0 {
            s = 1.0;
        }
        if l < 0.0 {
            l = 0.0;
        } else if l > 1.0 {
            l = 1.0;
        }

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let mut r1 = 0.0;
        let mut g1 = 0.0;
        let mut b1 = 0.0;
        if (h >= 0.0 && h < 60.0) || h == 360.0 {
            r1 = c;
            g1 = x;
        } else if h >= 60.0 && h < 120.0 {
            r1 = x;
            g1 = c;
        } else if h >= 120.0 && h < 180.0 {
            g1 = c;
            b1 = x;
        } else if h >= 180.0 && h < 240.0 {
            g1 = x;
            b1 = c;
        } else if h >= 240.0 && h < 300.0 {
            r1 = x;
            b1 = c;
        } else if h >= 300.0 && h < 360.0 {
            r1 = c;
            b1 = x;
        }

        let r = ((r1 + m) * 255.0).round() as u8;
        let g = ((g1 + m) * 255.0).round() as u8;
        let b = ((b1 + m) * 255.0).round() as u8;

        (r, g, b)
    }

    fn get_rgb_from_hsv(mut h: f64, mut s: f64, mut v: f64) -> (u8, u8, u8) {
        if h < 0.0 || h > 360.0 {
            h = ((h % 360.0) + 360.0) % 360.0;
        }
        if s < 0.0 {
            s = 0.0;
        } else if s > 1.0 {
            s = 1.0;
        }
        if v < 0.0 {
            v = 0.0;
        } else if v > 1.0 {
            v = 1.0;
        }

        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let mut r1 = 0.0;
        let mut g1 = 0.0;
        let mut b1 = 0.0;
        if (h >= 0.0 && h < 60.0) || h == 360.0 {
            r1 = c;
            g1 = x;
        } else if h >= 60.0 && h < 120.0 {
            r1 = x;
            g1 = c;
        } else if h >= 120.0 && h < 180.0 {
            g1 = c;
            b1 = x;
        } else if h >= 180.0 && h < 240.0 {
            g1 = x;
            b1 = c;
        } else if h >= 240.0 && h < 300.0 {
            r1 = x;
            b1 = c;
        } else if h >= 300.0 && h < 360.0 {
            r1 = c;
            b1 = x;
        }

        let r = ((r1 + m) * 255.0).round() as u8;
        let g = ((g1 + m) * 255.0).round() as u8;
        let b = ((b1 + m) * 255.0).round() as u8;

        (r, g, b)
    }

    fn get_rgb_from_hwb(h: f64, w: f64, b: f64) -> (u8, u8, u8) {
        let v = 1.0 - b;
        let s = 1.0 - (w / v);
        let rgb = Color::get_rgb_from_hsv(h, s, v);

        (rgb.0, rgb.1, rgb.2)
    }

    fn lch_2_lab(l: f64, c: f64, mut h: f64) -> (f64, f64, f64) {
        if h.is_nan() {
            h = 0.0;
        }
        h = h * Color::DEG2RAD;
        (l, h.cos() * c, h.sin() * c)
    }

    fn lab_2_rgb(l: f64, a: f64, b: f64) -> (f64, f64, f64) {
        let mut y = (l + 16.0) / 116.0;
        let mut x = if a.is_nan() { y } else { y + a / 500.0 };
        let mut z = if b.is_nan() { y } else { y - b / 200.0 };

        y = Color::LAB_CONSTANT_YN * Color::lab_xyz(y);
        x = Color::LAB_CONSTANT_XN * Color::lab_xyz(x);
        z = Color::LAB_CONSTANT_ZN * Color::lab_xyz(z);

        let r = Color::xyz_rgb(3.2404542 * x - 1.5371385 * y - 0.4985314 * z); // D65 -> sRGB
        let g = Color::xyz_rgb(-0.9692660 * x + 1.8760108 * y + 0.0415560 * z);
        let b = Color::xyz_rgb(0.0556434 * x - 0.2040259 * y + 1.0572252 * z);

        (r, g, b)
    }

    /// Colorizes this color with another color.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let white = Color::new_string("white").unwrap();
    /// let black = Color::new_string("black").unwrap();
    /// let red = Color::new_string("red").unwrap();
    /// let colorized_red_over_white = white.colorize(red.clone());
    /// let colorized_red_over_black = black.colorize(red.clone());
    ///
    /// assert_eq!("#FF0000", colorized_red_over_white.to_hex_string());
    /// assert_eq!("#000000", colorized_red_over_black.to_hex_string());
    /// ```
    pub fn colorize(&self, color: Color) -> Color {
        Color {
            alpha: (self.alpha as u16 * color.alpha as u16 / 255) as u8,
            red: (self.red as u16 * color.red as u16 / 255) as u8,
            green: (self.green as u16 * color.green as u16 / 255) as u8,
            blue: (self.blue as u16 * color.blue as u16 / 255) as u8,
            ..Default::default()
        }
    }

    /// Colorizes this color with another color.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let white = Color::new_string("white").unwrap();
    /// let black = Color::new_string("black").unwrap();
    /// let colorized_red_over_white = white.colorize_string("red").unwrap();
    /// let colorized_red_over_black = black.colorize_string("red").unwrap();
    ///
    /// assert_eq!("#FF0000", colorized_red_over_white.to_hex_string());
    /// assert_eq!("#000000", colorized_red_over_black.to_hex_string());
    /// ```
    pub fn colorize_string(&self, color: &str) -> Result<Color, &str> {
        match Color::new_string(color) {
            Some(color) => Ok(self.colorize(color)),
            None => Err("unable to parse color to colorize."),
        }
    }

    /// Gets a brightened color by a specified amount.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("#ff0000").unwrap();
    /// let red_brightened_1 = red.brighten(1.0);
    /// let red_brightened_10 = red.brighten(10.0);
    ///
    /// assert_eq!(red_brightened_1.to_hex_string(), "#FF5A36");
    /// assert_eq!(red_brightened_10.to_hex_string(), "#FFFFFF");
    /// ```
    pub fn brighten(&self, amount: f64) -> Color {
        self.darken(-amount)
    }

    /// Gets a darkened color by a specified amount.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("#ff0000").unwrap();
    /// let red_darkened_1 = red.darken(1.0);
    /// let red_darkened_10 = red.darken(10.0);
    ///
    /// assert_eq!(red_darkened_1.to_hex_string(), "#C20000");
    /// assert_eq!(red_darkened_10.to_hex_string(), "#000000");
    /// ```
    pub fn darken(&self, amount: f64) -> Color {
        let laba = self.get_lcha();
        let new_l = laba.0 - Color::LAB_CONSTANT_KN * amount;

        Color::new_lcha(new_l, laba.1, laba.2, laba.3)
    }

    /// Gets a grayscaled color from the color.
    ///
    /// This method uses the default formula used by PAL and NTSC systems.  
    /// `Y = 0.299 * R + 0.587 * G + 0.114 * B`
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("rgb(255, 0, 0)").unwrap();
    /// let grayscaled_red = red.grayscale();
    ///
    /// assert_eq!(76, grayscaled_red.red);
    /// assert_eq!(76, grayscaled_red.green);
    /// assert_eq!(76, grayscaled_red.blue);
    /// assert_eq!(255, grayscaled_red.alpha);
    /// ```
    pub fn grayscale(&self) -> Color {
        let gray_value = (self.red as f64 * 0.299
            + self.green as f64 * 0.587
            + self.blue as f64 * 0.114)
            .round() as u8;
        Color {
            red: gray_value,
            green: gray_value,
            blue: gray_value,
            alpha: self.alpha,
            ..Default::default()
        }
    }

    /// Gets a grayscaled color from the color.
    ///
    /// This method uses the default formula used by HDTV systems.  
    /// `Y = 0.2126 * R + 0.7152 * G + 0.0722 * B`
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("rgb(255, 0, 0)").unwrap();
    /// let grayscaled_red = red.grayscale_hdtv();
    ///
    /// assert_eq!(54, grayscaled_red.red);
    /// assert_eq!(54, grayscaled_red.green);
    /// assert_eq!(54, grayscaled_red.blue);
    /// assert_eq!(255, grayscaled_red.alpha);
    /// ```
    pub fn grayscale_hdtv(&self) -> Color {
        let gray_value =
            (self.red as f64 * 0.2126 + self.green as f64 * 0.7152 + self.blue as f64 * 0.0722)
                .round() as u8;
        Color {
            red: gray_value,
            green: gray_value,
            blue: gray_value,
            alpha: self.alpha,
            ..Default::default()
        }
    }

    /// Gets a grayscaled color from the color.
    ///
    /// This method uses the default formula used by HDTV systems.  
    /// `Y = 0.2627 * R + 0.678 * G + 0.0593 * B`
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("rgb(255, 0, 0)").unwrap();
    /// let grayscaled_red = red.grayscale_hdr();
    ///
    /// assert_eq!(67, grayscaled_red.red);
    /// assert_eq!(67, grayscaled_red.green);
    /// assert_eq!(67, grayscaled_red.blue);
    /// assert_eq!(255, grayscaled_red.alpha);
    /// ```
    pub fn grayscale_hdr(&self) -> Color {
        let gray_value =
            (self.red as f64 * 0.2627 + self.green as f64 * 0.678 + self.blue as f64 * 0.0593)
                .round() as u8;
        Color {
            red: gray_value,
            green: gray_value,
            blue: gray_value,
            alpha: self.alpha,
            ..Default::default()
        }
    }

    /// Gets a monochromed (black or white) color from the color.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let darker_gray = Color::new_string("rgb(100, 100, 100)").unwrap();
    /// let lighter_gray = Color::new_string("rgb(200, 200, 200)").unwrap();
    /// let black = darker_gray.monochrome();
    /// let white = lighter_gray.monochrome();
    ///
    /// assert_eq!(0, black.red);
    /// assert_eq!(0, black.green);
    /// assert_eq!(0, black.blue);
    /// assert_eq!(255, black.alpha);
    ///
    /// assert_eq!(255, white.red);
    /// assert_eq!(255, white.green);
    /// assert_eq!(255, white.blue);
    /// assert_eq!(255, white.alpha);
    /// ```
    pub fn monochrome(&self) -> Color {
        let grayscaled = self.grayscale();
        if grayscaled.red < 128 {
            Color {
                red: 0,
                green: 0,
                blue: 0,
                alpha: grayscaled.alpha,
                ..Default::default()
            }
        } else {
            Color {
                red: 255,
                green: 255,
                blue: 255,
                alpha: grayscaled.alpha,
                ..Default::default()
            }
        }
    }

    /// Gets the inverted color of a color.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let black = Color::new_string("#000000").unwrap();
    /// let black_inverted = black.invert();
    ///
    /// assert_eq!("#FFFFFF", black_inverted.to_hex_string());
    /// ```
    pub fn invert(&self) -> Color {
        Color {
            red: 255 - self.red,
            green: 255 - self.green,
            blue: 255 - self.blue,
            alpha: self.alpha,
            ..Default::default()
        }
    }

    /// Gets the inverted luminescenced color of a color.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let dark_green = Color::new_hsla(120.0, 1.0, 0.3, 1.0);
    /// let light_green = dark_green.invert_luminescence();
    ///
    /// assert_eq!("#009900", dark_green.to_hex_string());
    /// assert_eq!("#66FF66", light_green.to_hex_string());
    /// ```
    pub fn invert_luminescence(&self) -> Color {
        let hsla = self.get_hsla();
        Color::new_hsla(hsla.0, hsla.1, 1.0 - hsla.2, hsla.3)
    }

    fn luminance_x(x: u8) -> f64 {
        let x = x as f64 / 255.0;
        if x <= 0.03928 {
            x / 12.92
        } else {
            ((x + 0.055) / 1.055).powf(2.4)
        }
    }

    /// Gets the relative luminance of the Color as defined in [WCAG 2.0](https://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef)
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let white = Color::new_string("white").unwrap();
    /// let aquamarine = Color::new_string("aquamarine").unwrap();
    /// let hotpink = Color::new_string("hotpink").unwrap();
    /// let darkslateblue = Color::new_string("darkslateblue").unwrap();
    /// let black = Color::new_string("black").unwrap();
    ///
    /// assert_eq!(white.get_luminance(), 1.0);
    /// assert_eq!(aquamarine.get_luminance(), 0.8078549208338043);
    /// assert_eq!(hotpink.get_luminance(), 0.3465843816971475);
    /// assert_eq!(darkslateblue.get_luminance(), 0.06579284622798763);
    /// assert_eq!(black.get_luminance(), 0.0);
    /// ```
    pub fn get_luminance(&self) -> f64 {
        let r = Self::luminance_x(self.red);
        let g = Self::luminance_x(self.green);
        let b = Self::luminance_x(self.blue);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Computes the [WCAG contrast ratio](https://www.w3.org/TR/2008/REC-WCAG20-20081211/#contrast-ratiodef) between two colors. \
    /// A minimum contrast of 4.5:1 [is recommended](https://www.w3.org/TR/WCAG20-TECHS/G18.html) to ensure that text is still readable against a background color.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let pink = Color::new_string("pink").unwrap();
    /// let hotpink = Color::new_string("hotpink").unwrap();
    /// let purple = Color::new_string("purple").unwrap();
    ///
    /// assert_eq!(pink.get_contrast(hotpink), 1.7214765344592284);
    /// assert_eq!(pink.get_contrast(purple), 6.124225406859997);
    /// ```
    pub fn get_contrast(&self, color: Color) -> f64 {
        let l1 = self.get_luminance();
        let l2 = color.get_luminance();
        if l1 > l2 {
            (l1 + 0.05) / (l2 + 0.05)
        } else {
            (l2 + 0.05) / (l1 + 0.05)
        }
    }

    /// Gets a formatted cmyk String of the color as used in css.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("red").unwrap();
    ///
    /// assert_eq!("cmyk(0%, 100%, 100%, 0%)", red.to_cmyk_string());
    /// ```
    pub fn to_cmyk_string(&self) -> String {
        let cmyk = self.get_cmyk();

        format!(
            "cmyk({}%, {}%, {}%, {}%)",
            (cmyk.0 * 100.0).round(),
            (cmyk.1 * 100.0).round(),
            (cmyk.2 * 100.0).round(),
            (cmyk.3 * 100.0).round()
        )
    }

    /// Gets a formatted hex String of the color as used in css.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("red").unwrap();
    ///
    /// assert_eq!("gray(76)", red.to_gray_string());
    /// ```
    pub fn to_gray_string(&self) -> String {
        let gray = self.grayscale();
        let mut gray_string = format!("gray({}", gray.red);
        if gray.alpha != 255 {
            gray_string.push_str(format!(", {}", gray.alpha).as_str());
        }
        gray_string.push_str(")");
        gray_string
    }

    /// Gets a formatted hex String of the color as used in css.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("red").unwrap();
    /// let transparent_green = Color::new_string("rgba(0, 255, 0, 0.5)").unwrap();
    ///
    /// assert_eq!("#FF0000", red.to_hex_string());
    /// assert_eq!("#00FF0080", transparent_green.to_hex_string());
    /// ```
    pub fn to_hex_string(&self) -> String {
        let mut hex = String::from("#");
        hex.push_str(format!("{:01$X}", self.red, 2).as_str());
        hex.push_str(format!("{:01$X}", self.green, 2).as_str());
        hex.push_str(format!("{:01$X}", self.blue, 2).as_str());
        if self.alpha != 255 {
            hex.push_str(format!("{:01$X}", self.alpha, 2).as_str());
        }
        hex
    }

    /// Gets a formatted hsl String of the color as used in css.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("red").unwrap();
    /// let transparent_green = Color::new_string("rgba(0, 255, 0, 0.5)").unwrap();
    ///
    /// assert_eq!("hsl(0, 100%, 50%)", red.to_hsl_string());
    /// assert_eq!("hsla(120, 100%, 50%, 0.5)", transparent_green.to_hsl_string());
    /// ```
    pub fn to_hsl_string(&self) -> String {
        let hsla = self.get_hsla();
        let h_rounded = round_with_precision(hsla.0, 2);
        let s_rounded = round_with_precision(hsla.1 * 100.0, 2);
        let l_rounded = round_with_precision(hsla.2 * 100.0, 2);

        let mut hsl_string = String::from("hsl");
        if self.alpha != 255 {
            hsl_string.push_str("a");
        }
        hsl_string.push_str("(");
        hsl_string.push_str(format!("{}, {}%, {}%", h_rounded, s_rounded, l_rounded).as_str());
        if self.alpha != 255 {
            hsl_string.push_str(format!(", {}", round_with_precision(hsla.3, 2)).as_str());
        }
        hsl_string.push_str(")");
        hsl_string
    }

    /// Gets a formatted hsv String of the color as used in css.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("red").unwrap();
    /// let transparent_green = Color::new_string("rgba(0, 255, 0, 0.5)").unwrap();
    ///
    /// assert_eq!("hsv(0, 100%, 100%)", red.to_hsv_string());
    /// assert_eq!("hsva(120, 100%, 100%, 0.5)", transparent_green.to_hsv_string());
    /// ```
    pub fn to_hsv_string(&self) -> String {
        let hsva = self.get_hsva();
        let h_rounded = round_with_precision(hsva.0, 2);
        let s_rounded = round_with_precision(hsva.1 * 100.0, 2);
        let v_rounded = round_with_precision(hsva.2 * 100.0, 2);

        let mut hsv_string = String::from("hsv");
        if hsva.3 != 1.0 {
            hsv_string.push_str("a");
        }
        hsv_string.push_str("(");
        hsv_string.push_str(format!("{}, {}%, {}%", h_rounded, s_rounded, v_rounded).as_str());
        if hsva.3 != 1.0 {
            hsv_string.push_str(format!(", {}", round_with_precision(hsva.3, 2)).as_str());
        }
        hsv_string.push_str(")");
        hsv_string
    }

    /// Gets a formatted hwb String of the color as used in css.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("red").unwrap();
    /// let transparent_green = Color::new_string("rgba(0, 255, 0, 0.5)").unwrap();
    ///
    /// assert_eq!("hwb(0, 0%, 0%)", red.to_hwb_string());
    /// assert_eq!("hwba(120, 0%, 0%, 0.5)", transparent_green.to_hwb_string());
    /// ```
    pub fn to_hwb_string(&self) -> String {
        let hwba = self.get_hwba();
        let h_rounded = hwba.0.round() as u16;
        let w_rounded = round_with_precision(hwba.1 * 100.0, 2);
        let b_rounded = round_with_precision(hwba.2 * 100.0, 2);

        let mut hwb_string = String::from("hwb");
        if self.alpha != 255 {
            hwb_string.push_str("a");
        }
        hwb_string.push_str("(");
        hwb_string.push_str(format!("{}, {}%, {}%", h_rounded, w_rounded, b_rounded).as_str());
        if self.alpha != 255 {
            // round with a precision of 2 decimals.
            hwb_string.push_str(format!(", {}", round_with_precision(hwba.3, 2)).as_str());
        }
        hwb_string.push_str(")");
        hwb_string
    }

    /// Gets a formatted rgb String of the color as used in css.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("red").unwrap();
    /// let transparent_green = Color::new_string("rgba(0, 255, 0, 0.5)").unwrap();
    ///
    /// assert_eq!("rgb(255, 0, 0)", red.to_rgb_string());
    /// assert_eq!("rgba(0, 255, 0, 0.5)", transparent_green.to_rgb_string());
    /// ```
    pub fn to_rgb_string(&self) -> String {
        let mut rgb = String::from("rgb");
        if self.alpha != 255 {
            rgb.push_str("a");
        }
        rgb.push_str("(");
        rgb.push_str(format!("{}, {}, {}", self.red, self.green, self.blue).as_str());
        if self.alpha != 255 {
            rgb.push_str(
                format!(", {}", round_with_precision(self.alpha as f64 / 255.0, 2)).as_str(),
            );
        }
        rgb.push_str(")");

        rgb
    }

    /// Converts the Color-struct to an i32 number.  
    /// This conversion is made like the [dotnet](https://docs.microsoft.com/de-de/dotnet/api/system.drawing.color.toargb?view=netframework-4.7.2) version.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("red").unwrap();
    /// let red_i32 = red.to_number();
    ///
    /// assert_eq!(-65536, red_i32);
    /// ```
    pub fn to_number(&self) -> i32 {
        let mut numbered_color = self.blue as i32;
        numbered_color += (self.green as i32) << 8;
        numbered_color += (self.red as i32) << 16;
        numbered_color += (self.alpha as i32) << 24;

        numbered_color
    }

    /// Converts the Color-struct to an u16 number, that represents the color-temperature.  
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let candle_light = Color::new_string("#FF8B14").unwrap();
    /// let sunset = Color::new_string("#FFC38A").unwrap();
    /// let daylight = Color::new_string("#FFFAFE").unwrap();
    ///
    /// // differences in the conversion from temperature to color comes,  
    /// // because of rounding of the red, green and blue values.
    /// assert_eq!(2_000, candle_light.to_temperature());
    /// assert_eq!(3_486, sunset.to_temperature());
    /// assert_eq!(6_473, daylight.to_temperature());
    /// ```
    pub fn to_temperature(&self) -> u16 {
        let r = self.red as f64;
        let b = self.blue as f64;
        let mut min_temp = 1_000.0f64;
        let mut max_temp = 40_000.0f64;
        let eps = 0.4f64;
        let mut temp = 0.0f64;
        while (max_temp - min_temp) > eps {
            temp = (max_temp + min_temp) * 0.5;
            let rgb = Color::new_temperature(temp as u16);
            if (rgb.blue as f64 / rgb.red as f64) >= (b / r) {
                max_temp = temp;
            } else {
                min_temp = temp;
            }
        }

        temp.round() as u16
    }

    /// Gets an interpolated Color-struct from the current to the final color by an interpolation factor.
    /// The interpolation is made by the rgb values.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let white = Color::new_string("white").unwrap();
    /// let black = Color::new_string("black").unwrap();
    /// let gray = white.interpolate(black, 0.5);
    ///
    /// assert_eq!("rgb(128, 128, 128)", gray.to_rgb_string());
    /// ```
    pub fn interpolate(&self, color: Color, interpolation: f64) -> Color {
        let i = if interpolation < 0.0 {
            0.0
        } else if interpolation > 1.0 {
            1.0
        } else {
            interpolation
        };

        Color {
            red: (self.red as f64 + (color.red as i16 - self.red as i16) as f64 * i).round() as u8,
            green: (self.green as f64 + (color.green as i16 - self.green as i16) as f64 * i).round()
                as u8,
            blue: (self.blue as f64 + (color.blue as i16 - self.blue as i16) as f64 * i).round()
                as u8,
            alpha: (self.alpha as f64 + (color.alpha as i16 - self.alpha as i16) as f64 * i).round()
                as u8,
            ..Default::default()
        }
    }

    /// Gets an interpolated Color-struct from the current to the final color by an interpolation factor.
    /// The interpolation is made by the hsv values.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let white = Color::new_string("white").unwrap();
    /// let black = Color::new_string("black").unwrap();
    /// let gray = white.interpolate_hsv(black, 0.5);
    ///
    /// assert_eq!("rgb(128, 128, 128)", gray.to_rgb_string());
    /// ```
    pub fn interpolate_hsv(&self, color: Color, interpolation: f64) -> Color {
        let i = if interpolation < 0.0 {
            0.0
        } else if interpolation > 1.0 {
            1.0
        } else {
            interpolation
        };

        let hsva = self.get_hsva();
        let first_h = hsva.0;
        let first_s = hsva.1;
        let first_v = hsva.2;

        let second_hsva = color.get_hsva();
        let second_h = second_hsva.0;
        let second_s = second_hsva.1;
        let second_v = second_hsva.2;

        let new_h = first_h + (second_h - first_h) * i;
        let new_s = first_s + (second_s - first_s) * i;
        let new_v = first_v + (second_v - first_v) * i;
        let new_a = self.alpha as f64 + (color.alpha as i16 - self.alpha as i16) as f64 * i / 255.0;

        Color::new_hsva(new_h, new_s, new_v, new_a)
    }

    /// Gets an interpolated Color-struct from the current to the final color by an interpolation factor.
    /// The interpolation is made by the hsl values.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let white = Color::new_string("white").unwrap();
    /// let black = Color::new_string("black").unwrap();
    /// let gray = white.interpolate_hsl(black, 0.5);
    ///
    /// assert_eq!("rgb(128, 128, 128)", gray.to_rgb_string());
    /// ```
    pub fn interpolate_hsl(&self, color: Color, interpolation: f64) -> Color {
        let i = if interpolation < 0.0 {
            0.0
        } else if interpolation > 1.0 {
            1.0
        } else {
            interpolation
        };

        let hsla = self.get_hsla();
        let first_h = hsla.0;
        let first_s = hsla.1;
        let first_l = hsla.2;

        let second_hsla = color.get_hsla();
        let second_h = second_hsla.0;
        let second_s = second_hsla.1;
        let second_l = second_hsla.2;

        let new_h = first_h + (second_h - first_h) * i;
        let new_s = first_s + (second_s - first_s) * i;
        let new_l = first_l + (second_l - first_l) * i;
        let new_a = self.alpha as f64 + (color.alpha as i16 - self.alpha as i16) as f64 * i / 255.0;

        Color::new_hsla(new_h, new_s, new_l, new_a)
    }

    /// Gets an interpolated Color-struct from the current to the final color by an interpolation factor.
    /// The interpolation is made by the hwb values.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let white = Color::new_string("white").unwrap();
    /// let black = Color::new_string("black").unwrap();
    /// let gray = white.interpolate_hwb(black, 0.5);
    ///
    /// assert_eq!("rgb(128, 128, 128)", gray.to_rgb_string());
    /// ```
    pub fn interpolate_hwb(&self, color: Color, interpolation: f64) -> Color {
        let i = if interpolation < 0.0 {
            0.0
        } else if interpolation > 1.0 {
            1.0
        } else {
            interpolation
        };

        let hwba = self.get_hwba();
        let first_h = hwba.0;
        let first_w = hwba.1;
        let first_b = hwba.2;

        let second_hwba = color.get_hwba();
        let second_h = second_hwba.0;
        let second_w = second_hwba.1;
        let second_b = second_hwba.2;

        let new_h = first_h + (second_h - first_h) * interpolation;
        let new_s = first_w + (second_w - first_w) * interpolation;
        let new_l = first_b + (second_b - first_b) * interpolation;
        let new_a = self.alpha as f64 + (color.alpha as i16 - self.alpha as i16) as f64 * i / 255.0;

        Color::new_hwba(new_h, new_s, new_l, new_a)
    }

    /// Gets an interpolated Color-struct from the current to the final color by an interpolation factor.
    /// The interpolation is made by the lch values.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let white = Color::new_string("white").unwrap();
    /// let black = Color::new_string("black").unwrap();
    /// let gray = white.interpolate_lch(black, 0.5);
    ///
    /// assert_eq!("rgb(119, 119, 119)", gray.to_rgb_string());
    /// ```
    pub fn interpolate_lch(&self, color: Color, interpolation: f64) -> Color {
        let i = if interpolation < 0.0 {
            0.0
        } else if interpolation > 1.0 {
            1.0
        } else {
            interpolation
        };

        let lch = self.get_lcha();
        let first_l = lch.0;
        let first_c = lch.1;
        let first_h = lch.2;

        let second_lch = color.get_lcha();
        let second_l = second_lch.0;
        let second_c = second_lch.1;
        let second_h = second_lch.2;

        let new_h = if !first_h.is_nan() && !second_h.is_nan() {
            let dh = if second_h > first_h && second_h - first_h > 180.0 {
                second_h - (first_h + 360.0)
            } else if second_h < first_h && first_h - second_h > 180.0 {
                second_h + 360.0 - first_h
            } else {
                second_h - first_h
            };
            first_h + i * dh
        } else if !first_h.is_nan() {
            first_h
        } else if !second_h.is_nan() {
            second_h
        } else {
            std::f64::NAN
        };

        let new_l = first_l + (second_l - first_l) * i;
        let new_c = first_c + (second_c - first_c) * i;
        let new_a = self.alpha as f64 + (color.alpha as i16 - self.alpha as i16) as f64 * i / 255.0;

        Color::new_lcha(new_l, new_c, new_h, new_a)
    }

    fn try_parse_hex(string: &str) -> Option<Color> {
        lazy_static! {
            static ref RE_HEX: Regex = Regex::new(r"^#?([0-9a-f]{3,8})$").unwrap();
        }
        let caps = RE_HEX.captures(string);
        match caps {
            Some(cap) => {
                if cap[1].len() == 5 || cap[1].len() == 7 {
                    return None;
                }

                let has_alpha = if cap[1].len() == 4 || cap[1].len() == 8 {
                    true
                } else {
                    false
                };
                let expand_values = if cap[1].len() == 3 || cap[1].len() == 4 {
                    true
                } else {
                    false
                };

                let mut r_hex: String;
                let mut g_hex: String;
                let mut b_hex: String;
                let mut a_hex = String::from("ff");
                if expand_values {
                    r_hex = String::from(&cap[1][0..1]);
                    let r_hex_cloned = r_hex.clone();
                    r_hex.push_str(&r_hex_cloned);

                    g_hex = String::from(&cap[1][1..2]);
                    let g_hex_cloned = g_hex.clone();
                    g_hex.push_str(&g_hex_cloned);

                    b_hex = String::from(&cap[1][2..3]);
                    let b_hex_cloned = b_hex.clone();
                    b_hex.push_str(&b_hex_cloned);

                    if has_alpha {
                        a_hex = String::from(&cap[1][3..4]);
                        let a_hex_cloned = a_hex.clone();
                        a_hex.push_str(&a_hex_cloned);
                    }
                } else {
                    r_hex = String::from(&cap[1][0..2]);
                    g_hex = String::from(&cap[1][2..4]);
                    b_hex = String::from(&cap[1][4..6]);
                    if has_alpha {
                        a_hex = String::from(&cap[1][6..8]);
                    }
                }

                let r = u8::from_str_radix(r_hex.as_str(), 16).unwrap();
                let g = u8::from_str_radix(g_hex.as_str(), 16).unwrap();
                let b = u8::from_str_radix(b_hex.as_str(), 16).unwrap();
                let a = u8::from_str_radix(a_hex.as_str(), 16).unwrap();

                Some(Color::new_rgba(r, g, b, a))
            }
            None => None,
        }
    }

    fn try_parse_css_function(string: &str) -> Option<Color> {
        lazy_static! {
            // cap[1] -> css-function
            // cap[2] -> 1. value
            // cap[3] -> 1. value after dot
            // cap[4] -> unit of 1. value (° or % or empty)
            // cap[5] -> 2., 3. and 4. value
            // cap[6] -> 2. value
            // cap[7] -> 2. value after dot
            // cap[8] -> unit of 2. value (% or empty)
            // cap[9] -> 3. and 4. value
            // cap[10] -> 3. value
            // cap[11] -> 3. value after dot
            // cap[12] -> unit of 3. value (% or empty)
            // cap[13] -> whole 4. value
            // cap[14] -> 4. value
            // cap[15] -> 4. value after dot
            // cap[16] -> unit of 4. value
            static ref RE_CSS_FUNCTION: Regex = Regex::new(r"^(cmyk|gray|grey|hsla?|hsva?|hwba?|rgba?)\s*\(\s*(-?\d+(\.\d+)?)\s*(%|°)?\s*(,\s*(-?\d+(\.\d+)?)\s*(%)?\s*(,\s*(-?\d+(\.\d+)?)\s*(%)?\s*(,\s*(-?\d+(\.\d+)?)\s*(%)?\s*)?)?)?\)$").unwrap();
        }
        let caps = RE_CSS_FUNCTION.captures(string);
        if caps.is_none() {
            return None;
        }

        let cap = caps.unwrap();
        let css_function = &cap[1];
        let mut force_alpha = false;
        let css_base_function = match css_function {
            "cmyk" => "cmyk",
            "gray" => "gray",
            "grey" => "gray",
            "rgb" => "rgb",
            "rgba" => {
                force_alpha = true;
                "rgb"
            }
            "hsl" => "hsl",
            "hsla" => {
                force_alpha = true;
                "hsl"
            }
            "hsv" => "hsv",
            "hsva" => {
                force_alpha = true;
                "hsv"
            }
            "hwb" => "hwb",
            "hwba" => {
                force_alpha = true;
                "hwb"
            }
            _ => "",
        };

        let mut value_1: f64 = String::from(&cap[2]).parse().unwrap();
        let value_2_opt = if cap.get(6).is_some() && cap[6].len() > 0 {
            let float: f64 = String::from(&cap[6]).parse().unwrap();
            Some(float)
        } else {
            None
        };
        let value_3_opt = if cap.get(10).is_some() && cap[10].len() > 0 {
            let float: f64 = String::from(&cap[10]).parse().unwrap();
            Some(float)
        } else {
            None
        };
        let value_4_opt = if cap.get(14).is_some() && cap[14].len() > 0 {
            let float: f64 = String::from(&cap[14]).parse().unwrap();
            Some(float)
        } else {
            None
        };

        let get_alpha = |alpha_option: Option<f64>, is_percentage: bool| -> Option<u8> {
            if alpha_option.is_some() {
                let mut alpha = alpha_option.unwrap();
                if alpha < 0.0 {
                    alpha = 0.0;
                } else if is_percentage && alpha > 100.0 {
                    alpha = 100.0;
                } else if !is_percentage && alpha > 1.0 {
                    alpha = 1.0;
                }
                if is_percentage {
                    alpha /= 100.0;
                }

                Some((alpha * 255.0).round() as u8)
            } else {
                if force_alpha {
                    return None;
                }
                Some(255)
            }
        };

        match css_base_function {
            "cmyk" => {
                if value_2_opt.is_none() || value_3_opt.is_none() || value_4_opt.is_none() {
                    return None;
                }
                if &cap[4] == "°" {
                    return None;
                }

                let value_2 = value_2_opt.unwrap();
                let value_3 = value_3_opt.unwrap();
                let value_4 = value_4_opt.unwrap();

                let rgb = Color::get_rgb_from_cmyk(
                    value_1 / 100.0,
                    value_2 / 100.0,
                    value_3 / 100.0,
                    value_4 / 100.0,
                );
                Some(Color::new_rgb(rgb.0, rgb.1, rgb.2))
            }
            "rgb" => {
                if value_2_opt.is_none() || value_3_opt.is_none() {
                    return None;
                }
                if cap.get(4).is_some() && &cap[4] == "°" {
                    return None;
                }
                let mut value_2 = value_2_opt.unwrap();
                let mut value_3 = value_3_opt.unwrap();
                let is_in_percentage_mode = if cap.get(4).is_some() && &cap[4] == "%" {
                    true
                } else {
                    false
                };
                if is_in_percentage_mode {
                    if &cap[8] != "%" || &cap[12] != "%" {
                        return None;
                    }
                }
                if value_1 < 0.0 {
                    value_1 = 0.0;
                }
                if value_2 < 0.0 {
                    value_2 = 0.0;
                }
                if value_3 < 0.0 {
                    value_3 = 0.0;
                }

                let alpha_opt = get_alpha(value_4_opt, false);
                if alpha_opt.is_none() {
                    return None;
                }

                let rgb = if is_in_percentage_mode {
                    value_1 /= 100.0;
                    value_2 /= 100.0;
                    value_3 /= 100.0;
                    if value_1 > 1.0 {
                        value_1 = 1.0;
                    }
                    if value_2 > 1.0 {
                        value_2 = 1.0;
                    }
                    if value_3 > 1.0 {
                        value_3 = 1.0;
                    }

                    (
                        (value_1 * 255.0).round() as u8,
                        (value_2 * 255.0).round() as u8,
                        (value_3 * 255.0).round() as u8,
                    )
                } else {
                    if value_1 > 255.0 {
                        value_1 = 255.0;
                    }
                    if value_2 > 255.0 {
                        value_2 = 255.0;
                    }
                    if value_3 > 255.0 {
                        value_3 = 255.0;
                    }

                    (
                        value_1.round() as u8,
                        value_2.round() as u8,
                        value_3.round() as u8,
                    )
                };

                Some(Color::new_rgba(rgb.0, rgb.1, rgb.2, alpha_opt.unwrap()))
            }
            "hsl" => {
                if value_2_opt.is_none() || value_3_opt.is_none() {
                    return None;
                }
                if cap.get(4).is_some() && &cap[4] == "%" {
                    return None;
                }

                let value_2 = value_2_opt.unwrap();
                let value_3 = value_3_opt.unwrap();
                let alpha_opt = get_alpha(value_4_opt, false);
                if alpha_opt.is_none() {
                    return None;
                }

                let rgb = Color::get_rgb_from_hsl(value_1, value_2 / 100.0, value_3 / 100.0);

                Some(Color::new_rgba(rgb.0, rgb.1, rgb.2, alpha_opt.unwrap()))
            }
            "hsv" => {
                if value_2_opt.is_none() || value_3_opt.is_none() {
                    return None;
                }
                if cap.get(4).is_some() && &cap[4] == "%" {
                    return None;
                }

                let value_2 = value_2_opt.unwrap();
                let value_3 = value_3_opt.unwrap();
                let alpha_opt = get_alpha(value_4_opt, false);
                if alpha_opt.is_none() {
                    return None;
                }

                let rgb = Color::get_rgb_from_hsv(value_1, value_2 / 100.0, value_3 / 100.0);

                Some(Color::new_rgba(rgb.0, rgb.1, rgb.2, alpha_opt.unwrap()))
            }
            "hwb" => {
                if value_2_opt.is_none() || value_3_opt.is_none() {
                    return None;
                }
                if cap.get(4).is_some() && &cap[4] == "%" {
                    return None;
                }

                let value_2 = value_2_opt.unwrap();
                let value_3 = value_3_opt.unwrap();
                let alpha_opt = get_alpha(value_4_opt, false);
                if alpha_opt.is_none() {
                    return None;
                }

                let rgb = Color::get_rgb_from_hwb(value_1, value_2 / 100.0, value_3 / 100.0);

                Some(Color::new_rgba(rgb.0, rgb.1, rgb.2, alpha_opt.unwrap()))
            }
            "gray" => {
                let is_in_percentage_mode = if cap.get(4).is_some() && &cap[4] == "%" {
                    true
                } else {
                    false
                };
                let alpha_is_in_percentage_mode = if cap.get(8).is_some() && &cap[8] == "%" {
                    true
                } else {
                    false
                };
                if value_1 < 0.0 {
                    value_1 = 0.0;
                }
                if is_in_percentage_mode && value_1 > 100.0 {
                    value_1 = 100.0;
                } else if !is_in_percentage_mode && value_1 > 255.0 {
                    value_1 = 255.0;
                };
                let gray_value = if is_in_percentage_mode {
                    (value_1 / 100.0 * 255.0).round() as u8
                } else {
                    value_1.round() as u8
                };

                let alpha = get_alpha(value_2_opt, alpha_is_in_percentage_mode).unwrap_or(255);

                Some(Color::new_rgba(gray_value, gray_value, gray_value, alpha))
            }
            _ => None,
        }
    }

    fn try_parse_abbr_color(string: &str) -> Option<Color> {
        match string {
            "bk" => Some(Color {
                red: 0x00,
                green: 0x00,
                blue: 0x00,
                ..Default::default()
            }),
            "wh" => Some(Color {
                red: 0xFF,
                green: 0xFF,
                blue: 0xFF,
                ..Default::default()
            }),
            "gr" => Some(Color {
                red: 0x80,
                green: 0x80,
                blue: 0x80,
                ..Default::default()
            }),
            "si" => Some(Color {
                red: 0xC0,
                green: 0xC0,
                blue: 0xC0,
                ..Default::default()
            }),
            "mr" => Some(Color {
                red: 0x80,
                green: 0x00,
                blue: 0x00,
                ..Default::default()
            }),
            "rd" => Some(Color {
                red: 0xFF,
                green: 0x00,
                blue: 0x00,
                ..Default::default()
            }),
            "gn" => Some(Color {
                red: 0x00,
                green: 0x80,
                blue: 0x00,
                ..Default::default()
            }),
            "li" => Some(Color {
                red: 0x00,
                green: 0xFF,
                blue: 0x00,
                ..Default::default()
            }),
            "ol" => Some(Color {
                red: 0x80,
                green: 0x80,
                blue: 0x00,
                ..Default::default()
            }),
            "ye" => Some(Color {
                red: 0xFF,
                green: 0xFF,
                blue: 0x00,
                ..Default::default()
            }),
            "na" => Some(Color {
                red: 0x00,
                green: 0x00,
                blue: 0x80,
                ..Default::default()
            }),
            "bl" => Some(Color {
                red: 0x00,
                green: 0x00,
                blue: 0xFF,
                ..Default::default()
            }),
            "pu" => Some(Color {
                red: 0x80,
                green: 0x00,
                blue: 0x80,
                ..Default::default()
            }),
            "fu" => Some(Color {
                red: 0xFF,
                green: 0x00,
                blue: 0xFF,
                ..Default::default()
            }),
            "te" => Some(Color {
                red: 0x00,
                green: 0x80,
                blue: 0x80,
                ..Default::default()
            }),
            "aq" => Some(Color {
                red: 0x00,
                green: 0xFF,
                blue: 0xFF,
                ..Default::default()
            }),
            _ => None,
        }
    }

    fn try_parse_known_color(string: &str) -> Option<Color> {
        match string {
            "aliceblue" => Some(Color::new_enum(KnownColors::AliceBlue)),
            "antiquewhite" => Some(Color::new_enum(KnownColors::AntiqueWhite)),
            "aqua" => Some(Color::new_enum(KnownColors::Aqua)),
            "aquamarine" => Some(Color::new_enum(KnownColors::AquaMarine)),
            "azure" => Some(Color::new_enum(KnownColors::Azure)),
            "beige" => Some(Color::new_enum(KnownColors::Beige)),
            "bisque" => Some(Color::new_enum(KnownColors::Bisque)),
            "black" => Some(Color::new_enum(KnownColors::Black)),
            "blanchedalmond" => Some(Color::new_enum(KnownColors::BlanchedAlmond)),
            "blue" => Some(Color::new_enum(KnownColors::Blue)),
            "blueviolet" => Some(Color::new_enum(KnownColors::BlueViolet)),
            "brown" => Some(Color::new_enum(KnownColors::Brown)),
            "burlywood" => Some(Color::new_enum(KnownColors::BurlyWood)),
            "cadetblue" => Some(Color::new_enum(KnownColors::CadetBlue)),
            "chartreuse" => Some(Color::new_enum(KnownColors::Chartreuse)),
            "chocolate" => Some(Color::new_enum(KnownColors::Chocolate)),
            "coral" => Some(Color::new_enum(KnownColors::Coral)),
            "cornflowerblue" => Some(Color::new_enum(KnownColors::CornflowerBlue)),
            "cornsilk" => Some(Color::new_enum(KnownColors::Cornsilk)),
            "crimson" => Some(Color::new_enum(KnownColors::Crimson)),
            "cyan" => Some(Color::new_enum(KnownColors::Cyan)),
            "darkblue" => Some(Color::new_enum(KnownColors::DarkBlue)),
            "darkcyan" => Some(Color::new_enum(KnownColors::DarkCyan)),
            "darkgoldenrod" => Some(Color::new_enum(KnownColors::DarkGoldenrod)),
            "darkgray" => Some(Color::new_enum(KnownColors::DarkGray)),
            "darkgrey" => Some(Color::new_enum(KnownColors::DarkGray)),
            "darkgreen" => Some(Color::new_enum(KnownColors::DarkGreen)),
            "darkkhaki" => Some(Color::new_enum(KnownColors::DarkKhaki)),
            "darkmagenta" => Some(Color::new_enum(KnownColors::DarkMagenta)),
            "darkolivegreen" => Some(Color::new_enum(KnownColors::DarkOliveGreen)),
            "darkorange" => Some(Color::new_enum(KnownColors::DarkOrange)),
            "darkorchid" => Some(Color::new_enum(KnownColors::DarkOrchid)),
            "darkred" => Some(Color::new_enum(KnownColors::DarkRed)),
            "darksalmon" => Some(Color::new_enum(KnownColors::DarkSalmon)),
            "darkseagreen" => Some(Color::new_enum(KnownColors::DarkSeaGreen)),
            "darkslateblue" => Some(Color::new_enum(KnownColors::DarkSlateBlue)),
            "darkslategray" => Some(Color::new_enum(KnownColors::DarkSlateGray)),
            "darkslategrey" => Some(Color::new_enum(KnownColors::DarkSlateGray)),
            "darkturquoise" => Some(Color::new_enum(KnownColors::DarkTurquoise)),
            "darkviolet" => Some(Color::new_enum(KnownColors::DarkViolet)),
            "deeppink" => Some(Color::new_enum(KnownColors::DeepPink)),
            "deepskyblue" => Some(Color::new_enum(KnownColors::DeepSkyBlue)),
            "dimgray" => Some(Color::new_enum(KnownColors::DimGray)),
            "dimgrey" => Some(Color::new_enum(KnownColors::DimGray)),
            "dodgerblue" => Some(Color::new_enum(KnownColors::DodgerBlue)),
            "firebrick" => Some(Color::new_enum(KnownColors::Firebrick)),
            "floralwhite" => Some(Color::new_enum(KnownColors::FloralWhite)),
            "forestgreen" => Some(Color::new_enum(KnownColors::ForestGreen)),
            "fuchsia" => Some(Color::new_enum(KnownColors::Fuchsia)),
            "gainsboro" => Some(Color::new_enum(KnownColors::Gainsboro)),
            "ghostwhite" => Some(Color::new_enum(KnownColors::GhostWhite)),
            "gold" => Some(Color::new_enum(KnownColors::Gold)),
            "goldenrod" => Some(Color::new_enum(KnownColors::Goldenrod)),
            "gray" => Some(Color::new_enum(KnownColors::Gray)),
            "grey" => Some(Color::new_enum(KnownColors::Gray)),
            "green" => Some(Color::new_enum(KnownColors::Green)),
            "greenyellow" => Some(Color::new_enum(KnownColors::GreenYellow)),
            "honeydew" => Some(Color::new_enum(KnownColors::Honeydew)),
            "hotpink" => Some(Color::new_enum(KnownColors::HotPink)),
            "indianred" => Some(Color::new_enum(KnownColors::IndianRed)),
            "indigo" => Some(Color::new_enum(KnownColors::Indigo)),
            "ivory" => Some(Color::new_enum(KnownColors::Ivory)),
            "khaki" => Some(Color::new_enum(KnownColors::Khaki)),
            "lavender" => Some(Color::new_enum(KnownColors::Lavender)),
            "lavenderblush" => Some(Color::new_enum(KnownColors::LavenderBlush)),
            "lawngreen" => Some(Color::new_enum(KnownColors::LawnGreen)),
            "lemonchiffon" => Some(Color::new_enum(KnownColors::LemonChiffon)),
            "lightblue" => Some(Color::new_enum(KnownColors::LightBlue)),
            "lightcoral" => Some(Color::new_enum(KnownColors::LightCoral)),
            "lightcyan" => Some(Color::new_enum(KnownColors::LightCyan)),
            "lightgoldenrodyellow" => Some(Color::new_enum(KnownColors::LightGoldenrodYellow)),
            "lightgray" => Some(Color::new_enum(KnownColors::LightGray)),
            "lightgrey" => Some(Color::new_enum(KnownColors::LightGray)),
            "lightgreen" => Some(Color::new_enum(KnownColors::LightGreen)),
            "lightpink" => Some(Color::new_enum(KnownColors::LightPink)),
            "lightsalmon" => Some(Color::new_enum(KnownColors::LightSalmon)),
            "lightseagreen" => Some(Color::new_enum(KnownColors::LightSeaGreen)),
            "lightskyblue" => Some(Color::new_enum(KnownColors::LightSkyBlue)),
            "lightslategray" => Some(Color::new_enum(KnownColors::LightSlateGray)),
            "lightslategrey" => Some(Color::new_enum(KnownColors::LightSlateGray)),
            "lightsteelblue" => Some(Color::new_enum(KnownColors::LightSteelBlue)),
            "lightyellow" => Some(Color::new_enum(KnownColors::LightYellow)),
            "lime" => Some(Color::new_enum(KnownColors::Lime)),
            "limegreen" => Some(Color::new_enum(KnownColors::LimeGreen)),
            "linen" => Some(Color::new_enum(KnownColors::Linen)),
            "magenta" => Some(Color::new_enum(KnownColors::Magenta)),
            "maroon" => Some(Color::new_enum(KnownColors::Maroon)),
            "mediumaquamarine" => Some(Color::new_enum(KnownColors::MediumAquaMarine)),
            "mediumblue" => Some(Color::new_enum(KnownColors::MediumBlue)),
            "mediumorchid" => Some(Color::new_enum(KnownColors::MediumOrchid)),
            "mediumpurple" => Some(Color::new_enum(KnownColors::MediumPurple)),
            "mediumseagreen" => Some(Color::new_enum(KnownColors::MediumSeaGreen)),
            "mediumslateblue" => Some(Color::new_enum(KnownColors::MediumSlateBlue)),
            "mediumspringgreen" => Some(Color::new_enum(KnownColors::MediumSpringGreen)),
            "mediumturquoise" => Some(Color::new_enum(KnownColors::MediumTurquoise)),
            "mediumvioletred" => Some(Color::new_enum(KnownColors::MediumVioletRed)),
            "midnightblue" => Some(Color::new_enum(KnownColors::MidnightBlue)),
            "mintcream" => Some(Color::new_enum(KnownColors::MintCream)),
            "mistyrose" => Some(Color::new_enum(KnownColors::MistyRose)),
            "moccasin" => Some(Color::new_enum(KnownColors::Moccasin)),
            "navajowhite" => Some(Color::new_enum(KnownColors::NavajoWhite)),
            "navy" => Some(Color::new_enum(KnownColors::Navy)),
            "oldlace" => Some(Color::new_enum(KnownColors::OldLace)),
            "olive" => Some(Color::new_enum(KnownColors::Olive)),
            "olivedrab" => Some(Color::new_enum(KnownColors::OliveDrab)),
            "orange" => Some(Color::new_enum(KnownColors::Orange)),
            "orangered" => Some(Color::new_enum(KnownColors::OrangeRed)),
            "orchid" => Some(Color::new_enum(KnownColors::Orchid)),
            "palegoldenrod" => Some(Color::new_enum(KnownColors::PaleGoldenrod)),
            "palegreen" => Some(Color::new_enum(KnownColors::PaleGreen)),
            "paleturquoise" => Some(Color::new_enum(KnownColors::PaleTurquoise)),
            "palevioletred" => Some(Color::new_enum(KnownColors::PaleVioletRed)),
            "papayawhip" => Some(Color::new_enum(KnownColors::PapayaWhip)),
            "peachpuff" => Some(Color::new_enum(KnownColors::PeachPuff)),
            "peru" => Some(Color::new_enum(KnownColors::Peru)),
            "pink" => Some(Color::new_enum(KnownColors::Pink)),
            "plum" => Some(Color::new_enum(KnownColors::Plum)),
            "powderblue" => Some(Color::new_enum(KnownColors::PowderBlue)),
            "purple" => Some(Color::new_enum(KnownColors::Purple)),
            "red" => Some(Color::new_enum(KnownColors::Red)),
            "rosybrown" => Some(Color::new_enum(KnownColors::RosyBrown)),
            "royalblue" => Some(Color::new_enum(KnownColors::RoyalBlue)),
            "saddlebrown" => Some(Color::new_enum(KnownColors::SaddleBrown)),
            "salmon" => Some(Color::new_enum(KnownColors::Salmon)),
            "sandybrown" => Some(Color::new_enum(KnownColors::SandyBrown)),
            "seagreen" => Some(Color::new_enum(KnownColors::SeaGreen)),
            "seashell" => Some(Color::new_enum(KnownColors::SeaShell)),
            "sienna" => Some(Color::new_enum(KnownColors::Sienna)),
            "silver" => Some(Color::new_enum(KnownColors::Silver)),
            "skyblue" => Some(Color::new_enum(KnownColors::SkyBlue)),
            "slateblue" => Some(Color::new_enum(KnownColors::SlateBlue)),
            "slategray" => Some(Color::new_enum(KnownColors::SlateGray)),
            "slategrey" => Some(Color::new_enum(KnownColors::SlateGray)),
            "snow" => Some(Color::new_enum(KnownColors::Snow)),
            "springgreen" => Some(Color::new_enum(KnownColors::SpringGreen)),
            "steelblue" => Some(Color::new_enum(KnownColors::SteelBlue)),
            "tan" => Some(Color::new_enum(KnownColors::Tan)),
            "teal" => Some(Color::new_enum(KnownColors::Teal)),
            "thistle" => Some(Color::new_enum(KnownColors::Thistle)),
            "tomato" => Some(Color::new_enum(KnownColors::Tomato)),
            "transparent" => Some(Color::new_enum(KnownColors::Transparent)),
            "turquoise" => Some(Color::new_enum(KnownColors::Turquoise)),
            "violet" => Some(Color::new_enum(KnownColors::Violet)),
            "wheat" => Some(Color::new_enum(KnownColors::Wheat)),
            "white" => Some(Color::new_enum(KnownColors::White)),
            "whitesmoke" => Some(Color::new_enum(KnownColors::WhiteSmoke)),
            "yellow" => Some(Color::new_enum(KnownColors::Yellow)),
            "yellowgreen" => Some(Color::new_enum(KnownColors::YellowGreen)),
            _ => None,
        }
    }

    fn rgb_xyz(val: u8) -> f64 {
        let val = val as f64 / 255.0;
        if val <= 0.04045 {
            return val as f64 / 12.92;
        }

        ((val as f64 + 0.055) / 1.055).powf(2.4)
    }

    fn xyz_rgb(r: f64) -> f64 {
        if r <= 0.00304 {
            255.0 * (12.92 * r)
        } else {
            255.0 * (1.055 * r.powf(1.0 / 2.4) - 0.055)
        }
    }

    fn lab_xyz(t: f64) -> f64 {
        if t > Color::LAB_CONSTANT_T1 {
            t * t * t
        } else {
            Color::LAB_CONSTANT_T2 * (t - Color::LAB_CONSTANT_T0)
        }
    }

    fn xyz_lab(t: f64) -> f64 {
        if t > Color::LAB_CONSTANT_T3 {
            return t.powf(1.0 / 3.0);
        }

        t / Color::LAB_CONSTANT_T2 + Color::LAB_CONSTANT_T0
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new()
    }
}

impl From<Color> for i32 {
    /// Converts a Color-struct into a i32 number.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red = Color::new_string("#FF0000").unwrap();
    /// let red_number: i32 = red.into();
    ///
    /// assert_eq!(-65536, red_number);
    /// assert_eq!(0xffff0000_u32 as i32, red_number);
    ///
    /// // alternative:
    /// let green = Color::new_string("#00FF00").unwrap();
    /// let green_number = i32::from(green);
    ///
    /// assert_eq!(-16711936, green_number);
    /// assert_eq!(0xff00ff00_u32 as i32, green_number);
    /// ```
    fn from(color: Color) -> Self {
        color.to_number()
    }
}

impl From<i32> for Color {
    /// Converts a i32 number into a Color-struct.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red_number = -65536;
    /// let red_number_hex = 0xffff0000_u32 as i32;
    ///
    /// let red: Color = red_number.into();
    /// let red_from_hex: Color = red_number_hex.into();
    ///
    /// assert_eq!("#FF0000", red.to_hex_string());
    /// assert_eq!("#FF0000", red_from_hex.to_hex_string());
    ///
    /// // alternative:
    /// let green_number = -16711936;
    /// let green_number_hex = 0xff00ff00_u32 as i32;
    ///
    /// let green = Color::from(green_number);
    /// let green_from_hex = Color::from(green_number_hex);
    ///
    /// assert_eq!("#00FF00", green.to_hex_string());
    /// assert_eq!("#00FF00", green_from_hex.to_hex_string());
    /// ```
    fn from(number: i32) -> Self {
        Color {
            alpha: ((number & 0xff000000_u32 as i32) >> 24) as u8,
            red: ((number & 0xff0000) >> 16) as u8,
            green: ((number & 0xff00) >> 8) as u8,
            blue: (number & 0xff) as u8,
            ..Default::default()
        }
    }
}

impl FromStr for Color {
    type Err = &'static str;

    /// Parses a string into a Color-struct.
    ///
    /// # Example
    /// ```
    /// use color_processing::Color;
    ///
    /// let red: Color = "red".parse().unwrap();
    ///
    /// assert_eq!(255, red.red);
    /// assert_eq!(0, red.green);
    /// assert_eq!(0, red.blue);
    /// assert_eq!(255, red.alpha);
    ///
    /// // alternative:
    /// let green = "green".parse::<Color>().unwrap();
    ///
    /// assert_eq!(0, green.red);
    /// assert_eq!(128, green.green);
    /// assert_eq!(0, green.blue);
    /// assert_eq!(255, green.alpha);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Color::new_string(s) {
            Some(color) => Ok(color),
            None => Err("unable to parse string to Color-struct."),
        }
    }
}

pub enum KnownColors {
    AliceBlue,
    AntiqueWhite,
    Aqua,
    AquaMarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    Blue,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    CornflowerBlue,
    Cornsilk,
    Crimson,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGoldenrod,
    DarkGray,
    DarkGreen,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOrange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DodgerBlue,
    Firebrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    Goldenrod,
    Gray,
    Green,
    GreenYellow,
    Honeydew,
    HotPink,
    IndianRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenrodYellow,
    LightGray,
    LightGreen,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MediumAquaMarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenrod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    Red,
    RosyBrown,
    RoyalBlue,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    SeaShell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Transparent,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen,
}

#[cfg(test)]
mod tests {
    #[test]
    fn round() {
        let pi = 3.1425;
        let pi_round_1 = super::round_with_precision(pi, 1);
        let pi_round_2 = super::round_with_precision(pi, 2);
        let pi_round_3 = super::round_with_precision(pi, 3);
        assert_eq!(pi_round_1, 3.1);
        assert_eq!(pi_round_2, 3.14);
        assert_eq!(pi_round_3, 3.143);
    }
}
