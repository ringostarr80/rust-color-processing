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
        let color = Color::new_rgba(r, g, b, a);
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

    #[test]
    fn color_cmyk_string() {
        let random_color = Color::new_string("cmyk(0%, 55%, 100%, 0%)").unwrap();
        assert_eq!(random_color.red, 255);
        assert_eq!(random_color.green, 115);
        assert_eq!(random_color.blue, 0);
        assert_eq!(random_color.alpha, 255);
    }

    #[test]
    fn color_hsl_string() {
        let red_color = Color::new_string("hsl(0, 100%, 50%)").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);
        assert_eq!(red_color.alpha, 255);
    }

    #[test]
    fn color_hsla_string() {
        let red_color = Color::new_string("hsla(0, 100%, 50%, 0.3)").unwrap();
        assert_eq!(red_color.red, 255);
        assert_eq!(red_color.green, 0);
        assert_eq!(red_color.blue, 0);
        assert_eq!(red_color.alpha, 77);
    }

    #[test]
    fn color_hwb_string() {
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
    fn color_hwba_string() {
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

        let transparent_green_color = Color::new_string("#8000FF00").unwrap();
        assert_eq!(transparent_green_color.to_hex_string(), "#8000FF00");
    }

    #[test]
    fn color_to_rgb_string()
    {
        let red_color = Color::new_string("red").unwrap();
        assert_eq!(red_color.to_rgb_string(), "rgb(255, 0, 0)");

        let transparent_green_color = Color::new_string("#8000FF00").unwrap();
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

        let transparent_green_color = Color::new_string("#8000FF00").unwrap();
        assert_eq!(transparent_green_color.to_hsl_string(), "hsla(120, 100%, 50%, 0.5)");
    }

    #[test]
    fn color_to_hwb_string()
    {
        let red_color = Color::new_string("red").unwrap();
        assert_eq!(red_color.to_hwb_string(), "hwb(0, 0%, 0%)");

        let transparent_green_color = Color::new_string("#8000FF00").unwrap();
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
