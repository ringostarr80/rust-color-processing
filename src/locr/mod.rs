extern crate regex;

use self::regex::Regex;

pub struct Color {
	pub alpha: u8,
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

impl Color {
	pub fn new() -> Color {
		Color {
			alpha: 255,
			red: 0,
			green: 0,
			blue: 0
		}
	}

	pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
		Color {
			alpha: 255,
			red,
			green,
			blue
		}
	}

	pub fn new_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
		Color {
			alpha,
			red,
			green,
			blue
		}
	}

	pub fn new_hsl<'a>(h: f64, s: f64, l: f64) -> Result<Color, &'a str> {
		match Color::get_rgb_from_hsl(h, s, l) {
			Ok(rgb) => {
				Ok(Color {
					alpha: 255,
					red: rgb.0,
					green: rgb.1,
					blue: rgb.2
				})
			},
			Err(message) => Err(message)
		}
	}

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

	pub fn new_string(string: &str) -> Option<Color> {
		let mut color = Color::try_parse_known_color(string);
		if color.is_none() {
			color = Color::try_parse_abbr_color(string);
		}
		if color.is_none() {
			color = Color::try_parse_hex(string);
		}
		if color.is_none() {
			color = Color::try_parse_short_hex(string);
		}
		if color.is_none() {
			color = Color::try_parse_rgb(string);
		}
		if color.is_none() {
			color = Color::try_parse_rgba(string);
		}
		if color.is_none() {
			color = Color::try_parse_cmyk(string);
		}
		if color.is_none() {
			color = Color::try_parse_hsl(string);
		}
		if color.is_none() {
			color = Color::try_parse_hsla(string);
		}
		if color.is_none() {
			color = Color::try_parse_hwb(string);
		}
		if color.is_none() {
			color = Color::try_parse_hwba(string);
		}

		return color;
	}

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

		(h, s, l, self.alpha as f64 / 255.0)
	}

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

		(h, white, black, self.alpha as f64 / 255.0)
	}

	pub fn get_rgba(&self) -> (f64, f64, f64, f64) {
		(self.red as f64 / 255.0, self.green as f64 / 255.0, self.blue as f64 / 255.0, self.alpha as f64 / 255.0)
	}

	fn get_rgb_from_hsl<'a>(h: f64, s: f64, l: f64) -> Result<(u8, u8, u8), &'a str> {
		if h < 0.0 || h > 360.0 {
			return Err("h must be between 0.0 and 360.0.");
		}
		if s < 0.0 || s > 1.0 {
			return Err("s must be between 0.0 and 1.0.");
		}
		if l < 0.0 || l > 1.0 {
			return Err("l must be between 0.0 and 1.0.");
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

		Ok((r, g, b))
	}

	fn get_rgb_from_hwb<'a>(mut h: f64, mut w: f64, mut b: f64) -> Result<(u8, u8, u8), &'a str> {
		if h < 0.0 || h > 360.0 {
			return Err("h must be between 0.0 and 360.0.");
		}

		w = w / 100.0;
		b = b / 100.0;
		if w < 0.0 || w > 1.0 {
			return Err("w must be between 0.0 and 1.0.");
		}
		if b < 0.0 || b > 1.0 {
			return Err("b must be between 0.0 and 1.0.");
		}

		h = h / 60.0;
		let v = 1.0 - b;
		let i = h.floor() as u8;
		let mut f = h - (i as f64);
		if i % 1 != 0 {
			f = 1.0 - f;
		}
		let n = w + f * (v - w);

		let mut r1: f64 = 0.0;
		let mut g1: f64 = 0.0;
		let mut b1: f64 = 0.0;
		if h >= 0.0 && h <= 1.0 {
			r1 = v;
			g1 = n;
			b1 = w;
		} else if h >= 1.0 && h <= 2.0 {
			r1 = n;
			g1 = v;
			b1 = w;
		} else if h >= 2.0 && h <= 3.0 {
			r1 = w;
			g1 = v;
			b1 = n;
		} else if h >= 3.0 && h <= 4.0 {
			r1 = w;
			g1 = n;
			b1 = v;
		} else if h >= 4.0 && h <= 5.0 {
			r1 = n;
			g1 = w;
			b1 = v;
		} else if h >= 5.0 && h <= 6.0 {
			r1 = v;
			g1 = w;
			b1 = n;
		}

		let r = (r1 * 255.0).round() as u8;
		let g = (g1 * 255.0).round() as u8;
		let b = (b1 * 255.0).round() as u8;

		Ok((r, g, b))
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

	pub fn grayscale(&self) -> Color {
		let gray_value = (self.red as f64 * 0.299 + self.green as f64 * 0.587 + self.blue as f64 * 0.114).floor() as u8;
		Color {alpha: self.alpha, red: gray_value, green: gray_value, blue: gray_value}
	}

	pub fn invert(&self) -> Color {
		Color {alpha: self.alpha, red: 255 - self.red, green: 255 - self.green, blue: 255 - self.blue}
	}

	pub fn invert_luminescence(&self) -> Color {
		let hsla = self.get_hsla();
		Color::new_hsl(hsla.0, hsla.1, 1.0 - hsla.2).unwrap()
	}

	pub fn to_hex_string(&self) -> String {
		let mut hex = String::from("#");
		if self.alpha != 255 {
			hex.push_str(format!("{:01$X}", self.alpha, 2).as_str());
		}
		hex.push_str(format!("{:01$X}", self.red, 2).as_str());
		hex.push_str(format!("{:01$X}", self.green, 2).as_str());
		hex.push_str(format!("{:01$X}", self.blue, 2).as_str());
		hex
	}

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

	pub fn to_cmyk_string(&self) -> String {
		let cmyk = self.get_cmyk();

		let mut cmyk_string = String::from("cmyk(");
		cmyk_string.push_str(format!("{}%, {}%, {}%, {}%", cmyk.0 * 100.0, cmyk.1 * 100.0, cmyk.2 * 100.0, cmyk.3).as_str());
		cmyk_string.push_str(")");
		cmyk_string
	}

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
			hsl_string.push_str(format!(", {}", ((self.alpha as f64) / 255.0 * 100.0).round() / 100.0).as_str());
		}
		hsl_string.push_str(")");
		hsl_string
	}

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

	fn try_parse_cmyk(string: &str) -> Option<Color> {
		lazy_static! {
			static ref re_cmyk: Regex = Regex::new(r"^cmyk\s*\(\s*(\d+(\.\d+)?)\s*%?\s*,\s*(\d+(\.\d+)?)\s*%?\s*,\s*(\d+(\.\d+)?)\s*%?\s*,\s*(\d+(\.\d+)?)\s*%?\s*\)$").unwrap();
		}
		let caps = re_cmyk.captures(string);
		match caps {
			Some(cap) => {
				let cyan: f64 = String::from(&cap[1]).parse().unwrap();
				let magenta: f64 = String::from(&cap[3]).parse().unwrap();
				let yellow: f64 = String::from(&cap[5]).parse().unwrap();
				let black: f64 = String::from(&cap[7]).parse().unwrap();
				if cyan > 100.0 || magenta > 100.0 || yellow > 100.0 || black > 100.0 {
					return None;
				}

				let r = (255.0 * (1.0 - cyan / 100.0) * (1.0 - black / 100.0)).round() as u8;
				let g = (255.0 * (1.0 - magenta / 100.0) * (1.0 - black / 100.0)).round() as u8;
				let b = (255.0 * (1.0 - yellow / 100.0) * (1.0 - black / 100.0)).round() as u8;

				Some(Color::new_rgb(r, g, b))
			},
			None => None
		}
	}

	fn try_parse_hex(string: &str) -> Option<Color> {
		lazy_static! {
			static ref re_hex: Regex = Regex::new(r"^#?([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})?$").unwrap();
		}
		let caps = re_hex.captures(string);
		match caps {
			Some(cap) => {
				let mut a = 255;
				let r_index = match cap.get(4) {
					Some(_) => {
						a = u8::from_str_radix(&cap[1], 16).unwrap();
						2
					},
					None => 1
				};
				let r = u8::from_str_radix(&cap[r_index], 16).unwrap();
				let g = u8::from_str_radix(&cap[r_index + 1], 16).unwrap();
				let b = u8::from_str_radix(&cap[r_index + 2], 16).unwrap();

				Some(Color::new_rgba(r, g, b, a))
			},
			None => None
		}
	}

	fn try_parse_short_hex(string: &str) -> Option<Color> {
		lazy_static! {
			static ref re_short_hex: Regex = Regex::new(r"^#?([0-9a-fA-F]{1})([0-9a-fA-F]{1})([0-9a-fA-F]{1})([0-9a-fA-F]{1})?$").unwrap();
		}
		let caps = re_short_hex.captures(string);
		match caps {
			Some(cap) => {
				let mut a = 255;
				let r_index = match cap.get(4) {
					Some(_) => {
						let mut a_hex = String::from(&cap[1]);
						let a_hex2 = a_hex.clone();
						a_hex.push_str(&a_hex2);
						a = u8::from_str_radix(a_hex.as_str(), 16).unwrap();
						2
					},
					None => 1
				};

				let mut r_hex = String::from(&cap[r_index]);
				let r_hex2 = r_hex.clone();
				r_hex.push_str(&r_hex2);
				let mut g_hex = String::from(&cap[r_index + 1]);
				let g_hex2 = g_hex.clone();
				g_hex.push_str(&g_hex2);
				let mut b_hex = String::from(&cap[r_index + 2]);
				let b_hex2 = b_hex.clone();
				b_hex.push_str(&b_hex2);

				let r = u8::from_str_radix(r_hex.as_str(), 16).unwrap();
				let g = u8::from_str_radix(g_hex.as_str(), 16).unwrap();
				let b = u8::from_str_radix(b_hex.as_str(), 16).unwrap();
				Some(Color::new_rgba(r, g, b, a))
			},
			None => None
		}
	}

	fn try_parse_hsl(string: &str) -> Option<Color> {
		lazy_static! {
			static ref re_hsl: Regex = Regex::new(r"^hsl\s*\(\s*(\d{1,3})\s*,\s*(\d+(\.\d+)?)\s*%?\s*,\s*(\d+(\.\d+)?)\s*%?\s*\)$").unwrap();
		}
		let caps = re_hsl.captures(string);
		match caps {
			Some(cap) => {
				let mut h: f64 = String::from(&cap[1]).parse().unwrap();
				let mut s: f64 = String::from(&cap[2]).parse().unwrap();
				let mut l: f64 = String::from(&cap[4]).parse().unwrap();
				let rgb_result = Color::get_rgb_from_hsl(h, s / 100.0, l / 100.0);
				match rgb_result {
					Ok(rgb) => Some(Color::new_rgb(rgb.0, rgb.1, rgb.2)),
					Err(_) => None
				}
			},
			None => None
		}
	}

	fn try_parse_hsla(string: &str) -> Option<Color> {
		lazy_static! {
			static ref re_hsla: Regex = Regex::new(r"^hsla\s*\(\s*(\d{1,3})\s*,\s*(\d+(\.\d+)?)\s*%?\s*,\s*(\d+(\.\d+)?)\s*%?\s*,\s*(\d+(\.\d+)?)\s*\)$").unwrap();
		}
		let caps = re_hsla.captures(string);
		match caps {
			Some(cap) => {
				let mut h: f64 = String::from(&cap[1]).parse().unwrap();
				let mut s: f64 = String::from(&cap[2]).parse().unwrap();
				let mut l: f64 = String::from(&cap[4]).parse().unwrap();
				let a_float: f64 = String::from(&cap[6]).parse().unwrap();
				if a_float < 0.0 || a_float > 1.0 {
					return None;
				}
				let a = (a_float * 255.0).round() as u8;
				let rgb_result = Color::get_rgb_from_hsl(h, s / 100.0, l / 100.0);
				match rgb_result {
					Ok(rgb) => Some(Color::new_rgba(rgb.0, rgb.1, rgb.2, a)),
					Err(_) => None
				}
			},
			None => None
		}
	}

	fn try_parse_hwb(string: &str) -> Option<Color> {
		lazy_static! {
			static ref re_hwb: Regex = Regex::new(r"^hwb\s*\(\s*(\d{1,3})\s*,\s*(\d+(\.\d+)?)\s*%?\s*,\s*(\d+(\.\d+)?)\s*%?\s*\)$").unwrap();
		}
		let caps = re_hwb.captures(string);
		match caps {
			Some(cap) => {
				let mut h: f64 = String::from(&cap[1]).parse().unwrap();
				let mut w: f64 = String::from(&cap[2]).parse().unwrap();
				let mut b: f64 = String::from(&cap[4]).parse().unwrap();
				let rgb_result = Color::get_rgb_from_hwb(h, w, b);
				match rgb_result {
					Ok(rgb) => Some(Color::new_rgb(rgb.0, rgb.1, rgb.2)),
					Err(_) => None
				}
			},
			None => None
		}
	}

	fn try_parse_hwba(string: &str) -> Option<Color> {
		lazy_static! {
			static ref re_hwba: Regex = Regex::new(r"^hwba\s*\(\s*(\d{1,3})\s*,\s*(\d+(\.\d+)?)\s*%?\s*,\s*(\d+(\.\d+)?)\s*%?\s*,\s*(\d+(\.\d+)?)\s*\)$").unwrap();
		}
		let caps = re_hwba.captures(string);
		match caps {
			Some(cap) => {
				let mut h: f64 = String::from(&cap[1]).parse().unwrap();
				let mut w: f64 = String::from(&cap[2]).parse().unwrap();
				let mut b: f64 = String::from(&cap[4]).parse().unwrap();
				let a_float: f64 = String::from(&cap[6]).parse().unwrap();
				if a_float < 0.0 || a_float > 1.0 {
					return None;
				}
				let a = (a_float * 255.0).round() as u8;
				let rgb_result = Color::get_rgb_from_hwb(h, w, b);
				match rgb_result {
					Ok(rgb) => Some(Color::new_rgba(rgb.0, rgb.1, rgb.2, a)),
					Err(_) => None
				}
			},
			None => None
		}
	}

	fn try_parse_rgb(string: &str) -> Option<Color> {
		lazy_static! {
			static ref re_rgb: Regex = Regex::new(r"^rgb\s*\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*\)$").unwrap();
		}
		let caps = re_rgb.captures(string);
		match caps {
			Some(cap) => {
				let r: u8 = String::from(&cap[1]).parse().unwrap();
				let g: u8 = String::from(&cap[2]).parse().unwrap();
				let b: u8 = String::from(&cap[3]).parse().unwrap();
				Some(Color::new_rgb(r, g, b))
			},
			None => None
		}
	}

	fn try_parse_rgba(string: &str) -> Option<Color> {
		lazy_static! {
			static ref re_rgba: Regex = Regex::new(r"^rgba\s*\(\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d{1,3})\s*,\s*(\d(\.\d+)?)\s*\)$").unwrap();
		}
		let caps = re_rgba.captures(string);
		match caps {
			Some(cap) => {
				let r: u8 = String::from(&cap[1]).parse().unwrap();
				let g: u8 = String::from(&cap[2]).parse().unwrap();
				let b: u8 = String::from(&cap[3]).parse().unwrap();
				let a_float: f64 = String::from(&cap[4]).parse().unwrap();
				if a_float < 0.0 || a_float > 1.0 {
					return None;
				}
				let a = (a_float * 255.0).round() as u8;
				Some(Color::new_rgba(r, g, b, a))
			},
			None => None
		}
	}

	fn try_parse_abbr_color(string: &str) -> Option<Color> {
		match string.to_uppercase().as_ref() {
			"BK" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x00}),
			"WH" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0xFF}),
			"GR" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x80, blue: 0x80}),
			"SI" => Some(Color {alpha: 0xFF, red: 0xC0, green: 0xC0, blue: 0xC0}),
			"MR" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x00, blue: 0x00}),
			"RD" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0x00}),
			"GN" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x80, blue: 0x00}),
			"LI" => Some(Color {alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0x00}),
			"OL" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x80, blue: 0x00}),
			"YE" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0xFF, blue: 0x00}),
			"NA" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0x80}),
			"BL" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x00, blue: 0xFF}),
			"PU" => Some(Color {alpha: 0xFF, red: 0x80, green: 0x00, blue: 0x80}),
			"FU" => Some(Color {alpha: 0xFF, red: 0xFF, green: 0x00, blue: 0xFF}),
			"TE" => Some(Color {alpha: 0xFF, red: 0x00, green: 0x80, blue: 0x80}),
			"AQ" => Some(Color {alpha: 0xFF, red: 0x00, green: 0xFF, blue: 0xFF}),
			_ => None
		}
	}

	fn try_parse_known_color(string: &str) -> Option<Color> {
		match string.to_lowercase().as_ref() {
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
