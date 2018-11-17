//! # Color crate
//! 
//! This `color_processing` crate contains functions/methods to handle color values,
//! like converting a rgb color to another colorspace (cmyk, hsl, hwb, ...),
//! parse different kinds of color-strings or
//! modifying colors (inverting, grayscale, colorize, ...).
//! 
//! It's not intended for image manipulation, just for parsing and processing single colors.

#[macro_use] extern crate lazy_static;

extern crate regex;

use self::regex::Regex;
use std::f64::consts::PI;
use std::str::FromStr;

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
			KnownColors::AliceBlue => Color {alpha: 0xFF, red: 0xF0, green: 0xF8, blue: 0xFF},
			KnownColors::AntiqueWhite  => Color {alpha: 0xFF, red: 0xFA, green: 0xEB, blue: 0xD7},
			KnownColors::Aqua => Color {alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0xFF},
			KnownColors::AquaMarine => Color {alpha: 0xFF, red: 0x7F, green: 0xFF, blue: 0xD4},
			KnownColors::Azure => Color {alpha: 0xFF, red: 0xF0, green: 0xFF, blue: 0xFF},
			KnownColors::Beige => Color {alpha: 0xFF, red: 0xF5, green: 0xF5, blue: 0xDC},
			KnownColors::Bisque => Color {alpha: 0xFF, red: 0xFF, green: 0xE4, blue: 0xC4},
			KnownColors::Black => Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x00},
			KnownColors::BlanchedAlmond => Color {alpha: 0xFF, red: 0xFF, green: 0xEB, blue: 0xCD},
			KnownColors::Blue => Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0xFF},
			KnownColors::BlueViolet => Color {alpha: 0xFF, red: 0x8A, green: 0x2B, blue: 0xE2},
			KnownColors::Brown => Color {alpha: 0xFF, red: 0xA5, green: 0x2A, blue: 0x2A},
			KnownColors::BurlyWood => Color {alpha: 0xFF, red: 0xDE, green: 0xB8, blue: 0x87},
			KnownColors::CadetBlue => Color {alpha: 0xFF, red: 0x5F, green: 0x9E, blue: 0xA0},
			KnownColors::Chartreuse => Color {alpha: 0xFF, red: 0x7F, green: 0xFF, blue: 0x00},
			KnownColors::Chocolate => Color {alpha: 0xFF, red: 0xD2, green: 0x69, blue: 0x1E},
			KnownColors::Coral => Color {alpha: 0xFF, red: 0xFF, green: 0x7F, blue: 0x50},
			KnownColors::CornflowerBlue => Color {alpha: 0xFF, red: 0x64, green: 0x95, blue: 0xED},
			KnownColors::Cornsilk => Color {alpha: 0xFF, red: 0xFF, green: 0xF8, blue: 0xDC},
			KnownColors::Crimson => Color {alpha: 0xFF, red: 0xDC, green: 0x14, blue: 0x3C},
			KnownColors::Cyan => Color {alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0xFF},
			KnownColors::DarkBlue => Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x8B},
			KnownColors::DarkCyan => Color {alpha: 0xFF, red: 0x00, green: 0x8B, blue: 0x8B},
			KnownColors::DarkGoldenrod => Color {alpha: 0xFF, red: 0xB8, green: 0x86, blue: 0x0B},
			KnownColors::DarkGray => Color {alpha: 0xFF, red: 0xA9, green: 0xA9, blue: 0xA9},
			KnownColors::DarkGreen => Color {alpha: 0xFF, red: 0x00, green: 0x64, blue: 0x00},
			KnownColors::DarkKhaki => Color {alpha: 0xFF, red: 0xBD, green: 0xB7, blue: 0x6B},
			KnownColors::DarkMagenta => Color {alpha: 0xFF, red: 0x8B, green: 0x00, blue: 0x8B},
			KnownColors::DarkOliveGreen => Color {alpha: 0xFF, red: 0x55, green: 0x6B, blue: 0x2F},
			KnownColors::DarkOrange => Color {alpha: 0xFF, red: 0xFF, green: 0x8C, blue: 0x00},
			KnownColors::DarkOrchid => Color {alpha: 0xFF, red: 0x99, green: 0x32, blue: 0xCC},
			KnownColors::DarkRed => Color {alpha: 0xFF, red: 0x8B, green: 0x00, blue: 0x00},
			KnownColors::DarkSalmon => Color {alpha: 0xFF, red: 0xE9, green: 0x96, blue: 0x7A},
			KnownColors::DarkSeaGreen => Color {alpha: 0xFF, red: 0x8F, green: 0xBC, blue: 0x8B},
			KnownColors::DarkSlateBlue => Color {alpha: 0xFF, red: 0x48, green: 0x3D, blue: 0x8B},
			KnownColors::DarkSlateGray => Color {alpha: 0xFF, red: 0x2F, green: 0x4F, blue: 0x4F},
			KnownColors::DarkTurquoise => Color {alpha: 0xFF, red: 0x00, green: 0xCE, blue: 0xD1},
			KnownColors::DarkViolet => Color {alpha: 0xFF, red: 0x94, green: 0x00, blue: 0xD3},
			KnownColors::DeepPink => Color {alpha: 0xFF, red: 0xFF, green: 0x14, blue: 0x93},
			KnownColors::DeepSkyBlue => Color {alpha: 0xFF, red: 0x00, green: 0xBF, blue: 0xFF},
			KnownColors::DimGray => Color {alpha: 0xFF, red: 0x69, green: 0x69, blue: 0x69},
			KnownColors::DodgerBlue => Color {alpha: 0xFF, red: 0x1E, green: 0x90, blue: 0xFF},
			KnownColors::Firebrick => Color {alpha: 0xFF, red: 0xB2, green: 0x22, blue: 0x22},
			KnownColors::FloralWhite => Color {alpha: 0xFF, red: 0xFF, green: 0xFA, blue: 0xF0},
			KnownColors::ForestGreen => Color {alpha: 0xFF, red: 0x22, green: 0x8B, blue: 0x22},
			KnownColors::Fuchsia => Color {alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0xFF},
			KnownColors::Gainsboro => Color {alpha: 0xFF, red: 0xDC, green: 0xDC, blue: 0xDC},
			KnownColors::GhostWhite => Color {alpha: 0xFF, red: 0xF8, green: 0xF8, blue: 0xFF},
			KnownColors::Gold => Color {alpha: 0xFF, red: 0xFF, green: 0xD7, blue: 0x00},
			KnownColors::Goldenrod => Color {alpha: 0xFF, red: 0xDA, green: 0xA5, blue: 0x20},
			KnownColors::Gray => Color {alpha: 0xFF, red: 0x80, green: 0x80, blue: 0x80},
			KnownColors::Green => Color {alpha: 0xFF, red: 0x00, green: 0x80, blue: 0x00},
			KnownColors::GreenYellow => Color {alpha: 0xFF, red: 0xAD, green: 0xFF, blue: 0x2F},
			KnownColors::Honeydew => Color {alpha: 0xFF, red: 0xF0, green: 0xFF, blue: 0xF0},
			KnownColors::HotPink => Color {alpha: 0xFF, red: 0xFF, green: 0x69, blue: 0xB4},
			KnownColors::IndianRed => Color {alpha: 0xFF, red: 0xCD, green: 0x5C, blue: 0x5C},
			KnownColors::Indigo => Color {alpha: 0xFF, red: 0x4B, green: 0x00, blue: 0x82},
			KnownColors::Ivory => Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0xF0},
			KnownColors::Khaki => Color {alpha: 0xFF, red: 0xF0, green: 0xE6, blue: 0x8C},
			KnownColors::Lavender => Color {alpha: 0xFF, red: 0xE6, green: 0xE6, blue: 0xFA},
			KnownColors::LavenderBlush => Color {alpha: 0xFF, red: 0xFF, green: 0xF0, blue: 0xF5},
			KnownColors::LawnGreen => Color {alpha: 0xFF, red: 0x7C, green: 0xFC, blue: 0x00},
			KnownColors::LemonChiffon => Color {alpha: 0xFF, red: 0xFF, green: 0xFA, blue: 0xCD},
			KnownColors::LightBlue => Color {alpha: 0xFF, red: 0xAD, green: 0xD8, blue: 0xE6},
			KnownColors::LightCoral => Color {alpha: 0xFF, red: 0xF0, green: 0x80, blue: 0x80},
			KnownColors::LightCyan => Color {alpha: 0xFF, red: 0xE0, green: 0xFF, blue: 0xFF},
			KnownColors::LightGoldenrodYellow => Color {alpha: 0xFF, red: 0xFA, green: 0xFA, blue: 0xD2},
			KnownColors::LightGray => Color {alpha: 0xFF, red: 0xD3, green: 0xD3, blue: 0xD3},
			KnownColors::LightGreen => Color {alpha: 0xFF, red: 0x90, green: 0xEE, blue: 0x90},
			KnownColors::LightPink => Color {alpha: 0xFF, red: 0xFF, green: 0xB6, blue: 0xC1},
			KnownColors::LightSalmon => Color {alpha: 0xFF, red: 0xFF, green: 0xA0, blue: 0x7A},
			KnownColors::LightSeaGreen => Color {alpha: 0xFF, red: 0x20, green: 0xB2, blue: 0xAA},
			KnownColors::LightSkyBlue => Color {alpha: 0xFF, red: 0x87, green: 0xCE, blue: 0xFA},
			KnownColors::LightSlateGray => Color {alpha: 0xFF, red: 0x77, green: 0x88, blue: 0x99},
			KnownColors::LightSteelBlue => Color {alpha: 0xFF, red: 0xB0, green: 0xC4, blue: 0xDE},
			KnownColors::LightYellow => Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0xE0},
			KnownColors::Lime => Color {alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0x00},
			KnownColors::LimeGreen => Color {alpha: 0xFF, red: 0x32, green: 0xCD, blue: 0x32},
			KnownColors::Linen => Color {alpha: 0xFF, red: 0xFA, green: 0xF0, blue: 0xE6},
			KnownColors::Magenta => Color {alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0xFF},
			KnownColors::Maroon => Color {alpha: 0xFF, red: 0x80, green: 0x00, blue: 0x00},
			KnownColors::MediumAquaMarine => Color {alpha: 0xFF, red: 0x66, green: 0xCD, blue: 0xAA},
			KnownColors::MediumBlue => Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0xCD},
			KnownColors::MediumOrchid => Color {alpha: 0xFF, red: 0xBA, green: 0x55, blue: 0xD3},
			KnownColors::MediumPurple => Color {alpha: 0xFF, red: 0x93, green: 0x70, blue: 0xDB},
			KnownColors::MediumSeaGreen => Color {alpha: 0xFF, red: 0x3C, green: 0xB3, blue: 0x71},
			KnownColors::MediumSlateBlue => Color {alpha: 0xFF, red: 0x7B, green: 0x68, blue: 0xEE},
			KnownColors::MediumSpringGreen => Color {alpha: 0xFF, red: 0x00, green: 0xFA, blue: 0x9A},
			KnownColors::MediumTurquoise => Color {alpha: 0xFF, red: 0x48, green: 0xD1, blue: 0xCC},
			KnownColors::MediumVioletRed => Color {alpha: 0xFF, red: 0xC7, green: 0x15, blue: 0x85},
			KnownColors::MidnightBlue => Color {alpha: 0xFF, red: 0x19, green: 0x19, blue: 0x70},
			KnownColors::MintCream => Color {alpha: 0xFF, red: 0xF5, green: 0xFF, blue: 0xFA},
			KnownColors::MistyRose => Color {alpha: 0xFF, red: 0xFF, green: 0xE4, blue: 0xE1},
			KnownColors::Moccasin => Color {alpha: 0xFF, red: 0xFF, green: 0xE4, blue: 0xB5},
			KnownColors::NavajoWhite => Color {alpha: 0xFF, red: 0xFF, green: 0xDE, blue: 0xAD},
			KnownColors::Navy => Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x80},
			KnownColors::OldLace => Color {alpha: 0xFF, red: 0xFD, green: 0xF5, blue: 0xE6},
			KnownColors::Olive => Color {alpha: 0xFF, red: 0x80, green: 0x80, blue: 0x00},
			KnownColors::OliveDrab => Color {alpha: 0xFF, red: 0x6B, green: 0x8E, blue: 0x23},
			KnownColors::Orange => Color {alpha: 0xFF, red: 0xFF, green: 0xA5, blue: 0x00},
			KnownColors::OrangeRed => Color {alpha: 0xFF, red: 0xFF, green: 0x45, blue: 0x00},
			KnownColors::Orchid => Color {alpha: 0xFF, red: 0xDA, green: 0x70, blue: 0xD6},
			KnownColors::PaleGoldenrod => Color {alpha: 0xFF, red: 0xEE, green: 0xE8, blue: 0xAA},
			KnownColors::PaleGreen => Color {alpha: 0xFF, red: 0x98, green: 0xFB, blue: 0x98},
			KnownColors::PaleTurquoise => Color {alpha: 0xFF, red: 0xAF, green: 0xEE, blue: 0xEE},
			KnownColors::PaleVioletRed => Color {alpha: 0xFF, red: 0xDB, green: 0x70, blue: 0x93},
			KnownColors::PapayaWhip => Color {alpha: 0xFF, red: 0xFF, green: 0xEF, blue: 0xD5},
			KnownColors::PeachPuff => Color {alpha: 0xFF, red: 0xFF, green: 0xDA, blue: 0xB9},
			KnownColors::Peru => Color {alpha: 0xFF, red: 0xCD, green: 0x85, blue: 0x3F},
			KnownColors::Pink => Color {alpha: 0xFF, red: 0xFF, green: 0xC0, blue: 0xCB},
			KnownColors::Plum => Color {alpha: 0xFF, red: 0xDD, green: 0xA0, blue: 0xDD},
			KnownColors::PowderBlue => Color {alpha: 0xFF, red: 0xB0, green: 0xE0, blue: 0xE6},
			KnownColors::Purple => Color {alpha: 0xFF, red: 0x80, green: 0x00, blue: 0x80},
			KnownColors::Red => Color {alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0x00},
			KnownColors::RosyBrown => Color {alpha: 0xFF, red: 0xBC, green: 0x8F, blue: 0x8F},
			KnownColors::RoyalBlue => Color {alpha: 0xFF, red: 0x41, green: 0x69, blue: 0xE1},
			KnownColors::SaddleBrown => Color {alpha: 0xFF, red: 0x8B, green: 0x45, blue: 0x13},
			KnownColors::Salmon => Color {alpha: 0xFF, red: 0xFA, green: 0x80, blue: 0x72},
			KnownColors::SandyBrown => Color {alpha: 0xFF, red: 0xF4, green: 0xA4, blue: 0x60},
			KnownColors::SeaGreen => Color {alpha: 0xFF, red: 0x2E, green: 0x8B, blue: 0x57},
			KnownColors::SeaShell => Color {alpha: 0xFF, red: 0xFF, green: 0xF5, blue: 0xEE},
			KnownColors::Sienna => Color {alpha: 0xFF, red: 0xA0, green: 0x52, blue: 0x2D},
			KnownColors::Silver => Color {alpha: 0xFF, red: 0xC0, green: 0xC0, blue: 0xC0},
			KnownColors::SkyBlue => Color {alpha: 0xFF, red: 0x87, green: 0xCE, blue: 0xEB},
			KnownColors::SlateBlue => Color {alpha: 0xFF, red: 0x6A, green: 0x5A, blue: 0xCD},
			KnownColors::SlateGray => Color {alpha: 0xFF, red: 0x70, green: 0x80, blue: 0x90},
			KnownColors::Snow => Color {alpha: 0xFF, red: 0xFF, green: 0xFA, blue: 0xFA},
			KnownColors::SpringGreen => Color {alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0x7F},
			KnownColors::SteelBlue => Color {alpha: 0xFF, red: 0x46, green: 0x82, blue: 0xB4},
			KnownColors::Tan => Color {alpha: 0xFF, red: 0xD2, green: 0xB4, blue: 0x8C},
			KnownColors::Teal => Color {alpha: 0xFF, red: 0x00, green: 0x80, blue: 0x80},
			KnownColors::Thistle => Color {alpha: 0xFF, red: 0xD8, green: 0xBF, blue: 0xD8},
			KnownColors::Tomato => Color {alpha: 0xFF, red: 0xFF, green: 0x63, blue: 0x47},
			KnownColors::Transparent => Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0xFF},
			KnownColors::Turquoise => Color {alpha: 0xFF, red: 0x40, green: 0xE0, blue: 0xD0},
			KnownColors::Violet => Color {alpha: 0xFF, red: 0xEE, green: 0x82, blue: 0xEE},
			KnownColors::Wheat => Color {alpha: 0xFF, red: 0xF5, green: 0xDE, blue: 0xB3},
			KnownColors::White => Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0xFF},
			KnownColors::WhiteSmoke => Color {alpha: 0xFF, red: 0xF5, green: 0xF5, blue: 0xF5},
			KnownColors::Yellow => Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0x00},
			KnownColors::YellowGreen => Color {alpha: 0xFF, red: 0x9A, green: 0xCD, blue: 0x32}
		}
	}

    /// Gets a new Color struct, that represents a color with the hue, saturation and lightness values.
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

    /// Gets a new Color struct, that represents a color by a string.
    /// 
	/// # Example
	/// ```
	/// use color_processing::Color;
	/// 
	/// let red = Color::new_string("red").unwrap();
	/// 
	/// assert_eq!(255, red.red);
	/// assert_eq!(0, red.green);
	/// assert_eq!(0, red.blue);
	/// assert_eq!(255, red.alpha);
    /// 
    /// let green = Color::new_string("GN").unwrap();
	/// 
	/// assert_eq!(0, green.red);
	/// assert_eq!(128, green.green);
	/// assert_eq!(0, green.blue);
	/// assert_eq!(255, green.alpha);
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
    /// 
    /// let red = Color::new_string("cmyk(0%, 100%, 100%, 0%)").unwrap();
    /// 
    /// assert_eq!(255, red.red);
	/// assert_eq!(0, red.green);
	/// assert_eq!(0, red.blue);
	/// assert_eq!(255, red.alpha);
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
		let cyan = ((1.0 - r - black) / (1.0 - black)).round();
		let magenta = ((1.0 - g - black) / (1.0 - black)).round();
		let yellow = ((1.0 - b - black) / (1.0 - black)).round();

		(cyan, magenta, yellow, black)
	}

    /// Gets a hsla tuple of the color.
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

        // round with a precision of 2 decimals.
        let alpha = (self.alpha as f64 / 255.0 * 100.0).round() / 100.0;

		(h, s, l, alpha)
	}

    /// Gets a hsva tuple of the color.
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
        // round with a precision of 2 decimals.
        let alpha = (self.alpha as f64 / 255.0 * 100.0).round() / 100.0;

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
		let mut h = if red == max {
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

		if h == std::f64::NAN {
			h = 0.0;
		}

        (h, s, v, alpha)
	}

    /// Gets a hsva tuple of the color.
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
		
		let white = if r < g && r < b {
			r
		} else if g < r && g < b {
			g
		} else {
			b
		};
		let value = if r > g && r > b {
			r
		} else if g > r && g > b {
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
		
		let mut h = (i - f / (value - white)) * 60.0;
		if h == 360.0 {
			h = 0.0;
		}

        // round with a precision of 2 decimals.
        let alpha = (self.alpha as f64 / 255.0 * 100.0).round() / 100.0;

		(h, white, black, alpha)
	}

    /// Gets a rgba tuple of the color.
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
        // round with a precision of 2 decimals.
        let alpha = (self.alpha as f64 / 255.0 * 100.0).round() / 100.0;

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

	fn get_rgb_from_cmyk<'a>(mut c: f64, mut m: f64, mut y: f64, mut k: f64) -> (u8, u8, u8) {
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
		let gray_value = (self.red as f64 * 0.299 + self.green as f64 * 0.587 + self.blue as f64 * 0.114).floor() as u8;
		Color {
            red: gray_value,
            green: gray_value,
            blue: gray_value,
            alpha: self.alpha
        }
	}

	pub fn invert(&self) -> Color {
		Color {alpha: self.alpha, red: 255 - self.red, green: 255 - self.green, blue: 255 - self.blue}
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

		let mut cmyk_string = String::from("cmyk(");
		cmyk_string.push_str(format!("{}%, {}%, {}%, {}%", cmyk.0 * 100.0, cmyk.1 * 100.0, cmyk.2 * 100.0, cmyk.3).as_str());
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

		let mut hsl_string = String::from("hsl");
		if self.alpha != 255 {
			hsl_string.push_str("a");
		}
		hsl_string.push_str("(");
		hsl_string.push_str(format!("{}, {}%, {}%", hsla.0, hsla.1 * 100.0, hsla.2 * 100.0).as_str());
		if self.alpha != 255 {
			// round with a precision of 2 decimals.
			hsl_string.push_str(format!(", {}", (hsla.3 * 100.0).round() / 100.0).as_str());
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

		let mut hsv_string = String::from("hsv");
		if hsva.3 != 1.0 {
			hsv_string.push_str("a");
		}
		hsv_string.push_str("(");
		hsv_string.push_str(format!("{}, {}%, {}%", hsva.0, hsva.1 * 100.0, hsva.2 * 100.0).as_str());
		if hsva.3 != 1.0 {
			// round with a precision of 2 decimals.
			hsv_string.push_str(format!(", {}", (hsva.3 * 100.0).round() / 100.0).as_str());
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

		let mut hwb_string = String::from("hwb");
		if self.alpha != 255 {
			hwb_string.push_str("a");
		}
		hwb_string.push_str("(");
		hwb_string.push_str(format!("{}, {}%, {}%", hwba.0.round() as u16, hwba.1, hwba.2).as_str());
		if self.alpha != 255 {
			// round with a precision of 2 decimals.
			hwb_string.push_str(format!(", {}", (hwba.3 * 100.0).round() / 100.0).as_str());
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
			static ref re_css_function: Regex = Regex::new(r"^(cmyk|rgba?|hsla?|hsva?|hwba?)\s*\(\s*(-?\d+(\.\d+)?)\s*(%|°)?\s*,\s*(-?\d+(\.\d+)?)\s*(%)?\s*,\s*(-?\d+(\.\d+)?)\s*(%)?\s*(,\s*(-?\d+(\.\d+)?)\s*(%)?\s*)?\)$").unwrap();
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
        let mut value_2: f64 = String::from(&cap[5]).parse().unwrap();
        let mut value_3: f64 = String::from(&cap[8]).parse().unwrap();
        let value_4_opt = if cap.get(12).is_some() && cap[12].len() > 0 {
            let float: f64 = String::from(&cap[12]).parse().unwrap();
            Some(float)
        } else {
            None
        };

        let get_alpha = |alpha_option: Option<f64>| -> Option<u8> {
            if alpha_option.is_some() {
                let mut alpha = alpha_option.unwrap();
                if alpha < 0.0 {
                    alpha = 0.0;
                }
                if alpha > 1.0 {
                    alpha = 1.0;
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
                if value_4_opt.is_none() {
                    return None;
                }
                let mut value_4 = value_4_opt.unwrap();
                if &cap[4] == "°" {
                    return None;
                }

                let rgb = Color::get_rgb_from_cmyk(value_1 / 100.0, value_2 / 100.0, value_3 / 100.0, value_4 / 100.0);
                Some(Color::new_rgb(rgb.0, rgb.1, rgb.2))
            },
            "rgb" => {
                if cap.get(4).is_some() && &cap[4] == "°" {
                    return None;
                }
                let is_in_percentage_mode = if cap.get(4).is_some() && &cap[4] == "%" {
                    true
                } else {
                    false
                };
                if is_in_percentage_mode {
                    if &cap[7] != "%" || &cap[10] != "%" {
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

                let alpha_opt = get_alpha(value_4_opt);
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
                if cap.get(4).is_some() && &cap[4] == "%" {
                    return None;
                }

                let alpha_opt = get_alpha(value_4_opt);
                if alpha_opt.is_none() {
                    return None;
                }

                let rgb = Color::get_rgb_from_hsl(value_1, value_2 / 100.0, value_3 / 100.0);

                Some(Color::new_rgba(rgb.0, rgb.1, rgb.2, alpha_opt.unwrap()))
            },
            "hsv" => {
                if cap.get(4).is_some() && &cap[4] == "%" {
                    return None;
                }

                let alpha_opt = get_alpha(value_4_opt);
                if alpha_opt.is_none() {
                    return None;
                }

                let rgb = Color::get_rgb_from_hsv(value_1, value_2 / 100.0, value_3 / 100.0);

                Some(Color::new_rgba(rgb.0, rgb.1, rgb.2, alpha_opt.unwrap()))
            },
            "hwb" => {
                if cap.get(4).is_some() && &cap[4] == "%" {
                    return None;
                }

                let alpha_opt = get_alpha(value_4_opt);
                if alpha_opt.is_none() {
                    return None;
                }

                let rgb = Color::get_rgb_from_hwb(value_1, value_2 / 100.0, value_3 / 100.0);

                Some(Color::new_rgba(rgb.0, rgb.1, rgb.2, alpha_opt.unwrap()))
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
			"aliceblue" => Some(Color {alpha: 0xFF, red: 0xF0, green: 0xF8, blue: 0xFF}),
			"antiquewhite"  => Some(Color {alpha: 0xFF, red: 0xFA, green: 0xEB, blue: 0xD7}),
			"aqua" => Some(Color {alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0xFF}),
			"aquamarine" => Some(Color {alpha: 0xFF, red: 0x7F, green: 0xFF, blue: 0xD4}),
			"azure" => Some(Color {alpha: 0xFF, red: 0xF0, green: 0xFF, blue: 0xFF}),
			"beige" => Some(Color {alpha: 0xFF, red: 0xF5, green: 0xF5, blue: 0xDC}),
			"bisque" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xE4, blue: 0xC4}),
			"black" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x00}),
			"blanchedalmond" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xEB, blue: 0xCD}),
			"blue" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0xFF}),
			"blueviolet" => Some(Color {alpha: 0xFF, red: 0x8A, green: 0x2B, blue: 0xE2}),
			"brown" => Some(Color {alpha: 0xFF, red: 0xA5, green: 0x2A, blue: 0x2A}),
			"burlywood" => Some(Color {alpha: 0xFF, red: 0xDE, green: 0xB8, blue: 0x87}),
			"cadetblue" => Some(Color {alpha: 0xFF, red: 0x5F, green: 0x9E, blue: 0xA0}),
			"chartreuse" => Some(Color {alpha: 0xFF, red: 0x7F, green: 0xFF, blue: 0x00}),
			"chocolate" => Some(Color {alpha: 0xFF, red: 0xD2, green: 0x69, blue: 0x1E}),
			"coral" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x7F, blue: 0x50}),
			"cornflowerblue" => Some(Color {alpha: 0xFF, red: 0x64, green: 0x95, blue: 0xED}),
			"cornsilk" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xF8, blue: 0xDC}),
			"crimson" => Some(Color {alpha: 0xFF, red: 0xDC, green: 0x14, blue: 0x3C}),
			"cyan" => Some(Color {alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0xFF}),
			"darkblue" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x8B}),
			"darkcyan" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x8B, blue: 0x8B}),
			"darkgoldenrod" => Some(Color {alpha: 0xFF, red: 0xB8, green: 0x86, blue: 0x0B}),
			"darkgray" => Some(Color {alpha: 0xFF, red: 0xA9, green: 0xA9, blue: 0xA9}),
			"darkgrey" => Some(Color {alpha: 0xFF, red: 0xA9, green: 0xA9, blue: 0xA9}),
			"darkgreen" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x64, blue: 0x00}),
			"darkkhaki" => Some(Color {alpha: 0xFF, red: 0xBD, green: 0xB7, blue: 0x6B}),
			"darkmagenta" => Some(Color {alpha: 0xFF, red: 0x8B, green: 0x00, blue: 0x8B}),
			"darkolivegreen" => Some(Color {alpha: 0xFF, red: 0x55, green: 0x6B, blue: 0x2F}),
			"darkorange" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x8C, blue: 0x00}),
			"darkorchid" => Some(Color {alpha: 0xFF, red: 0x99, green: 0x32, blue: 0xCC}),
			"darkred" => Some(Color {alpha: 0xFF, red: 0x8B, green: 0x00, blue: 0x00}),
			"darksalmon" => Some(Color {alpha: 0xFF, red: 0xE9, green: 0x96, blue: 0x7A}),
			"darkseagreen" => Some(Color {alpha: 0xFF, red: 0x8F, green: 0xBC, blue: 0x8B}),
			"darkslateblue" => Some(Color {alpha: 0xFF, red: 0x48, green: 0x3D, blue: 0x8B}),
			"darkslategray" => Some(Color {alpha: 0xFF, red: 0x2F, green: 0x4F, blue: 0x4F}),
			"darkslategrey" => Some(Color {alpha: 0xFF, red: 0x2F, green: 0x4F, blue: 0x4F}),
			"darkturquoise" => Some(Color {alpha: 0xFF, red: 0x00, green: 0xCE, blue: 0xD1}),
			"darkviolet" => Some(Color {alpha: 0xFF, red: 0x94, green: 0x00, blue: 0xD3}),
			"deeppink" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x14, blue: 0x93}),
			"deepskyblue" => Some(Color {alpha: 0xFF, red: 0x00, green: 0xBF, blue: 0xFF}),
			"dimgray" => Some(Color {alpha: 0xFF, red: 0x69, green: 0x69, blue: 0x69}),
			"dimgrey" => Some(Color {alpha: 0xFF, red: 0x69, green: 0x69, blue: 0x69}),
			"dodgerblue" => Some(Color {alpha: 0xFF, red: 0x1E, green: 0x90, blue: 0xFF}),
			"firebrick" => Some(Color {alpha: 0xFF, red: 0xB2, green: 0x22, blue: 0x22}),
			"floralwhite" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xFA, blue: 0xF0}),
			"forestgreen" => Some(Color {alpha: 0xFF, red: 0x22, green: 0x8B, blue: 0x22}),
			"fuchsia" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0xFF}),
			"gainsboro" => Some(Color {alpha: 0xFF, red: 0xDC, green: 0xDC, blue: 0xDC}),
			"ghostwhite" => Some(Color {alpha: 0xFF, red: 0xF8, green: 0xF8, blue: 0xFF}),
			"gold" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xD7, blue: 0x00}),
			"goldenrod" => Some(Color {alpha: 0xFF, red: 0xDA, green: 0xA5, blue: 0x20}),
			"gray" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x80, blue: 0x80}),
			"grey" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x80, blue: 0x80}),
			"green" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x80, blue: 0x00}),
			"greenyellow" => Some(Color {alpha: 0xFF, red: 0xAD, green: 0xFF, blue: 0x2F}),
			"honeydew" => Some(Color {alpha: 0xFF, red: 0xF0, green: 0xFF, blue: 0xF0}),
			"hotpink" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x69, blue: 0xB4}),
			"indianred" => Some(Color {alpha: 0xFF, red: 0xCD, green: 0x5C, blue: 0x5C}),
			"indigo" => Some(Color {alpha: 0xFF, red: 0x4B, green: 0x00, blue: 0x82}),
			"ivory" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0xF0}),
			"khaki" => Some(Color {alpha: 0xFF, red: 0xF0, green: 0xE6, blue: 0x8C}),
			"lavender" => Some(Color {alpha: 0xFF, red: 0xE6, green: 0xE6, blue: 0xFA}),
			"lavenderblush" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xF0, blue: 0xF5}),
			"lawngreen" => Some(Color {alpha: 0xFF, red: 0x7C, green: 0xFC, blue: 0x00}),
			"lemonchiffon" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xFA, blue: 0xCD}),
			"lightblue" => Some(Color {alpha: 0xFF, red: 0xAD, green: 0xD8, blue: 0xE6}),
			"lightcoral" => Some(Color {alpha: 0xFF, red: 0xF0, green: 0x80, blue: 0x80}),
			"lightcyan" => Some(Color {alpha: 0xFF, red: 0xE0, green: 0xFF, blue: 0xFF}),
			"lightgoldenrodyellow" => Some(Color {alpha: 0xFF, red: 0xFA, green: 0xFA, blue: 0xD2}),
			"lightgray" => Some(Color {alpha: 0xFF, red: 0xD3, green: 0xD3, blue: 0xD3}),
			"lightgrey" => Some(Color {alpha: 0xFF, red: 0xD3, green: 0xD3, blue: 0xD3}),
			"lightgreen" => Some(Color {alpha: 0xFF, red: 0x90, green: 0xEE, blue: 0x90}),
			"lightpink" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xB6, blue: 0xC1}),
			"lightsalmon" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xA0, blue: 0x7A}),
			"lightseagreen" => Some(Color {alpha: 0xFF, red: 0x20, green: 0xB2, blue: 0xAA}),
			"lightskyblue" => Some(Color {alpha: 0xFF, red: 0x87, green: 0xCE, blue: 0xFA}),
			"lightslategray" => Some(Color {alpha: 0xFF, red: 0x77, green: 0x88, blue: 0x99}),
			"lightslategrey" => Some(Color {alpha: 0xFF, red: 0x77, green: 0x88, blue: 0x99}),
			"lightsteelblue" => Some(Color {alpha: 0xFF, red: 0xB0, green: 0xC4, blue: 0xDE}),
			"lightyellow" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0xE0}),
			"lime" => Some(Color {alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0x00}),
			"limegreen" => Some(Color {alpha: 0xFF, red: 0x32, green: 0xCD, blue: 0x32}),
			"linen" => Some(Color {alpha: 0xFF, red: 0xFA, green: 0xF0, blue: 0xE6}),
			"magenta" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0xFF}),
			"maroon" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x00, blue: 0x00}),
			"mediumaquamarine" => Some(Color {alpha: 0xFF, red: 0x66, green: 0xCD, blue: 0xAA}),
			"mediumblue" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0xCD}),
			"mediumorchid" => Some(Color {alpha: 0xFF, red: 0xBA, green: 0x55, blue: 0xD3}),
			"mediumpurple" => Some(Color {alpha: 0xFF, red: 0x93, green: 0x70, blue: 0xDB}),
			"mediumseagreen" => Some(Color {alpha: 0xFF, red: 0x3C, green: 0xB3, blue: 0x71}),
			"mediumslateblue" => Some(Color {alpha: 0xFF, red: 0x7B, green: 0x68, blue: 0xEE}),
			"mediumspringgreen" => Some(Color {alpha: 0xFF, red: 0x00, green: 0xFA, blue: 0x9A}),
			"mediumturquoise" => Some(Color {alpha: 0xFF, red: 0x48, green: 0xD1, blue: 0xCC}),
			"mediumvioletred" => Some(Color {alpha: 0xFF, red: 0xC7, green: 0x15, blue: 0x85}),
			"midnightblue" => Some(Color {alpha: 0xFF, red: 0x19, green: 0x19, blue: 0x70}),
			"mintcream" => Some(Color {alpha: 0xFF, red: 0xF5, green: 0xFF, blue: 0xFA}),
			"mistyrose" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xE4, blue: 0xE1}),
			"moccasin" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xE4, blue: 0xB5}),
			"navajowhite" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xDE, blue: 0xAD}),
			"navy" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x80}),
			"oldlace" => Some(Color {alpha: 0xFF, red: 0xFD, green: 0xF5, blue: 0xE6}),
			"olive" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x80, blue: 0x00}),
			"olivedrab" => Some(Color {alpha: 0xFF, red: 0x6B, green: 0x8E, blue: 0x23}),
			"orange" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xA5, blue: 0x00}),
			"orangered" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x45, blue: 0x00}),
			"orchid" => Some(Color {alpha: 0xFF, red: 0xDA, green: 0x70, blue: 0xD6}),
			"palegoldenrod" => Some(Color {alpha: 0xFF, red: 0xEE, green: 0xE8, blue: 0xAA}),
			"palegreen" => Some(Color {alpha: 0xFF, red: 0x98, green: 0xFB, blue: 0x98}),
			"paleturquoise" => Some(Color {alpha: 0xFF, red: 0xAF, green: 0xEE, blue: 0xEE}),
			"palevioletred" => Some(Color {alpha: 0xFF, red: 0xDB, green: 0x70, blue: 0x93}),
			"papayawhip" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xEF, blue: 0xD5}),
			"peachpuff" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xDA, blue: 0xB9}),
			"peru" => Some(Color {alpha: 0xFF, red: 0xCD, green: 0x85, blue: 0x3F}),
			"pink" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xC0, blue: 0xCB}),
			"plum" => Some(Color {alpha: 0xFF, red: 0xDD, green: 0xA0, blue: 0xDD}),
			"powderBlue" => Some(Color {alpha: 0xFF, red: 0xB0, green: 0xE0, blue: 0xE6}),
			"purple" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x00, blue: 0x80}),
			"red" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0x00}),
			"rosybrown" => Some(Color {alpha: 0xFF, red: 0xBC, green: 0x8F, blue: 0x8F}),
			"royalblue" => Some(Color {alpha: 0xFF, red: 0x41, green: 0x69, blue: 0xE1}),
			"saddlebrown" => Some(Color {alpha: 0xFF, red: 0x8B, green: 0x45, blue: 0x13}),
			"salmon" => Some(Color {alpha: 0xFF, red: 0xFA, green: 0x80, blue: 0x72}),
			"sandybrown" => Some(Color {alpha: 0xFF, red: 0xF4, green: 0xA4, blue: 0x60}),
			"seagreen" => Some(Color {alpha: 0xFF, red: 0x2E, green: 0x8B, blue: 0x57}),
			"seashell" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xF5, blue: 0xEE}),
			"sienna" => Some(Color {alpha: 0xFF, red: 0xA0, green: 0x52, blue: 0x2D}),
			"silver" => Some(Color {alpha: 0xFF, red: 0xC0, green: 0xC0, blue: 0xC0}),
			"skyblue" => Some(Color {alpha: 0xFF, red: 0x87, green: 0xCE, blue: 0xEB}),
			"slateblue" => Some(Color {alpha: 0xFF, red: 0x6A, green: 0x5A, blue: 0xCD}),
			"slategray" => Some(Color {alpha: 0xFF, red: 0x70, green: 0x80, blue: 0x90}),
			"slategrey" => Some(Color {alpha: 0xFF, red: 0x70, green: 0x80, blue: 0x90}),
			"snow" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xFA, blue: 0xFA}),
			"springgreen" => Some(Color {alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0x7F}),
			"steelblue" => Some(Color {alpha: 0xFF, red: 0x46, green: 0x82, blue: 0xB4}),
			"tan" => Some(Color {alpha: 0xFF, red: 0xD2, green: 0xB4, blue: 0x8C}),
			"teal" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x80, blue: 0x80}),
			"thistle" => Some(Color {alpha: 0xFF, red: 0xD8, green: 0xBF, blue: 0xD8}),
			"tomato" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x63, blue: 0x47}),
			"transparent" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0xFF}),
			"turquoise" => Some(Color {alpha: 0xFF, red: 0x40, green: 0xE0, blue: 0xD0}),
			"violet" => Some(Color {alpha: 0xFF, red: 0xEE, green: 0x82, blue: 0xEE}),
			"wheat" => Some(Color {alpha: 0xFF, red: 0xF5, green: 0xDE, blue: 0xB3}),
			"white" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0xFF}),
			"whitesmoke" => Some(Color {alpha: 0xFF, red: 0xF5, green: 0xF5, blue: 0xF5}),
			"yellow" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0x00}),
			"yellowgreen" => Some(Color {alpha: 0xFF, red: 0x9A, green: 0xCD, blue: 0x32}),
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
    fn default_color() {
        let default_color = Color::new();
        assert_eq!(default_color.alpha, 255);
        assert_eq!(default_color.red, 0);
        assert_eq!(default_color.green, 0);
        assert_eq!(default_color.blue, 0);
    }

    #[test]
    fn color_rgb() {
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
    fn color_rgba() {
        let red_transparent = Color::new_rgba(255, 0, 0, 128);
        assert_eq!(red_transparent.red, 255);
        assert_eq!(red_transparent.green, 0);
        assert_eq!(red_transparent.blue, 0);
        assert_eq!(red_transparent.alpha, 128);
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
    fn color_grayscale() {
        let color = Color::new_string("#FF7300").unwrap();
        let grayscaled = color.grayscale();
        assert_eq!(grayscaled.to_hex_string(), "#8F8F8F");

        let red = Color::new_string("rgb(255, 0, 0)").unwrap();
        let grayscaled_red = red.grayscale();
        assert_eq!(76, grayscaled_red.red);
        assert_eq!(76, grayscaled_red.green);
        assert_eq!(76, grayscaled_red.blue);
        assert_eq!(255, grayscaled_red.alpha);

        let green = Color::new_string("rgb(0, 255, 0)").unwrap();
        let grayscaled_green = green.grayscale();
        assert_eq!(149, grayscaled_green.red);
        assert_eq!(149, grayscaled_green.green);
        assert_eq!(149, grayscaled_green.blue);
        assert_eq!(255, grayscaled_green.alpha);

        let blue = Color::new_string("rgb(0, 0, 255)").unwrap();
        let grayscaled_blue = blue.grayscale();
        assert_eq!(29, grayscaled_blue.red);
        assert_eq!(29, grayscaled_blue.green);
        assert_eq!(29, grayscaled_blue.blue);
        assert_eq!(255, grayscaled_blue.alpha);
    }

    #[test]
    fn color_invert() {
        let white = Color::new_string("#FFFFFF").unwrap();
        let inverted_white = white.invert();
        assert_eq!(inverted_white.to_hex_string(), "#000000");
    }

    #[test]
    fn color_invert_luminescence() {
        let color = Color::new_string("#FF7300").unwrap();
        let inverted_color = color.invert_luminescence();
        assert_eq!(inverted_color.to_hex_string(), "#FF7300");
    }
}
