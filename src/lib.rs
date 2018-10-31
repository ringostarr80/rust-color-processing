#[macro_use] extern crate lazy_static;

pub mod locr;

#[cfg(test)]
mod tests {
    use locr::{Color, KnownColors};

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
        let r = 1;
        let g = 2;
        let b = 3;
        let color = Color::new_rgb(r, g, b);
        assert_eq!(color.red, r);
        assert_eq!(color.green, g);
        assert_eq!(color.blue, b);
    }

    #[test]
    fn color_argb() {
        let a = 128;
        let r = 1;
        let g = 2;
        let b = 3;
        let color = Color::new_argb(a, r, g, b);
        assert_eq!(color.alpha, a);
        assert_eq!(color.red, r);
        assert_eq!(color.green, g);
        assert_eq!(color.blue, b);
    }

    #[test]
    fn color_enum() {
        let r = 255;
        let g = 0;
        let b = 0;
        let color = Color::new_enum(KnownColors::Red);
        assert_eq!(color.red, r);
        assert_eq!(color.green, g);
        assert_eq!(color.blue, b);
    }

    #[test]
    fn color_string() {
        let green_color = Color::new_string("green").unwrap();
        assert_eq!(green_color.red, 0);
        assert_eq!(green_color.green, 128);
        assert_eq!(green_color.blue, 0);

        let blue_color = Color::new_string("BL").unwrap();
        assert_eq!(blue_color.red, 0);
        assert_eq!(blue_color.green, 0);
        assert_eq!(blue_color.blue, 255);
    }

    #[test]
    fn color_hex() {
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

        let transparent_white_color = Color::new_string("#80ffffff").unwrap();
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

        let transparent_black_color = Color::new_string("#7000").unwrap();
        assert_eq!(transparent_black_color.red, 0);
        assert_eq!(transparent_black_color.green, 0);
        assert_eq!(transparent_black_color.blue, 0);
        assert_eq!(transparent_black_color.alpha, 119);
    }

    #[test]
    fn color_rgb_string() {
        let red_color = Color::new_string("rgb(255, 0, 0)").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);
    }

    #[test]
    fn color_rgba_string() {
        let red_color = Color::new_string("rgba(255, 0, 0, 0.5)").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);
        assert_eq!(red_color.alpha, 128);
    }
}
