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
//! assert_eq!("cmyk(0%, 0%, 0%, 70.2%)", grayscaled_red.to_cmyk_string());
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

#[macro_use] extern crate lazy_static;

extern crate regex;

use self::regex::Regex;
use std::f64::consts::PI;
use std::str::FromStr;

fn round_with_precision(number: f64, precision: u8) -> f64 {
    let multiplier = 10_f64.powi(precision as i32);
    (number * multiplier).round() / multiplier
}

pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
    pub alpha: u8,
}

impl Color {
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
            alpha: 255
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
			KnownColors::AliceBlue => Color {red: 0xF0, green: 0xF8, blue: 0xFF, alpha: 0xFF},
			KnownColors::AntiqueWhite  => Color {red: 0xFA, green: 0xEB, blue: 0xD7, alpha: 0xFF},
			KnownColors::Aqua => Color {red: 0x00, green: 0xFF, blue: 0xFF, alpha: 0xFF},
			KnownColors::AquaMarine => Color {red: 0x7F, green: 0xFF, blue: 0xD4, alpha: 0xFF},
			KnownColors::Azure => Color {red: 0xF0, green: 0xFF, blue: 0xFF, alpha: 0xFF},
			KnownColors::Beige => Color {red: 0xF5, green: 0xF5, blue: 0xDC, alpha: 0xFF},
			KnownColors::Bisque => Color {red: 0xFF, green: 0xE4, blue: 0xC4, alpha: 0xFF},
			KnownColors::Black => Color {red: 0x00, green: 0x00, blue: 0x00, alpha: 0xFF},
			KnownColors::BlanchedAlmond => Color {red: 0xFF, green: 0xEB, blue: 0xCD, alpha: 0xFF},
			KnownColors::Blue => Color {red: 0x00, green: 0x00, blue: 0xFF, alpha: 0xFF},
			KnownColors::BlueViolet => Color {red: 0x8A, green: 0x2B, blue: 0xE2, alpha: 0xFF},
			KnownColors::Brown => Color {red: 0xA5, green: 0x2A, blue: 0x2A, alpha: 0xFF},
			KnownColors::BurlyWood => Color {red: 0xDE, green: 0xB8, blue: 0x87, alpha: 0xFF},
			KnownColors::CadetBlue => Color {red: 0x5F, green: 0x9E, blue: 0xA0, alpha: 0xFF},
			KnownColors::Chartreuse => Color {red: 0x7F, green: 0xFF, blue: 0x00, alpha: 0xFF},
			KnownColors::Chocolate => Color {red: 0xD2, green: 0x69, blue: 0x1E, alpha: 0xFF},
			KnownColors::Coral => Color {red: 0xFF, green: 0x7F, blue: 0x50, alpha: 0xFF},
			KnownColors::CornflowerBlue => Color {red: 0x64, green: 0x95, blue: 0xED, alpha: 0xFF},
			KnownColors::Cornsilk => Color {red: 0xFF, green: 0xF8, blue: 0xDC, alpha: 0xFF},
			KnownColors::Crimson => Color {red: 0xDC, green: 0x14, blue: 0x3C, alpha: 0xFF},
			KnownColors::Cyan => Color {red: 0x00, green: 0xFF, blue: 0xFF, alpha: 0xFF},
			KnownColors::DarkBlue => Color {red: 0x00, green: 0x00, blue: 0x8B, alpha: 0xFF},
			KnownColors::DarkCyan => Color {red: 0x00, green: 0x8B, blue: 0x8B, alpha: 0xFF},
			KnownColors::DarkGoldenrod => Color {red: 0xB8, green: 0x86, blue: 0x0B, alpha: 0xFF},
			KnownColors::DarkGray => Color {red: 0xA9, green: 0xA9, blue: 0xA9, alpha: 0xFF},
			KnownColors::DarkGreen => Color {red: 0x00, green: 0x64, blue: 0x00, alpha: 0xFF},
			KnownColors::DarkKhaki => Color {red: 0xBD, green: 0xB7, blue: 0x6B, alpha: 0xFF},
			KnownColors::DarkMagenta => Color {red: 0x8B, green: 0x00, blue: 0x8B, alpha: 0xFF},
			KnownColors::DarkOliveGreen => Color {red: 0x55, green: 0x6B, blue: 0x2F, alpha: 0xFF},
			KnownColors::DarkOrange => Color {red: 0xFF, green: 0x8C, blue: 0x00, alpha: 0xFF},
			KnownColors::DarkOrchid => Color {red: 0x99, green: 0x32, blue: 0xCC, alpha: 0xFF},
			KnownColors::DarkRed => Color {red: 0x8B, green: 0x00, blue: 0x00, alpha: 0xFF},
			KnownColors::DarkSalmon => Color {red: 0xE9, green: 0x96, blue: 0x7A, alpha: 0xFF},
			KnownColors::DarkSeaGreen => Color {red: 0x8F, green: 0xBC, blue: 0x8B, alpha: 0xFF},
			KnownColors::DarkSlateBlue => Color {red: 0x48, green: 0x3D, blue: 0x8B, alpha: 0xFF},
			KnownColors::DarkSlateGray => Color {red: 0x2F, green: 0x4F, blue: 0x4F, alpha: 0xFF},
			KnownColors::DarkTurquoise => Color {red: 0x00, green: 0xCE, blue: 0xD1, alpha: 0xFF},
			KnownColors::DarkViolet => Color {red: 0x94, green: 0x00, blue: 0xD3, alpha: 0xFF},
			KnownColors::DeepPink => Color {red: 0xFF, green: 0x14, blue: 0x93, alpha: 0xFF},
			KnownColors::DeepSkyBlue => Color {red: 0x00, green: 0xBF, blue: 0xFF, alpha: 0xFF},
			KnownColors::DimGray => Color {red: 0x69, green: 0x69, blue: 0x69, alpha: 0xFF},
			KnownColors::DodgerBlue => Color {red: 0x1E, green: 0x90, blue: 0xFF, alpha: 0xFF},
			KnownColors::Firebrick => Color {red: 0xB2, green: 0x22, blue: 0x22, alpha: 0xFF},
			KnownColors::FloralWhite => Color {red: 0xFF, green: 0xFA, blue: 0xF0, alpha: 0xFF},
			KnownColors::ForestGreen => Color {red: 0x22, green: 0x8B, blue: 0x22, alpha: 0xFF},
			KnownColors::Fuchsia => Color {red: 0xFF, green: 0x00, blue: 0xFF, alpha: 0xFF},
			KnownColors::Gainsboro => Color {red: 0xDC, green: 0xDC, blue: 0xDC, alpha: 0xFF},
			KnownColors::GhostWhite => Color {red: 0xF8, green: 0xF8, blue: 0xFF, alpha: 0xFF},
			KnownColors::Gold => Color {red: 0xFF, green: 0xD7, blue: 0x00, alpha: 0xFF},
			KnownColors::Goldenrod => Color {red: 0xDA, green: 0xA5, blue: 0x20, alpha: 0xFF},
			KnownColors::Gray => Color {red: 0x80, green: 0x80, blue: 0x80, alpha: 0xFF},
			KnownColors::Green => Color {red: 0x00, green: 0x80, blue: 0x00, alpha: 0xFF},
			KnownColors::GreenYellow => Color {red: 0xAD, green: 0xFF, blue: 0x2F, alpha: 0xFF},
			KnownColors::Honeydew => Color {red: 0xF0, green: 0xFF, blue: 0xF0, alpha: 0xFF},
			KnownColors::HotPink => Color {red: 0xFF, green: 0x69, blue: 0xB4, alpha: 0xFF},
			KnownColors::IndianRed => Color {red: 0xCD, green: 0x5C, blue: 0x5C, alpha: 0xFF},
			KnownColors::Indigo => Color {red: 0x4B, green: 0x00, blue: 0x82, alpha: 0xFF},
			KnownColors::Ivory => Color {red: 0xFF, green: 0xFF, blue: 0xF0, alpha: 0xFF},
			KnownColors::Khaki => Color {red: 0xF0, green: 0xE6, blue: 0x8C, alpha: 0xFF},
			KnownColors::Lavender => Color {red: 0xE6, green: 0xE6, blue: 0xFA, alpha: 0xFF},
			KnownColors::LavenderBlush => Color {red: 0xFF, green: 0xF0, blue: 0xF5, alpha: 0xFF},
			KnownColors::LawnGreen => Color {red: 0x7C, green: 0xFC, blue: 0x00, alpha: 0xFF},
			KnownColors::LemonChiffon => Color {red: 0xFF, green: 0xFA, blue: 0xCD, alpha: 0xFF},
			KnownColors::LightBlue => Color {red: 0xAD, green: 0xD8, blue: 0xE6, alpha: 0xFF},
			KnownColors::LightCoral => Color {red: 0xF0, green: 0x80, blue: 0x80, alpha: 0xFF},
			KnownColors::LightCyan => Color {red: 0xE0, green: 0xFF, blue: 0xFF, alpha: 0xFF},
			KnownColors::LightGoldenrodYellow => Color {red: 0xFA, green: 0xFA, blue: 0xD2, alpha: 0xFF},
			KnownColors::LightGray => Color {red: 0xD3, green: 0xD3, blue: 0xD3, alpha: 0xFF},
			KnownColors::LightGreen => Color {red: 0x90, green: 0xEE, blue: 0x90, alpha: 0xFF},
			KnownColors::LightPink => Color {red: 0xFF, green: 0xB6, blue: 0xC1, alpha: 0xFF},
			KnownColors::LightSalmon => Color {red: 0xFF, green: 0xA0, blue: 0x7A, alpha: 0xFF},
			KnownColors::LightSeaGreen => Color {red: 0x20, green: 0xB2, blue: 0xAA, alpha: 0xFF},
			KnownColors::LightSkyBlue => Color {red: 0x87, green: 0xCE, blue: 0xFA, alpha: 0xFF},
			KnownColors::LightSlateGray => Color {red: 0x77, green: 0x88, blue: 0x99, alpha: 0xFF},
			KnownColors::LightSteelBlue => Color {red: 0xB0, green: 0xC4, blue: 0xDE, alpha: 0xFF},
			KnownColors::LightYellow => Color {red: 0xFF, green: 0xFF, blue: 0xE0, alpha: 0xFF},
			KnownColors::Lime => Color {red: 0x00, green: 0xFF, blue: 0x00, alpha: 0xFF},
			KnownColors::LimeGreen => Color {red: 0x32, green: 0xCD, blue: 0x32, alpha: 0xFF},
			KnownColors::Linen => Color {red: 0xFA, green: 0xF0, blue: 0xE6, alpha: 0xFF},
			KnownColors::Magenta => Color {red: 0xFF, green: 0x00, blue: 0xFF, alpha: 0xFF},
			KnownColors::Maroon => Color {red: 0x80, green: 0x00, blue: 0x00, alpha: 0xFF},
			KnownColors::MediumAquaMarine => Color {red: 0x66, green: 0xCD, blue: 0xAA, alpha: 0xFF},
			KnownColors::MediumBlue => Color {red: 0x00, green: 0x00, blue: 0xCD, alpha: 0xFF},
			KnownColors::MediumOrchid => Color {red: 0xBA, green: 0x55, blue: 0xD3, alpha: 0xFF},
			KnownColors::MediumPurple => Color {red: 0x93, green: 0x70, blue: 0xDB, alpha: 0xFF},
			KnownColors::MediumSeaGreen => Color {red: 0x3C, green: 0xB3, blue: 0x71, alpha: 0xFF},
			KnownColors::MediumSlateBlue => Color {red: 0x7B, green: 0x68, blue: 0xEE, alpha: 0xFF},
			KnownColors::MediumSpringGreen => Color {red: 0x00, green: 0xFA, blue: 0x9A, alpha: 0xFF},
			KnownColors::MediumTurquoise => Color {red: 0x48, green: 0xD1, blue: 0xCC, alpha: 0xFF},
			KnownColors::MediumVioletRed => Color {red: 0xC7, green: 0x15, blue: 0x85, alpha: 0xFF},
			KnownColors::MidnightBlue => Color {red: 0x19, green: 0x19, blue: 0x70, alpha: 0xFF},
			KnownColors::MintCream => Color {red: 0xF5, green: 0xFF, blue: 0xFA, alpha: 0xFF},
			KnownColors::MistyRose => Color {red: 0xFF, green: 0xE4, blue: 0xE1, alpha: 0xFF},
			KnownColors::Moccasin => Color {red: 0xFF, green: 0xE4, blue: 0xB5, alpha: 0xFF},
			KnownColors::NavajoWhite => Color {red: 0xFF, green: 0xDE, blue: 0xAD, alpha: 0xFF},
			KnownColors::Navy => Color {red: 0x00, green: 0x00, blue: 0x80, alpha: 0xFF},
			KnownColors::OldLace => Color {red: 0xFD, green: 0xF5, blue: 0xE6, alpha: 0xFF},
			KnownColors::Olive => Color {red: 0x80, green: 0x80, blue: 0x00, alpha: 0xFF},
			KnownColors::OliveDrab => Color {red: 0x6B, green: 0x8E, blue: 0x23, alpha: 0xFF},
			KnownColors::Orange => Color {red: 0xFF, green: 0xA5, blue: 0x00, alpha: 0xFF},
			KnownColors::OrangeRed => Color {red: 0xFF, green: 0x45, blue: 0x00, alpha: 0xFF},
			KnownColors::Orchid => Color {red: 0xDA, green: 0x70, blue: 0xD6, alpha: 0xFF},
			KnownColors::PaleGoldenrod => Color {red: 0xEE, green: 0xE8, blue: 0xAA, alpha: 0xFF},
			KnownColors::PaleGreen => Color {red: 0x98, green: 0xFB, blue: 0x98, alpha: 0xFF},
			KnownColors::PaleTurquoise => Color {red: 0xAF, green: 0xEE, blue: 0xEE, alpha: 0xFF},
			KnownColors::PaleVioletRed => Color {red: 0xDB, green: 0x70, blue: 0x93, alpha: 0xFF},
			KnownColors::PapayaWhip => Color {red: 0xFF, green: 0xEF, blue: 0xD5, alpha: 0xFF},
			KnownColors::PeachPuff => Color {red: 0xFF, green: 0xDA, blue: 0xB9, alpha: 0xFF},
			KnownColors::Peru => Color {red: 0xCD, green: 0x85, blue: 0x3F, alpha: 0xFF},
			KnownColors::Pink => Color {red: 0xFF, green: 0xC0, blue: 0xCB, alpha: 0xFF},
			KnownColors::Plum => Color {red: 0xDD, green: 0xA0, blue: 0xDD, alpha: 0xFF},
			KnownColors::PowderBlue => Color {red: 0xB0, green: 0xE0, blue: 0xE6, alpha: 0xFF},
			KnownColors::Purple => Color {red: 0x80, green: 0x00, blue: 0x80, alpha: 0xFF},
			KnownColors::Red => Color {red: 0xFF, green: 0x00, blue: 0x00, alpha: 0xFF},
			KnownColors::RosyBrown => Color {red: 0xBC, green: 0x8F, blue: 0x8F, alpha: 0xFF},
			KnownColors::RoyalBlue => Color {red: 0x41, green: 0x69, blue: 0xE1, alpha: 0xFF},
			KnownColors::SaddleBrown => Color {red: 0x8B, green: 0x45, blue: 0x13, alpha: 0xFF},
			KnownColors::Salmon => Color {red: 0xFA, green: 0x80, blue: 0x72, alpha: 0xFF},
			KnownColors::SandyBrown => Color {red: 0xF4, green: 0xA4, blue: 0x60, alpha: 0xFF},
			KnownColors::SeaGreen => Color {red: 0x2E, green: 0x8B, blue: 0x57, alpha: 0xFF},
			KnownColors::SeaShell => Color {red: 0xFF, green: 0xF5, blue: 0xEE, alpha: 0xFF},
			KnownColors::Sienna => Color {red: 0xA0, green: 0x52, blue: 0x2D, alpha: 0xFF},
			KnownColors::Silver => Color {red: 0xC0, green: 0xC0, blue: 0xC0, alpha: 0xFF},
			KnownColors::SkyBlue => Color {red: 0x87, green: 0xCE, blue: 0xEB, alpha: 0xFF},
			KnownColors::SlateBlue => Color {red: 0x6A, green: 0x5A, blue: 0xCD, alpha: 0xFF},
			KnownColors::SlateGray => Color {red: 0x70, green: 0x80, blue: 0x90, alpha: 0xFF},
			KnownColors::Snow => Color {red: 0xFF, green: 0xFA, blue: 0xFA, alpha: 0xFF},
			KnownColors::SpringGreen => Color {red: 0x00, green: 0xFF, blue: 0x7F, alpha: 0xFF},
			KnownColors::SteelBlue => Color {red: 0x46, green: 0x82, blue: 0xB4, alpha: 0xFF},
			KnownColors::Tan => Color {red: 0xD2, green: 0xB4, blue: 0x8C, alpha: 0xFF},
			KnownColors::Teal => Color {red: 0x00, green: 0x80, blue: 0x80, alpha: 0xFF},
			KnownColors::Thistle => Color {red: 0xD8, green: 0xBF, blue: 0xD8, alpha: 0xFF},
			KnownColors::Tomato => Color {red: 0xFF, green: 0x63, blue: 0x47, alpha: 0xFF},
			KnownColors::Transparent => Color {red: 0x00, green: 0x00, blue: 0x00, alpha: 0x00},
			KnownColors::Turquoise => Color {red: 0x40, green: 0xE0, blue: 0xD0, alpha: 0xFF},
			KnownColors::Violet => Color {red: 0xEE, green: 0x82, blue: 0xEE, alpha: 0xFF},
			KnownColors::Wheat => Color {red: 0xF5, green: 0xDE, blue: 0xB3, alpha: 0xFF},
			KnownColors::White => Color {red: 0xFF, green: 0xFF, blue: 0xFF, alpha: 0xFF},
			KnownColors::WhiteSmoke => Color {red: 0xF5, green: 0xF5, blue: 0xF5, alpha: 0xFF},
			KnownColors::Yellow => Color {red: 0xFF, green: 0xFF, blue: 0x00, alpha: 0xFF},
			KnownColors::YellowGreen => Color {red: 0x9A, green: 0xCD, blue: 0x32, alpha: 0xFF}
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
        Color { red: gray, green: gray, blue: gray, alpha: 255 }
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
        let rgb = Color::get_rgb_from_hsl(hue, saturation, lightness);

        Color::new_rgb(rgb.0, rgb.1, rgb.2)
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
            alpha: 255
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
            alpha
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
	pub fn new_string(string: &str) -> Option<Color> {
        let trimmed_string = string.trim();
        let normalized_string = trimmed_string.to_lowercase();
        let normalized_str = normalized_string.as_str();

		Color::try_parse_known_color(normalized_str)
            .or_else(|| Color::try_parse_abbr_color(normalized_str))
            .or_else(|| Color::try_parse_hex(normalized_str))
            .or_else(|| Color::try_parse_css_function(normalized_str))
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
            ((1.0 - r - black) / white).round()
        } else {
            0.0
        };
        let magenta = if white != 0.0 {
            ((1.0 - g - black) / white).round()
        } else {
            0.0
        };
        let yellow = if white != 0.0 {
            ((1.0 - b - black) / white).round()
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
            h+= 360.0;
        }
        while h > 360.0 {
            h-= 360.0;
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

		(self.red as f64 / 255.0, self.green as f64 / 255.0, self.blue as f64 / 255.0, alpha)
	}

	pub fn get_lab(&self) -> (f64, f64, f64) {
		let mut red = self.red as f64 / 255.0;
		let mut green = self.green as f64 / 255.0;
		let mut blue = self.blue as f64 / 255.0;

		red = if red > 0.04045 {
			((red + 0.055) / 1.055).powf(2.4)
		} else {
			(red / 12.92)
		};
		green = if green > 0.04045 {
			((green + 0.055) / 1.055).powf(2.4)
		} else {
			(green / 12.92)
		};
		blue = if blue > 0.04045 {
			((blue + 0.055) / 1.055).powf(2.4)
		} else {
			(blue / 12.92)
		};

		let mut x = (red * 0.4124) + (green * 0.3576) + (blue * 0.1805);
		let mut y = (red * 0.2126) + (green * 0.7152) + (blue * 0.0722);
		let mut z = (red * 0.0193) + (green * 0.1192) + (blue * 0.9505);

		x*= 100.0;
		y*= 100.0;
		z*= 100.0;

		x/= 95.047;
		y/= 100.0;
		z/= 108.883;

		x = if x > 0.008856 {
			x.powf(1.0 / 3.0)
		} else {
			(7.787 * x) + (16.0 / 116.0)
		};
		y = if y > 0.008856 {
			y.powf(1.0 / 3.0)
		} else {
			(7.787 * y) + (16.0 / 116.0)
		};
		z = if z > 0.008856 {
			z.powf(1.0 / 3.0)
		} else {
			(7.787 * z) + (16.0 / 116.0)
		};

		((116.0 * y) - 16.0, 500.0 * (x - y), 200.0 * (y - z))
	}

	pub fn get_lch(&self) -> (f64, f64, f64) {
		let lab = self.get_lab();

		let c = (lab.1 * lab.1 + lab.2 * lab.2).sqrt();
		let h = (lab.2.atan2(lab.1) * 180.0 / PI + 360.0).round() % 360.0;

		(lab.0, c, h)
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

	pub fn colorize(&self, color: Color) -> Color {
		Color {
			alpha: (self.alpha as u16 * color.alpha as u16 / 255) as u8,
			red: (self.red as u16 * color.red as u16 / 255) as u8,
			green: (self.green as u16 * color.green as u16 / 255) as u8,
			blue: (self.blue as u16 * color.blue as u16 / 255) as u8
		}
	}

	pub fn colorize_string(&self, color: &str) -> Result<Color, &str> {
		match Color::new_string(color) {
			Some(color) => Ok(self.colorize(color)),
			None => Err("unable to parse color to colorize.")
		}
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
		let gray_value = (self.red as f64 * 0.299 + self.green as f64 * 0.587 + self.blue as f64 * 0.114).round() as u8;
		Color {
            red: gray_value,
            green: gray_value,
            blue: gray_value,
            alpha: self.alpha
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
		let gray_value = (self.red as f64 * 0.2126 + self.green as f64 * 0.7152 + self.blue as f64 * 0.0722).round() as u8;
		Color {
            red: gray_value,
            green: gray_value,
            blue: gray_value,
            alpha: self.alpha
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
		let gray_value = (self.red as f64 * 0.2627 + self.green as f64 * 0.678 + self.blue as f64 * 0.0593).round() as u8;
		Color {
            red: gray_value,
            green: gray_value,
            blue: gray_value,
            alpha: self.alpha
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
            Color { red: 0, green: 0, blue: 0, alpha: grayscaled.alpha }
        } else {
            Color { red: 255, green: 255, blue: 255, alpha: grayscaled.alpha }
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
		Color { red: 255 - self.red, green: 255 - self.green, blue: 255 - self.blue, alpha: self.alpha }
	}

	pub fn invert_luminescence(&self) -> Color {
		let hsla = self.get_hsla();
		Color::new_hsl(hsla.0, hsla.1, 1.0 - hsla.2)
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
		let key_rounded = round_with_precision(cmyk.3 * 100.0, 2);

		let mut cmyk_string = String::from("cmyk(");
		cmyk_string.push_str(format!("{}%, {}%, {}%, {}%", cmyk.0 * 100.0, cmyk.1 * 100.0, cmyk.2 * 100.0, key_rounded).as_str());
		cmyk_string.push_str(")");
		cmyk_string
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
			// round with a precision of 2 decimals.
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
			// round with a precision of 2 decimals.
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
			// round with a precision of 2 decimals.
			rgb.push_str(format!(", {}", ((self.alpha as f64) / 255.0 * 100.0).round() / 100.0).as_str());
		}
		rgb.push_str(")");

		rgb
	}

	pub fn interpolate(&self, color: Color, interpolation: f64) -> Color {
        let i = if interpolation < 0.0 {
            0.0
        } else if interpolation > 1.0 {
            1.0
        } else {
            interpolation
        };

		Color {
			red: (self.red as f64 + (color.red - self.red) as f64 * i).round() as u8,
			green: (self.green as f64 + (color.green - self.green) as f64 * i).round() as u8,
			blue: (self.blue as f64 + (color.blue - self.blue) as f64 * i).round() as u8,
            alpha: (self.alpha as f64 + (color.alpha - self.alpha) as f64 * i).round() as u8
		}
	}

    pub fn interpolate_hsv(&self, color: Color, mut interpolation: f64) -> Color {
        if interpolation < 0.0 {
            interpolation = 0.0;
        } else if interpolation > 1.0 {
            interpolation = 1.0;
        }

		let hsva = self.get_hsva();
		let first_h = hsva.0 / 255.0;
		let first_s = hsva.1;
		let first_v = hsva.2;

		let second_hsva = color.get_hsva();
		let second_h = second_hsva.0 / 255.0;
		let second_s = second_hsva.1;
		let second_v = second_hsva.2;

		let new_h = first_h + (second_h - first_h) * interpolation;
		let new_s = first_s + (second_s - first_s) * interpolation;
		let new_v = first_v + (second_v - first_v) * interpolation;

		Color::new_hsv(new_h * 255.0, new_s, new_v)
	}

	fn try_parse_hex(string: &str) -> Option<Color> {
		lazy_static! {
			static ref re_hex: Regex = Regex::new(r"^#?([0-9a-f]{3,8})$").unwrap();
		}
		let caps = re_hex.captures(string);
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
			},
			None => None
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
			static ref re_css_function: Regex = Regex::new(r"^(cmyk|gray|grey|hsla?|hsva?|hwba?|rgba?)\s*\(\s*(-?\d+(\.\d+)?)\s*(%|°)?\s*(,\s*(-?\d+(\.\d+)?)\s*(%)?\s*(,\s*(-?\d+(\.\d+)?)\s*(%)?\s*(,\s*(-?\d+(\.\d+)?)\s*(%)?\s*)?)?)?\)$").unwrap();
		}
		let caps = re_css_function.captures(string);
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
            },
            "hsl" => "hsl",
            "hsla" => {
                force_alpha = true;
                "hsl"
            },
            "hsv" => "hsv",
            "hsva" => {
                force_alpha = true;
                "hsv"
            },
            "hwb" => "hwb",
            "hwba" => {
                force_alpha = true;
                "hwb"
            },
            _ => ""
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

                let rgb = Color::get_rgb_from_cmyk(value_1 / 100.0, value_2 / 100.0, value_3 / 100.0, value_4 / 100.0);
                Some(Color::new_rgb(rgb.0, rgb.1, rgb.2))
            },
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

                    ((value_1 * 255.0).round() as u8, (value_2 * 255.0).round() as u8, (value_3 * 255.0).round() as u8)
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

                    (value_1.round() as u8, value_2.round() as u8, value_3.round() as u8)
                };

                Some(Color::new_rgba(rgb.0, rgb.1, rgb.2, alpha_opt.unwrap()))
            },
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
            },
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
            },
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
            },
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
            },
            _ => None
        }
	}

	fn try_parse_abbr_color(string: &str) -> Option<Color> {
		match string {
			"bk" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x00}),
			"wh" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0xFF}),
			"gr" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x80, blue: 0x80}),
			"si" => Some(Color {alpha: 0xFF, red: 0xC0, green: 0xC0, blue: 0xC0}),
			"mr" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x00, blue: 0x00}),
			"rd" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0x00}),
			"gn" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x80, blue: 0x00}),
			"li" => Some(Color {alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0x00}),
			"ol" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x80, blue: 0x00}),
			"ye" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0x00}),
			"na" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x80}),
			"bl" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0xFF}),
			"pu" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x00, blue: 0x80}),
			"fu" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0xFF}),
			"te" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x80, blue: 0x80}),
			"aq" => Some(Color {alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0xFF}),
			_ => None
		}
	}

	fn try_parse_known_color(string: &str) -> Option<Color> {
		match string {
			"aliceblue" => Some(Color::new_enum(KnownColors::AliceBlue)),
			"antiquewhite"  => Some(Color::new_enum(KnownColors::AntiqueWhite)),
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
			_ => None
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
            None => Err("unable to parse string to Color-struct.")
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
	YellowGreen
}

#[cfg(test)]
mod tests {
    use {Color, KnownColors};

    #[test]
    fn color_new() {
        let default_color = Color::new();
        assert_eq!(default_color.alpha, 255);
        assert_eq!(default_color.red, 0);
        assert_eq!(default_color.green, 0);
        assert_eq!(default_color.blue, 0);
    }

    #[test]
    fn color_new_rgb() {
        let red = Color::new_rgb(255, 0, 0);
        assert_eq!(red.red, 255);
        assert_eq!(red.green, 0);
        assert_eq!(red.blue, 0);
        assert_eq!(red.alpha, 255);

        let green = Color::new_rgb(0, 255, 0);
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
        assert_eq!(green.alpha, 255);

        let blue = Color::new_rgb(0, 0, 255);
        assert_eq!(blue.red, 0);
        assert_eq!(blue.green, 0);
        assert_eq!(blue.blue, 255);
        assert_eq!(blue.alpha, 255);

        let black = Color::new_rgb(0, 0, 0);
        assert_eq!(black.red, 0);
        assert_eq!(black.green, 0);
        assert_eq!(black.blue, 0);
        assert_eq!(black.alpha, 255);

        let white = Color::new_rgb(255, 255, 255);
        assert_eq!(white.red, 255);
        assert_eq!(white.green, 255);
        assert_eq!(white.blue, 255);
        assert_eq!(white.alpha, 255);
    }

    #[test]
    fn color_new_rgba() {
        let red_transparent = Color::new_rgba(255, 0, 0, 128);
        assert_eq!(red_transparent.red, 255);
        assert_eq!(red_transparent.green, 0);
        assert_eq!(red_transparent.blue, 0);
        assert_eq!(red_transparent.alpha, 128);
    }

	#[test]
	fn color_new_cmyk() {
		let red = Color::new_cmyk(0.0, 1.0, 1.0, 0.0);
        assert_eq!(red.red, 255);
        assert_eq!(red.green, 0);
        assert_eq!(red.blue, 0);
        assert_eq!(red.alpha, 255);

        let green = Color::new_cmyk(1.0, 0.0, 1.0, 0.0);
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
        assert_eq!(green.alpha, 255);

        let blue = Color::new_cmyk(1.0, 1.0, 0.0, 0.0);
        assert_eq!(blue.red, 0);
        assert_eq!(blue.green, 0);
        assert_eq!(blue.blue, 255);
        assert_eq!(blue.alpha, 255);

        let black = Color::new_cmyk(0.0, 0.0, 0.0, 1.0);
        assert_eq!(black.red, 0);
        assert_eq!(black.green, 0);
        assert_eq!(black.blue, 0);
        assert_eq!(black.alpha, 255);

        let white = Color::new_cmyk(0.0, 0.0, 0.0, 0.0);
        assert_eq!(white.red, 255);
        assert_eq!(white.green, 255);
        assert_eq!(white.blue, 255);
        assert_eq!(white.alpha, 255);

		let white_clipped = Color::new_cmyk(-10.0, -1.0, -0.5, -1000.0);
        assert_eq!(white_clipped.red, 255);
        assert_eq!(white_clipped.green, 255);
        assert_eq!(white_clipped.blue, 255);
        assert_eq!(white_clipped.alpha, 255);

		let black_clipped = Color::new_cmyk(0.0, 0.0, 0.0, 1.5);
        assert_eq!(black_clipped.red, 0);
        assert_eq!(black_clipped.green, 0);
        assert_eq!(black_clipped.blue, 0);
        assert_eq!(black_clipped.alpha, 255);

		let alternative_black_clipped = Color::new_cmyk(10.0, 100.0, 1.5, 0.0);
        assert_eq!(alternative_black_clipped.red, 0);
        assert_eq!(alternative_black_clipped.green, 0);
        assert_eq!(alternative_black_clipped.blue, 0);
        assert_eq!(alternative_black_clipped.alpha, 255);
	}

    #[test]
    fn color_enum() {
        let red = Color::new_enum(KnownColors::Red);
        assert_eq!(red.red, 255);
        assert_eq!(red.green, 0);
        assert_eq!(red.blue, 0);
        assert_eq!(red.alpha, 255);

        let green = Color::new_enum(KnownColors::Green);
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 128);
        assert_eq!(green.blue, 0);
        assert_eq!(green.alpha, 255);

        let blue = Color::new_enum(KnownColors::Blue);
        assert_eq!(blue.red, 0);
        assert_eq!(blue.green, 0);
        assert_eq!(blue.blue, 255);
        assert_eq!(blue.alpha, 255);

        let black = Color::new_enum(KnownColors::Black);
        assert_eq!(black.red, 0);
        assert_eq!(black.green, 0);
        assert_eq!(black.blue, 0);
        assert_eq!(black.alpha, 255);

        let white = Color::new_enum(KnownColors::White);
        assert_eq!(white.red, 255);
        assert_eq!(white.green, 255);
        assert_eq!(white.blue, 255);
        assert_eq!(white.alpha, 255);
    }

    #[test]
    fn color_new_hsl() {
        let red = Color::new_hsl(0.0, 1.0, 0.5);
        assert_eq!(red.red, 255);
        assert_eq!(red.green, 0);
        assert_eq!(red.blue, 0);
        assert_eq!(red.alpha, 255);

        let green = Color::new_hsl(120.0, 1.0, 0.5);
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
        assert_eq!(green.alpha, 255);

        let blue = Color::new_hsl(240.0, 1.0, 0.5);
        assert_eq!(blue.red, 0);
        assert_eq!(blue.green, 0);
        assert_eq!(blue.blue, 255);
        assert_eq!(blue.alpha, 255);

        let black = Color::new_hsl(0.0, 0.0, 0.0);
        assert_eq!(black.red, 0);
        assert_eq!(black.green, 0);
        assert_eq!(black.blue, 0);
        assert_eq!(black.alpha, 255);

        let white = Color::new_hsl(0.0, 0.0, 1.0);
        assert_eq!(white.red, 255);
        assert_eq!(white.green, 255);
        assert_eq!(white.blue, 255);
        assert_eq!(white.alpha, 255);

        let yellow = Color::new_hsl(60.0, 1.0, 0.5);
        assert_eq!(yellow.red, 255);
        assert_eq!(yellow.green, 255);
        assert_eq!(yellow.blue, 0);
        assert_eq!(yellow.alpha, 255);

        let cyan = Color::new_hsl(180.0, 1.0, 0.5);
        assert_eq!(cyan.red, 0);
        assert_eq!(cyan.green, 255);
        assert_eq!(cyan.blue, 255);
        assert_eq!(cyan.alpha, 255);

        let magenta = Color::new_hsl(300.0, 1.0, 0.5);
        assert_eq!(magenta.red, 255);
        assert_eq!(magenta.green, 0);
        assert_eq!(magenta.blue, 255);
        assert_eq!(magenta.alpha, 255);

		let saturation_clipped1 = Color::new_hsl(0.0, 2.0, 0.5);
        assert_eq!(saturation_clipped1.red, 255);
        assert_eq!(saturation_clipped1.green, 0);
        assert_eq!(saturation_clipped1.blue, 0);
        assert_eq!(saturation_clipped1.alpha, 255);

		let saturation_clipped = Color::new_hsl(0.0, -0.5, 0.0);
        assert_eq!(saturation_clipped.red, 0);
        assert_eq!(saturation_clipped.green, 0);
        assert_eq!(saturation_clipped.blue, 0);
        assert_eq!(saturation_clipped.alpha, 255);

		let green_normalized1 = Color::new_hsl(480.0, 1.0, 0.5);
        assert_eq!(green_normalized1.red, 0);
        assert_eq!(green_normalized1.green, 255);
        assert_eq!(green_normalized1.blue, 0);
        assert_eq!(green_normalized1.alpha, 255);

		let green_normalized2 = Color::new_hsl(-240.0, 1.0, 0.5);
        assert_eq!(green_normalized2.red, 0);
        assert_eq!(green_normalized2.green, 255);
        assert_eq!(green_normalized2.blue, 0);
        assert_eq!(green_normalized2.alpha, 255);

		let black_clipped = Color::new_hsl(0.0, 0.0, -0.3);
        assert_eq!(black_clipped.red, 0);
        assert_eq!(black_clipped.green, 0);
        assert_eq!(black_clipped.blue, 0);
        assert_eq!(black_clipped.alpha, 255);

        let white_clipped = Color::new_hsl(0.0, 0.0, 1.5);
        assert_eq!(white_clipped.red, 255);
        assert_eq!(white_clipped.green, 255);
        assert_eq!(white_clipped.blue, 255);
        assert_eq!(white_clipped.alpha, 255);
    }

    #[test]
    fn color_new_hsla() {
        let red_transparent = Color::new_hsla(0.0, 1.0, 0.5, 0.5);
        assert_eq!(red_transparent.red, 255);
        assert_eq!(red_transparent.green, 0);
        assert_eq!(red_transparent.blue, 0);
        assert_eq!(red_transparent.alpha, 128);

        let green_transparent = Color::new_hsla(120.0, 1.0, 0.5, 0.0);
        assert_eq!(green_transparent.red, 0);
        assert_eq!(green_transparent.green, 255);
        assert_eq!(green_transparent.blue, 0);
        assert_eq!(green_transparent.alpha, 0);

        let blue = Color::new_hsla(240.0, 1.0, 0.5, 1.0);
        assert_eq!(blue.red, 0);
        assert_eq!(blue.green, 0);
        assert_eq!(blue.blue, 255);
        assert_eq!(blue.alpha, 255);

		let green_transparent_clipped = Color::new_hsla(120.0, 1.0, 0.5, -0.4);
        assert_eq!(green_transparent_clipped.red, 0);
        assert_eq!(green_transparent_clipped.green, 255);
        assert_eq!(green_transparent_clipped.blue, 0);
        assert_eq!(green_transparent_clipped.alpha, 0);

		let blue_clipped = Color::new_hsla(240.0, 1.0, 0.5, 1.5);
        assert_eq!(blue_clipped.red, 0);
        assert_eq!(blue_clipped.green, 0);
        assert_eq!(blue_clipped.blue, 255);
        assert_eq!(blue_clipped.alpha, 255);
    }

    #[test]
    fn color_new_hsv() {
        let red = Color::new_hsv(0.0, 1.0, 1.0);
        assert_eq!(red.red, 255);
        assert_eq!(red.green, 0);
        assert_eq!(red.blue, 0);
        assert_eq!(red.alpha, 255);

        let green = Color::new_hsv(120.0, 1.0, 1.0);
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
        assert_eq!(green.alpha, 255);

        let blue = Color::new_hsv(240.0, 1.0, 1.0);
        assert_eq!(blue.red, 0);
        assert_eq!(blue.green, 0);
        assert_eq!(blue.blue, 255);
        assert_eq!(blue.alpha, 255);

        let black = Color::new_hsv(0.0, 0.0, 0.0);
        assert_eq!(black.red, 0);
        assert_eq!(black.green, 0);
        assert_eq!(black.blue, 0);
        assert_eq!(black.alpha, 255);

        let white = Color::new_hsv(0.0, 0.0, 1.0);
        assert_eq!(white.red, 255);
        assert_eq!(white.green, 255);
        assert_eq!(white.blue, 255);
        assert_eq!(white.alpha, 255);

        let yellow = Color::new_hsv(60.0, 1.0, 1.0);
        assert_eq!(yellow.red, 255);
        assert_eq!(yellow.green, 255);
        assert_eq!(yellow.blue, 0);
        assert_eq!(yellow.alpha, 255);

        let cyan = Color::new_hsv(180.0, 1.0, 1.0);
        assert_eq!(cyan.red, 0);
        assert_eq!(cyan.green, 255);
        assert_eq!(cyan.blue, 255);
        assert_eq!(cyan.alpha, 255);

        let magenta = Color::new_hsv(300.0, 1.0, 1.0);
        assert_eq!(magenta.red, 255);
        assert_eq!(magenta.green, 0);
        assert_eq!(magenta.blue, 255);
        assert_eq!(magenta.alpha, 255);

		let red_clipped_s_and_v = Color::new_hsv(0.0, 1.4, 1.8);
        assert_eq!(red_clipped_s_and_v.red, 255);
        assert_eq!(red_clipped_s_and_v.green, 0);
        assert_eq!(red_clipped_s_and_v.blue, 0);
        assert_eq!(red_clipped_s_and_v.alpha, 255);

		let black_clipped_s_and_v = Color::new_hsv(0.0, -10.0, -0.3);
        assert_eq!(black_clipped_s_and_v.red, 0);
        assert_eq!(black_clipped_s_and_v.green, 0);
        assert_eq!(black_clipped_s_and_v.blue, 0);
        assert_eq!(black_clipped_s_and_v.alpha, 255);

		let green_normalized = Color::new_hsv(480.0, 1.0, 1.0);
        assert_eq!(green_normalized.red, 0);
        assert_eq!(green_normalized.green, 255);
        assert_eq!(green_normalized.blue, 0);
        assert_eq!(green_normalized.alpha, 255);

		let blue_normalized = Color::new_hsv(-120.0, 1.0, 1.0);
        assert_eq!(blue_normalized.red, 0);
        assert_eq!(blue_normalized.green, 0);
        assert_eq!(blue_normalized.blue, 255);
        assert_eq!(blue_normalized.alpha, 255);
    }

    #[test]
    fn color_new_hsva() {
        let red_transparent = Color::new_hsva(0.0, 1.0, 1.0, 0.5);
        assert_eq!(red_transparent.red, 255);
        assert_eq!(red_transparent.green, 0);
        assert_eq!(red_transparent.blue, 0);
        assert_eq!(red_transparent.alpha, 128);

        let green_transparent = Color::new_hsva(120.0, 1.0, 1.0, 0.0);
        assert_eq!(green_transparent.red, 0);
        assert_eq!(green_transparent.green, 255);
        assert_eq!(green_transparent.blue, 0);
        assert_eq!(green_transparent.alpha, 0);

        let blue = Color::new_hsva(240.0, 1.0, 1.0, 1.0);
        assert_eq!(blue.red, 0);
        assert_eq!(blue.green, 0);
        assert_eq!(blue.blue, 255);
        assert_eq!(blue.alpha, 255);

		let green_transparent_clipped_alpha = Color::new_hsva(120.0, 1.0, 1.0, -0.3);
        assert_eq!(green_transparent_clipped_alpha.red, 0);
        assert_eq!(green_transparent_clipped_alpha.green, 255);
        assert_eq!(green_transparent_clipped_alpha.blue, 0);
        assert_eq!(green_transparent_clipped_alpha.alpha, 0);

		let blue_clipped_alpha = Color::new_hsva(240.0, 1.0, 1.0, 1.3);
        assert_eq!(blue_clipped_alpha.red, 0);
        assert_eq!(blue_clipped_alpha.green, 0);
        assert_eq!(blue_clipped_alpha.blue, 255);
        assert_eq!(blue_clipped_alpha.alpha, 255);
    }

    #[test]
    fn color_new_hwb() {
        let red = Color::new_hwb(0.0, 0.0, 0.0);
        assert_eq!(red.red, 255);
        assert_eq!(red.green, 0);
        assert_eq!(red.blue, 0);
        assert_eq!(red.alpha, 255);

        let green = Color::new_hwb(120.0, 0.0, 0.0);
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
        assert_eq!(green.alpha, 255);

        let blue = Color::new_hwb(240.0, 0.0, 0.0);
        assert_eq!(blue.red, 0);
        assert_eq!(blue.green, 0);
        assert_eq!(blue.blue, 255);
        assert_eq!(blue.alpha, 255);

        let black = Color::new_hwb(0.0, 0.0, 1.0);
        assert_eq!(black.red, 0);
        assert_eq!(black.green, 0);
        assert_eq!(black.blue, 0);
        assert_eq!(black.alpha, 255);

        let white = Color::new_hwb(0.0, 1.0, 0.0);
        assert_eq!(white.red, 255);
        assert_eq!(white.green, 255);
        assert_eq!(white.blue, 255);
        assert_eq!(white.alpha, 255);

        let yellow = Color::new_hwb(60.0, 0.0, 0.0);
        assert_eq!(yellow.red, 255);
        assert_eq!(yellow.green, 255);
        assert_eq!(yellow.blue, 0);
        assert_eq!(yellow.alpha, 255);

        let cyan = Color::new_hwb(180.0, 0.0, 0.0);
        assert_eq!(cyan.red, 0);
        assert_eq!(cyan.green, 255);
        assert_eq!(cyan.blue, 255);
        assert_eq!(cyan.alpha, 255);

        let magenta = Color::new_hwb(300.0, 0.0, 0.0);
        assert_eq!(magenta.red, 255);
        assert_eq!(magenta.green, 0);
        assert_eq!(magenta.blue, 255);
        assert_eq!(magenta.alpha, 255);

		let red_clipped_w_and_b = Color::new_hwb(0.0, -0.5, -10.0);
        assert_eq!(red_clipped_w_and_b.red, 255);
        assert_eq!(red_clipped_w_and_b.green, 0);
        assert_eq!(red_clipped_w_and_b.blue, 0);
        assert_eq!(red_clipped_w_and_b.alpha, 255);

		let black_clipped_b = Color::new_hwb(0.0, 0.0, 1.2);
        assert_eq!(black_clipped_b.red, 0);
        assert_eq!(black_clipped_b.green, 0);
        assert_eq!(black_clipped_b.blue, 0);
        assert_eq!(black_clipped_b.alpha, 255);

		let white_clipped_w = Color::new_hwb(0.0, 1.5, 0.0);
        assert_eq!(white_clipped_w.red, 255);
        assert_eq!(white_clipped_w.green, 255);
        assert_eq!(white_clipped_w.blue, 255);
        assert_eq!(white_clipped_w.alpha, 255);

		let green_normalized = Color::new_hwb(480.0, 0.0, 0.0);
        assert_eq!(green_normalized.red, 0);
        assert_eq!(green_normalized.green, 255);
        assert_eq!(green_normalized.blue, 0);
        assert_eq!(green_normalized.alpha, 255);

        let blue_normalized = Color::new_hwb(-120.0, 0.0, 0.0);
        assert_eq!(blue_normalized.red, 0);
        assert_eq!(blue_normalized.green, 0);
        assert_eq!(blue_normalized.blue, 255);
        assert_eq!(blue_normalized.alpha, 255);
    }

    #[test]
    fn color_new_hwba() {
        let red_transparent = Color::new_hwba(0.0, 0.0, 0.0, 0.5);
        assert_eq!(red_transparent.red, 255);
        assert_eq!(red_transparent.green, 0);
        assert_eq!(red_transparent.blue, 0);
        assert_eq!(red_transparent.alpha, 128);

        let green_transparent = Color::new_hwba(120.0, 0.0, 0.0, 0.0);
        assert_eq!(green_transparent.red, 0);
        assert_eq!(green_transparent.green, 255);
        assert_eq!(green_transparent.blue, 0);
        assert_eq!(green_transparent.alpha, 0);

        let blue = Color::new_hwba(240.0, 0.0, 0.0, 1.0);
        assert_eq!(blue.red, 0);
        assert_eq!(blue.green, 0);
        assert_eq!(blue.blue, 255);
        assert_eq!(blue.alpha, 255);

		let green_transparent_clipped_alpha = Color::new_hwba(120.0, 0.0, 0.0, -1.0);
        assert_eq!(green_transparent_clipped_alpha.red, 0);
        assert_eq!(green_transparent_clipped_alpha.green, 255);
        assert_eq!(green_transparent_clipped_alpha.blue, 0);
        assert_eq!(green_transparent_clipped_alpha.alpha, 0);

        let blue_clipped_alpha = Color::new_hwba(240.0, 0.0, 0.0, 1.3);
        assert_eq!(blue_clipped_alpha.red, 0);
        assert_eq!(blue_clipped_alpha.green, 0);
        assert_eq!(blue_clipped_alpha.blue, 255);
        assert_eq!(blue_clipped_alpha.alpha, 255);
    }

    #[test]
    fn color_new_string_known_color() {
        let red = Color::new_string("red").unwrap();
        assert_eq!(red.red, 255);
        assert_eq!(red.green, 0);
        assert_eq!(red.blue, 0);
        assert_eq!(red.alpha, 255);

        let green = Color::new_string("GREEN").unwrap();
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 128);
        assert_eq!(green.blue, 0);
        assert_eq!(green.alpha, 255);

        let blue = Color::new_string("BlUe").unwrap();
        assert_eq!(blue.red, 0);
        assert_eq!(blue.green, 0);
        assert_eq!(blue.blue, 255);
        assert_eq!(blue.alpha, 255);

        let yellow = Color::new_string("Yellow").unwrap();
        assert_eq!(yellow.red, 255);
        assert_eq!(yellow.green, 255);
        assert_eq!(yellow.blue, 0);
        assert_eq!(yellow.alpha, 255);

        let cyan = Color::new_string("cyan").unwrap();
        assert_eq!(cyan.red, 0);
        assert_eq!(cyan.green, 255);
        assert_eq!(cyan.blue, 255);
        assert_eq!(cyan.alpha, 255);

        let magenta = Color::new_string("magenta").unwrap();
        assert_eq!(magenta.red, 255);
        assert_eq!(magenta.green, 0);
        assert_eq!(magenta.blue, 255);
        assert_eq!(magenta.alpha, 255);

        let black = Color::new_string("black").unwrap();
        assert_eq!(black.red, 0);
        assert_eq!(black.green, 0);
        assert_eq!(black.blue, 0);
        assert_eq!(black.alpha, 255);

        let white = Color::new_string("white").unwrap();
        assert_eq!(white.red, 255);
        assert_eq!(white.green, 255);
        assert_eq!(white.blue, 255);
        assert_eq!(white.alpha, 255);
    }

    #[test]
    fn color_new_string_abbr_color() {
        let red = Color::new_string("RD").unwrap();
        assert_eq!(red.red, 255);
        assert_eq!(red.green, 0);
        assert_eq!(red.blue, 0);
        assert_eq!(red.alpha, 255);

        let green = Color::new_string("gn").unwrap();
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 128);
        assert_eq!(green.blue, 0);
        assert_eq!(green.alpha, 255);

        let blue = Color::new_string("Bl").unwrap();
        assert_eq!(blue.red, 0);
        assert_eq!(blue.green, 0);
        assert_eq!(blue.blue, 255);
        assert_eq!(blue.alpha, 255);

        let yellow = Color::new_string("yE").unwrap();
        assert_eq!(yellow.red, 255);
        assert_eq!(yellow.green, 255);
        assert_eq!(yellow.blue, 0);
        assert_eq!(yellow.alpha, 255);

        let purple = Color::new_string("PU").unwrap();
        assert_eq!(purple.red, 128);
        assert_eq!(purple.green, 0);
        assert_eq!(purple.blue, 128);
        assert_eq!(purple.alpha, 255);

        let black = Color::new_string("BK").unwrap();
        assert_eq!(black.red, 0);
        assert_eq!(black.green, 0);
        assert_eq!(black.blue, 0);
        assert_eq!(black.alpha, 255);

        let white = Color::new_string("WH").unwrap();
        assert_eq!(white.red, 255);
        assert_eq!(white.green, 255);
        assert_eq!(white.blue, 255);
        assert_eq!(white.alpha, 255);
    }

    #[test]
    fn color_new_string_hex() {
        let red_color = Color::new_string("#ff0000").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);

        let green_color = Color::new_string("#00FF00").unwrap();
        assert_eq!(green_color.red, 0);
        assert_eq!(green_color.green, 255);
        assert_eq!(green_color.blue, 0);

        let blue_color = Color::new_string("0000ff").unwrap();
        assert_eq!(blue_color.red, 0);
        assert_eq!(blue_color.green, 0);
        assert_eq!(blue_color.blue, 255);

        let transparent_white_color = Color::new_string("#ffffff80").unwrap();
        assert_eq!(transparent_white_color.red, 255);
        assert_eq!(transparent_white_color.green, 255);
        assert_eq!(transparent_white_color.blue, 255);
        assert_eq!(transparent_white_color.alpha, 128);

        let yellow_color = Color::new_string("#ff0").unwrap();
        assert_eq!(yellow_color.red, 255);
        assert_eq!(yellow_color.green, 255);
        assert_eq!(yellow_color.blue, 0);

        let magenta_color = Color::new_string("f0f").unwrap();
        assert_eq!(magenta_color.red, 255);
        assert_eq!(magenta_color.green, 0);
        assert_eq!(magenta_color.blue, 255);

        let transparent_black_color = Color::new_string("#0007").unwrap();
        assert_eq!(transparent_black_color.red, 0);
        assert_eq!(transparent_black_color.green, 0);
        assert_eq!(transparent_black_color.blue, 0);
        assert_eq!(transparent_black_color.alpha, 119);
    }

    #[test]
    fn color_new_string_rgb() {
        let red_color = Color::new_string("rgb(255, 0, 0)").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);

        let green_color = Color::new_string("rgb(0%, 100%, 0%)").unwrap();
        assert_eq!(green_color.red, 0);
        assert_eq!(green_color.green, 255);
        assert_eq!(green_color.blue, 0);

        let blue_color = Color::new_string("rgb(0, 0, 255, 0.5)").unwrap();
        assert_eq!(blue_color.red, 0);
        assert_eq!(blue_color.green, 0);
        assert_eq!(blue_color.blue, 255);
        assert_eq!(blue_color.alpha, 128);

        let yellow_color = Color::new_string("rgb(100%, 100%, 0%, 0.5)").unwrap();
        assert_eq!(yellow_color.red, 255);
        assert_eq!(yellow_color.green, 255);
        assert_eq!(yellow_color.blue, 0);
        assert_eq!(yellow_color.alpha, 128);
    }

    #[test]
    fn color_new_string_rgba() {
        let red_color = Color::new_string("rgba(255, 0, 0, 0.5)").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);
        assert_eq!(red_color.alpha, 128);

        let green_color = Color::new_string("rgba(0%, 100%, 0%, 0.5)").unwrap();
        assert_eq!(green_color.red, 0);
        assert_eq!(green_color.green, 255);
        assert_eq!(green_color.blue, 0);
        assert_eq!(green_color.alpha, 128);
    }

    #[test]
    fn color_new_string_gray() {
        let light_gray = Color::new_string("gray(100)").unwrap();
        assert_eq!(100, light_gray.red);
        assert_eq!(100, light_gray.green);
        assert_eq!(100, light_gray.blue);
        assert_eq!(255, light_gray.alpha);

        let dark_gray_with_alpha = Color::new_string("gray(200, 0.5)").unwrap();
        assert_eq!(200, dark_gray_with_alpha.red);
        assert_eq!(200, dark_gray_with_alpha.green);
        assert_eq!(200, dark_gray_with_alpha.blue);
        assert_eq!(128, dark_gray_with_alpha.alpha);

        let medium_gray_with_alpha = Color::new_string("gray(128, 25%)").unwrap();
        assert_eq!(128, medium_gray_with_alpha.red);
        assert_eq!(128, medium_gray_with_alpha.green);
        assert_eq!(128, medium_gray_with_alpha.blue);
        assert_eq!(64, medium_gray_with_alpha.alpha);

        let medium_gray = Color::new_string("gray(50%)").unwrap();
        assert_eq!(128, medium_gray.red);
        assert_eq!(128, medium_gray.green);
        assert_eq!(128, medium_gray.blue);
        assert_eq!(255, medium_gray.alpha);
    }

    #[test]
    fn color_new_string_cmyk() {
        let red_color = Color::new_string("cmyk(0%, 100%, 100%, 0%)").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);
        assert_eq!(red_color.alpha, 255);

        let green_color = Color::new_string("cmyk(100%, 0%, 100%, 0%)").unwrap();
        assert_eq!(green_color.red, 0);
        assert_eq!(green_color.green, 255);
        assert_eq!(green_color.blue, 0);
        assert_eq!(green_color.alpha, 255);

        let blue_color = Color::new_string("cmyk(100%, 100%, 0%, 0%)").unwrap();
        assert_eq!(blue_color.red, 0);
        assert_eq!(blue_color.green, 0);
        assert_eq!(blue_color.blue, 255);
        assert_eq!(blue_color.alpha, 255);

        let black_color = Color::new_string("cmyk(0%, 0%, 0%, 100%)").unwrap();
        assert_eq!(black_color.red, 0);
        assert_eq!(black_color.green, 0);
        assert_eq!(black_color.blue, 0);
        assert_eq!(black_color.alpha, 255);

        let white_color = Color::new_string("cmyk(0%, 0%, 0%, 0%)").unwrap();
        assert_eq!(white_color.red, 255);
        assert_eq!(white_color.green, 255);
        assert_eq!(white_color.blue, 255);
        assert_eq!(white_color.alpha, 255);

        let cyan_color = Color::new_string("cmyk(100%, 0%, 0%, 0%)").unwrap();
        assert_eq!(cyan_color.red, 0);
        assert_eq!(cyan_color.green, 255);
        assert_eq!(cyan_color.blue, 255);
        assert_eq!(cyan_color.alpha, 255);

        let magenta_color = Color::new_string("cmyk(0%, 100%, 0%, 0%)").unwrap();
        assert_eq!(magenta_color.red, 255);
        assert_eq!(magenta_color.green, 0);
        assert_eq!(magenta_color.blue, 255);
        assert_eq!(magenta_color.alpha, 255);

        let yellow_color = Color::new_string("cmyk(0%, 0%, 100%, 0%)").unwrap();
        assert_eq!(yellow_color.red, 255);
        assert_eq!(yellow_color.green, 255);
        assert_eq!(yellow_color.blue, 0);
        assert_eq!(yellow_color.alpha, 255);
    }

    #[test]
    fn color_new_string_hsl() {
        let red_color = Color::new_string("hsl(0, 100%, 50%)").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);
        assert_eq!(red_color.alpha, 255);

        let green_color = Color::new_string("hsl(120, 100%, 50%)").unwrap();
        assert_eq!(green_color.red, 0);
        assert_eq!(green_color.green, 255);
        assert_eq!(green_color.blue, 0);
        assert_eq!(green_color.alpha, 255);

        let blue_color = Color::new_string("hsl(240, 100%, 50%)").unwrap();
        assert_eq!(blue_color.red, 0);
        assert_eq!(blue_color.green, 0);
        assert_eq!(blue_color.blue, 255);
        assert_eq!(blue_color.alpha, 255);

        let black_color = Color::new_string("hsl(0, 0%, 0%)").unwrap();
        assert_eq!(black_color.red, 0);
        assert_eq!(black_color.green, 0);
        assert_eq!(black_color.blue, 0);
        assert_eq!(black_color.alpha, 255);

        let white_color = Color::new_string("hsl(0, 0%, 100%)").unwrap();
        assert_eq!(white_color.red, 255);
        assert_eq!(white_color.green, 255);
        assert_eq!(white_color.blue, 255);
        assert_eq!(white_color.alpha, 255);

        let cyan_color = Color::new_string("hsl(180, 100%, 50%)").unwrap();
        assert_eq!(cyan_color.red, 0);
        assert_eq!(cyan_color.green, 255);
        assert_eq!(cyan_color.blue, 255);
        assert_eq!(cyan_color.alpha, 255);

        let magenta_color = Color::new_string("hsl(300, 100%, 50%)").unwrap();
        assert_eq!(magenta_color.red, 255);
        assert_eq!(magenta_color.green, 0);
        assert_eq!(magenta_color.blue, 255);
        assert_eq!(magenta_color.alpha, 255);

        let yellow_color = Color::new_string("hsl(60, 100%, 50%)").unwrap();
        assert_eq!(yellow_color.red, 255);
        assert_eq!(yellow_color.green, 255);
        assert_eq!(yellow_color.blue, 0);
        assert_eq!(yellow_color.alpha, 255);
    }

    #[test]
    fn color_new_string_hsla() {
        let red_color = Color::new_string("hsla(0, 100%, 50%, 0.5)").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);
        assert_eq!(red_color.alpha, 128);

        let green_color = Color::new_string("hsla(120°, 100%, 50%, 0.5)").unwrap();
        assert_eq!(green_color.red, 0);
        assert_eq!(green_color.green, 255);
        assert_eq!(green_color.blue, 0);
        assert_eq!(green_color.alpha, 128);
    }

    #[test]
    fn color_new_string_hsv() {
        let red_color = Color::new_string("hsv(0, 100%, 100%)").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);
    }

    #[test]
    fn color_new_string_hwb() {
        let red_color = Color::new_string("hwb(0, 0%, 0%)").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);
        assert_eq!(red_color.alpha, 255);

        let green_color = Color::new_string("hwb(120, 0%, 0%)").unwrap();
        assert_eq!(green_color.red, 0);
        assert_eq!(green_color.green, 255);
        assert_eq!(green_color.blue, 0);
        assert_eq!(green_color.alpha, 255);

        let blue_color = Color::new_string("hwb(240, 0%, 0%)").unwrap();
        assert_eq!(blue_color.red, 0);
        assert_eq!(blue_color.green, 0);
        assert_eq!(blue_color.blue, 255);
        assert_eq!(blue_color.alpha, 255);

        let another_blue_color = Color::new_string("hwb(240, 26%, 50%)").unwrap();
        assert_eq!(another_blue_color.red, 66);
        assert_eq!(another_blue_color.green, 66);
        assert_eq!(another_blue_color.blue, 128);
        assert_eq!(another_blue_color.alpha, 255);
    }

    #[test]
    fn color_new_string_hwba() {
        let red_color = Color::new_string("hwba(0, 0%, 0%, 0.3)").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);
        assert_eq!(red_color.alpha, 77);

        let green_color = Color::new_string("hwba(120, 0%, 0%, 0.5)").unwrap();
        assert_eq!(green_color.red, 0);
        assert_eq!(green_color.green, 255);
        assert_eq!(green_color.blue, 0);
        assert_eq!(green_color.alpha, 128);

        let blue_color = Color::new_string("hwba(240, 0%, 0%, 0.6)").unwrap();
        assert_eq!(blue_color.red, 0);
        assert_eq!(blue_color.green, 0);
        assert_eq!(blue_color.blue, 255);
        assert_eq!(blue_color.alpha, 153);

        let another_blue_color = Color::new_string("hwba(240, 26%, 50%, 1)").unwrap();
        assert_eq!(another_blue_color.red, 66);
        assert_eq!(another_blue_color.green, 66);
        assert_eq!(another_blue_color.blue, 128);
        assert_eq!(another_blue_color.alpha, 255);
    }

    #[test]
    fn color_to_hex_string()
    {
        let red_color = Color::new_string("red").unwrap();
        assert_eq!(red_color.to_hex_string(), "#FF0000");

        let transparent_green_color = Color::new_string("#00FF0080").unwrap();
        assert_eq!(transparent_green_color.to_hex_string(), "#00FF0080");
    }

    #[test]
    fn color_to_rgb_string()
    {
        let red_color = Color::new_string("red").unwrap();
        assert_eq!(red_color.to_rgb_string(), "rgb(255, 0, 0)");

        let transparent_green_color = Color::new_string("#00FF0080").unwrap();
        assert_eq!(transparent_green_color.to_rgb_string(), "rgba(0, 255, 0, 0.5)");
    }
    
    #[test]
    fn color_to_cmyk_string()
    {
        let red_color = Color::new_string("red").unwrap();
        assert_eq!(red_color.to_cmyk_string(), "cmyk(0%, 100%, 100%, 0%)");

		let grayscaled_red_color = red_color.grayscale();
        assert_eq!(grayscaled_red_color.to_cmyk_string(), "cmyk(0%, 0%, 0%, 70.2%)");
    }

    #[test]
    fn color_to_gray_string()
    {
        let red_color = Color::new_string("red").unwrap();
        assert_eq!(red_color.to_gray_string(), "gray(76)");
    }

    #[test]
    fn color_to_hsl_string()
    {
        let red_color = Color::new_string("red").unwrap();
        assert_eq!(red_color.to_hsl_string(), "hsl(0, 100%, 50%)");

        let transparent_green_color = Color::new_string("#00FF0080").unwrap();
        assert_eq!(transparent_green_color.to_hsl_string(), "hsla(120, 100%, 50%, 0.5)");
    }

    #[test]
    fn color_to_hsv_string()
    {
        let red_color = Color::new_string("red").unwrap();
        assert_eq!(red_color.to_hsv_string(), "hsv(0, 100%, 100%)");

        let transparent_green_color = Color::new_string("#00FF0080").unwrap();
        assert_eq!(transparent_green_color.to_hsv_string(), "hsva(120, 100%, 100%, 0.5)");
    }

    #[test]
    fn color_to_hwb_string()
    {
        let red_color = Color::new_string("red").unwrap();
        assert_eq!(red_color.to_hwb_string(), "hwb(0, 0%, 0%)");

        let transparent_green_color = Color::new_string("#00FF0080").unwrap();
        assert_eq!(transparent_green_color.to_hwb_string(), "hwba(120, 0%, 0%, 0.5)");
    }

    #[test]
    fn color_colorize() {
        let white = Color::new_string("white").unwrap();
        let red_colorized = white.colorize(Color::new_string("red").unwrap());
        assert_eq!(red_colorized.to_hex_string(), "#FF0000");

        let lime_colorized = white.colorize(Color::new_string("lime").unwrap());
        assert_eq!(lime_colorized.to_hex_string(), "#00FF00");

        let blue_colorized = white.colorize(Color::new_string("blue").unwrap());
        assert_eq!(blue_colorized.to_hex_string(), "#0000FF");

        let random_color = Color::new_string("#ABCDEF").unwrap();
        let random_colorized = random_color.colorize(Color::new_string("#FEDCBA").unwrap());
        assert_eq!(random_colorized.to_hex_string(), "#AAB0AE");
    }

    #[test]
    fn color_colorize_string() {
        let white = Color::new_string("white").unwrap();
        let red_colorized = white.colorize_string("red").unwrap();
        assert_eq!(red_colorized.to_hex_string(), "#FF0000");

        let lime_colorized = white.colorize_string("lime").unwrap();
        assert_eq!(lime_colorized.to_hex_string(), "#00FF00");

        let blue_colorized = white.colorize_string("blue").unwrap();
        assert_eq!(blue_colorized.to_hex_string(), "#0000FF");

        let random_color = Color::new_string("#ABCDEF").unwrap();
        let random_colorized = random_color.colorize_string("#FEDCBA").unwrap();
        assert_eq!(random_colorized.to_hex_string(), "#AAB0AE");
    }

    #[test]
    fn color_new_gray() {
        let gray1 = Color::new_gray(100);
        assert_eq!(100, gray1.red);
        assert_eq!(100, gray1.green);
        assert_eq!(100, gray1.blue);
        assert_eq!(255, gray1.alpha);

        let gray2 = Color::new_gray(211);
        assert_eq!(211, gray2.red);
        assert_eq!(211, gray2.green);
        assert_eq!(211, gray2.blue);
        assert_eq!(255, gray2.alpha);
    }

    #[test]
    fn color_grayscale() {
        let color = Color::new_string("#FF7300").unwrap();
        let grayscaled = color.grayscale();
        assert_eq!(grayscaled.to_hex_string(), "#909090");

        let red = Color::new_string("rgb(255, 0, 0)").unwrap();
        let grayscaled_red = red.grayscale();
        assert_eq!(76, grayscaled_red.red);
        assert_eq!(76, grayscaled_red.green);
        assert_eq!(76, grayscaled_red.blue);
        assert_eq!(255, grayscaled_red.alpha);

        let green = Color::new_string("rgb(0, 255, 0)").unwrap();
        let grayscaled_green = green.grayscale();
        assert_eq!(150, grayscaled_green.red);
        assert_eq!(150, grayscaled_green.green);
        assert_eq!(150, grayscaled_green.blue);
        assert_eq!(255, grayscaled_green.alpha);

        let blue = Color::new_string("rgb(0, 0, 255)").unwrap();
        let grayscaled_blue = blue.grayscale();
        assert_eq!(29, grayscaled_blue.red);
        assert_eq!(29, grayscaled_blue.green);
        assert_eq!(29, grayscaled_blue.blue);
        assert_eq!(255, grayscaled_blue.alpha);
    }

    #[test]
    fn color_grayscale_hdtv() {
        let color = Color::new_string("#FF7300").unwrap();
        let grayscaled = color.grayscale_hdtv();
        assert_eq!(grayscaled.to_hex_string(), "#888888");

        let red = Color::new_string("rgb(255, 0, 0)").unwrap();
        let grayscaled_red = red.grayscale_hdtv();
        assert_eq!(54, grayscaled_red.red);
        assert_eq!(54, grayscaled_red.green);
        assert_eq!(54, grayscaled_red.blue);
        assert_eq!(255, grayscaled_red.alpha);

        let green = Color::new_string("rgb(0, 255, 0)").unwrap();
        let grayscaled_green = green.grayscale_hdtv();
        assert_eq!(182, grayscaled_green.red);
        assert_eq!(182, grayscaled_green.green);
        assert_eq!(182, grayscaled_green.blue);
        assert_eq!(255, grayscaled_green.alpha);

        let blue = Color::new_string("rgb(0, 0, 255)").unwrap();
        let grayscaled_blue = blue.grayscale_hdtv();
        assert_eq!(18, grayscaled_blue.red);
        assert_eq!(18, grayscaled_blue.green);
        assert_eq!(18, grayscaled_blue.blue);
        assert_eq!(255, grayscaled_blue.alpha);
    }

    #[test]
    fn color_grayscale_hdr() {
        let color = Color::new_string("#FF7300").unwrap();
        let grayscaled = color.grayscale_hdr();
        assert_eq!(grayscaled.to_hex_string(), "#919191");

        let red = Color::new_string("rgb(255, 0, 0)").unwrap();
        let grayscaled_red = red.grayscale_hdr();
        assert_eq!(67, grayscaled_red.red);
        assert_eq!(67, grayscaled_red.green);
        assert_eq!(67, grayscaled_red.blue);
        assert_eq!(255, grayscaled_red.alpha);

        let green = Color::new_string("rgb(0, 255, 0)").unwrap();
        let grayscaled_green = green.grayscale_hdr();
        assert_eq!(173, grayscaled_green.red);
        assert_eq!(173, grayscaled_green.green);
        assert_eq!(173, grayscaled_green.blue);
        assert_eq!(255, grayscaled_green.alpha);

        let blue = Color::new_string("rgb(0, 0, 255)").unwrap();
        let grayscaled_blue = blue.grayscale_hdr();
        assert_eq!(15, grayscaled_blue.red);
        assert_eq!(15, grayscaled_blue.green);
        assert_eq!(15, grayscaled_blue.blue);
        assert_eq!(255, grayscaled_blue.alpha);
    }

    #[test]
    fn color_monochrome()
    {
        let white = Color::new_string("white").unwrap();
        let monochromed_white = white.monochrome();
        assert_eq!(255, monochromed_white.red);
        assert_eq!(255, monochromed_white.green);
        assert_eq!(255, monochromed_white.blue);
        assert_eq!(255, monochromed_white.alpha);

        let black = Color::new_string("black").unwrap();
        let monochromed_black = black.monochrome();
        assert_eq!(0, monochromed_black.red);
        assert_eq!(0, monochromed_black.green);
        assert_eq!(0, monochromed_black.blue);
        assert_eq!(255, monochromed_black.alpha);
        
        let gray1 = Color::new_rgb(127, 127, 127);
        let monochromed_gray1 = gray1.monochrome();
        assert_eq!(0, monochromed_gray1.red);
        assert_eq!(0, monochromed_gray1.green);
        assert_eq!(0, monochromed_gray1.blue);
        assert_eq!(255, monochromed_gray1.alpha);

        let gray2 = Color::new_rgb(128, 128, 128);
        let grayscaled_gray2 = gray2.grayscale();
        assert_eq!(128, grayscaled_gray2.red);
        assert_eq!(128, grayscaled_gray2.green);
        assert_eq!(128, grayscaled_gray2.blue);
        assert_eq!(255, grayscaled_gray2.alpha);
        let monochromed_gray2 = gray2.monochrome();
        assert_eq!(255, monochromed_gray2.red);
        assert_eq!(255, monochromed_gray2.green);
        assert_eq!(255, monochromed_gray2.blue);
        assert_eq!(255, monochromed_gray2.alpha);
    }

    #[test]
    fn color_invert() {
        let white = Color::new_string("#FFFFFF").unwrap();
        let inverted_white = white.invert();
        assert_eq!(inverted_white.to_hex_string(), "#000000");

        let black = Color::new_string("#000000").unwrap();
        let inverted_black = black.invert();
        assert_eq!(inverted_black.to_hex_string(), "#FFFFFF");

        // some examples from: https://en.wikipedia.org/wiki/Negative_(photography)

        let color1 = Color::new_string("#550e0c").unwrap();
        let color1_inverted = color1.invert();
        assert_eq!("#AAF1F3", color1_inverted.to_hex_string());

        let color2 = Color::new_string("#ff0000").unwrap();
        let color2_inverted = color2.invert();
        assert_eq!("#00FFFF", color2_inverted.to_hex_string());

        let color3 = Color::new_string("#006400").unwrap();
        let color3_inverted = color3.invert();
        assert_eq!("#FF9BFF", color3_inverted.to_hex_string());

        let color4 = Color::new_string("#00ff00").unwrap();
        let color4_inverted = color4.invert();
        assert_eq!("#FF00FF", color4_inverted.to_hex_string());

        let color5 = Color::new_string("#0000ff").unwrap();
        let color5_inverted = color5.invert();
        assert_eq!("#FFFF00", color5_inverted.to_hex_string());

        let color6 = Color::new_string("#ff7f00").unwrap();
        let color6_inverted = color6.invert();
        assert_eq!("#0080FF", color6_inverted.to_hex_string());

        let color7 = Color::new_string("#800080").unwrap();
        let color7_inverted = color7.invert();
        assert_eq!("#7FFF7F", color7_inverted.to_hex_string());

        let color8 = Color::new_string("#ffb6c1").unwrap();
        let color8_inverted = color8.invert();
        assert_eq!("#00493E", color8_inverted.to_hex_string());

        let color9 = Color::new_string("#964b00").unwrap();
        let color9_inverted = color9.invert();
        assert_eq!("#69B4FF", color9_inverted.to_hex_string());

        let color10 = Color::new_string("#ffcc99").unwrap();
        let color10_inverted = color10.invert();
        assert_eq!("#003366", color10_inverted.to_hex_string());
    }

    #[test]
    fn color_invert_luminescence() {
        let color = Color::new_string("#FF7300").unwrap();
        let inverted_color = color.invert_luminescence();
        assert_eq!(inverted_color.to_hex_string(), "#FF7300");
    }
}
