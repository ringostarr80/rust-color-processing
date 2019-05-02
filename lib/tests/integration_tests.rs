extern crate color_processing;

use color_processing::{Color, KnownColors};

#[test]
fn color_new() {
    let default_color = Color::new();
    assert_eq!(default_color.alpha, 255);
    assert_eq!(default_color.red, 0);
    assert_eq!(default_color.green, 0);
    assert_eq!(default_color.blue, 0);
}

#[test]
fn color_default() {
    let default_color: Color = Default::default();
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
fn color_new_string_as_ref() {
    let red_str = "red";
    let red_color = Color::new_string(red_str).unwrap();
    assert_eq!(red_color.red, 255);
    assert_eq!(red_color.green, 0);
    assert_eq!(red_color.blue, 0);
    assert_eq!(red_color.alpha, 255);

    let red_string = String::from("red");
    let red_color = Color::new_string(&red_string).unwrap();
    assert_eq!(red_color.red, 255);
    assert_eq!(red_color.green, 0);
    assert_eq!(red_color.blue, 0);
    assert_eq!(red_color.alpha, 255);

    let red_color = Color::new_string(red_string).unwrap();
    assert_eq!(red_color.red, 255);
    assert_eq!(red_color.green, 0);
    assert_eq!(red_color.blue, 0);
    assert_eq!(red_color.alpha, 255);
}

#[test]
fn color_new_temperature() {
    let temperature_1k = Color::new_temperature(1_000);
    let candle_light = Color::new_temperature(2_000);
    let sunset = Color::new_temperature(3_500);
    let temperature_4k = Color::new_temperature(4_000);
    let temperature_5k = Color::new_temperature(5_000);
    let daylight = Color::new_temperature(6_500);
    let temperature_7k = Color::new_temperature(7_000);
    let temperature_10k = Color::new_temperature(10_000);
    let temperature_20k = Color::new_temperature(20_000);
    let temperature_30k = Color::new_temperature(30_000);
    assert_eq!(temperature_1k.to_rgb_string(), "rgb(255, 58, 0)");
    assert_eq!(candle_light.to_hex_string(), "#FF8B14");
    assert_eq!(sunset.to_hex_string(), "#FFC38A");
    assert_eq!(temperature_4k.to_rgb_string(), "rgb(255, 208, 164)");
    assert_eq!(temperature_5k.to_rgb_string(), "rgb(255, 228, 205)");
    assert_eq!(daylight.to_hex_string(), "#FFFAFE");
    assert_eq!(temperature_7k.to_rgb_string(), "rgb(245, 243, 255)");
    assert_eq!(temperature_10k.to_rgb_string(), "rgb(204, 220, 255)");
    assert_eq!(temperature_20k.to_rgb_string(), "rgb(168, 197, 255)");
    assert_eq!(temperature_30k.to_rgb_string(), "rgb(159, 190, 255)");
}

#[test]
fn color_to_temperature() {
    let temperature_1k = Color::new_string("rgb(255, 58, 0)").unwrap();
    let candle_light = Color::new_string("#FF8B14").unwrap();
    let sunset = Color::new_string("#FFC38A").unwrap();
    let temperature_4k = Color::new_string("rgb(255, 208, 164)").unwrap();
    let temperature_5k = Color::new_string("rgb(255, 228, 205)").unwrap();
    let daylight = Color::new_string("#FFFAFE").unwrap();
    let temperature_7k = Color::new_string("rgb(245, 243, 255)").unwrap();
    let temperature_10k = Color::new_string("rgb(204, 220, 255)").unwrap();
    let temperature_20k = Color::new_string("rgb(168, 197, 255)").unwrap();
    let temperature_30k = Color::new_string("rgb(159, 190, 255)").unwrap();

    // differences comes, because of roundings of rgb-values.
    assert_eq!(temperature_1k.to_temperature(), 1_000);
    assert_eq!(candle_light.to_temperature(), 2_000);
    assert_eq!(sunset.to_temperature(), 3_486); // 3_500
    assert_eq!(temperature_4k.to_temperature(), 4_000);
    assert_eq!(temperature_5k.to_temperature(), 4_986); // 5_000
    assert_eq!(daylight.to_temperature(), 6_473); // 6_500
    assert_eq!(temperature_7k.to_temperature(), 6_969); // 7_000
    assert_eq!(temperature_10k.to_temperature(), 9_922); // 10_000
    assert_eq!(temperature_20k.to_temperature(), 19_822); // 20_000
    assert_eq!(temperature_30k.to_temperature(), 28_244); // 30_000
}

#[test]
fn color_to_hex_string() {
    let red_color = Color::new_string("red").unwrap();
    assert_eq!(red_color.to_hex_string(), "#FF0000");

    let transparent_green_color = Color::new_string("#00FF0080").unwrap();
    assert_eq!(transparent_green_color.to_hex_string(), "#00FF0080");
}

#[test]
fn color_to_rgb_string() {
    let red_color = Color::new_string("red").unwrap();
    assert_eq!(red_color.to_rgb_string(), "rgb(255, 0, 0)");

    let transparent_green_color = Color::new_string("#00FF0080").unwrap();
    assert_eq!(
        transparent_green_color.to_rgb_string(),
        "rgba(0, 255, 0, 0.5)"
    );
}

#[test]
fn color_to_cmyk_string() {
    let red_color = Color::new_string("red").unwrap();
    assert_eq!(red_color.to_cmyk_string(), "cmyk(0%, 100%, 100%, 0%)");

    let grayscaled_red_color = red_color.grayscale();
    assert_eq!(
        grayscaled_red_color.to_cmyk_string(),
        "cmyk(0%, 0%, 0%, 70.2%)"
    );
}

#[test]
fn color_to_gray_string() {
    let red_color = Color::new_string("red").unwrap();
    assert_eq!(red_color.to_gray_string(), "gray(76)");
}

#[test]
fn color_to_hsl_string() {
    let red_color = Color::new_string("red").unwrap();
    assert_eq!(red_color.to_hsl_string(), "hsl(0, 100%, 50%)");

    let transparent_green_color = Color::new_string("#00FF0080").unwrap();
    assert_eq!(
        transparent_green_color.to_hsl_string(),
        "hsla(120, 100%, 50%, 0.5)"
    );
}

#[test]
fn color_to_hsv_string() {
    let red_color = Color::new_string("red").unwrap();
    assert_eq!(red_color.to_hsv_string(), "hsv(0, 100%, 100%)");

    let transparent_green_color = Color::new_string("#00FF0080").unwrap();
    assert_eq!(
        transparent_green_color.to_hsv_string(),
        "hsva(120, 100%, 100%, 0.5)"
    );
}

#[test]
fn color_to_hwb_string() {
    let red_color = Color::new_string("red").unwrap();
    assert_eq!(red_color.to_hwb_string(), "hwb(0, 0%, 0%)");

    let transparent_green_color = Color::new_string("#00FF0080").unwrap();
    assert_eq!(
        transparent_green_color.to_hwb_string(),
        "hwba(120, 0%, 0%, 0.5)"
    );
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
fn color_monochrome() {
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

#[test]
fn color_rgb_2_lab() {
    let black = Color::new_string("black").unwrap();
    let black_lab = black.get_laba();
    assert_eq!(black_lab.0, 0.0);
    assert_eq!(black_lab.1, 0.0);
    assert_eq!(black_lab.2, 0.0);

    let white = Color::new_string("white").unwrap();
    let white_lab = white.get_laba();
    assert_eq!(white_lab.0, 100.0);
    assert_eq!(white_lab.1, 0.0);
    assert_eq!(white_lab.2, 0.0);

    let gray = Color::new_string("gray").unwrap();
    let gray_lab = gray.get_laba();
    assert_eq!(gray_lab.0, 53.59);
    assert_eq!(gray_lab.1, 0.0);
    assert_eq!(gray_lab.2, 0.0);

    let red = Color::new_string("red").unwrap();
    let red_lab = red.get_laba();
    assert_eq!(red_lab.0, 53.24);
    assert_eq!(red_lab.1, 80.09);
    assert_eq!(red_lab.2, 67.2);

    let yellow = Color::new_string("yellow").unwrap();
    let yellow_lab = yellow.get_laba();
    assert_eq!(yellow_lab.0, 97.14);
    assert_eq!(yellow_lab.1, -21.55);
    assert_eq!(yellow_lab.2, 94.48);

    let green = Color::new_string("rgb(0, 255, 0)").unwrap();
    let green_lab = green.get_laba();
    assert_eq!(green_lab.0, 87.73);
    assert_eq!(green_lab.1, -86.18);
    assert_eq!(green_lab.2, 83.18);

    let cyan = Color::new_string("cyan").unwrap();
    let cyan_lab = cyan.get_laba();
    assert_eq!(cyan_lab.0, 91.11);
    assert_eq!(cyan_lab.1, -48.09);
    assert_eq!(cyan_lab.2, -14.13);

    let blue = Color::new_string("blue").unwrap();
    let blue_lab = blue.get_laba();
    assert_eq!(blue_lab.0, 32.3);
    assert_eq!(blue_lab.1, 79.19);
    assert_eq!(blue_lab.2, -107.86);

    let magenta = Color::new_string("magenta").unwrap();
    let magenta_lab = magenta.get_laba();
    assert_eq!(magenta_lab.0, 60.32);
    assert_eq!(magenta_lab.1, 98.23);
    assert_eq!(magenta_lab.2, -60.82);
}

#[test]
fn color_rgb_2_lch() {
    let black = Color::new_string("black").unwrap();
    let black_lch = black.get_lcha();
    assert_eq!(black_lch.0, 0.0);
    assert_eq!(black_lch.1, 0.0);
    assert_eq!(black_lch.2.is_nan(), true);

    let white = Color::new_string("white").unwrap();
    let white_lch = white.get_lcha();
    assert_eq!(white_lch.0, 100.0);
    assert_eq!(white_lch.1, 0.0);
    assert_eq!(white_lch.2.is_nan(), true);

    let gray = Color::new_string("gray").unwrap();
    let gray_lch = gray.get_lcha();
    assert_eq!(gray_lch.0, 53.59);
    assert_eq!(gray_lch.1, 0.0);
    assert_eq!(gray_lch.2.is_nan(), true);

    let red = Color::new_string("red").unwrap();
    let red_lch = red.get_lcha();
    assert_eq!(red_lch.0, 53.24);
    assert_eq!(red_lch.1, 104.55);
    assert_eq!(red_lch.2, 40.0);

    let yellow = Color::new_string("yellow").unwrap();
    let yellow_lch = yellow.get_lcha();
    assert_eq!(yellow_lch.0, 97.14);
    assert_eq!(yellow_lch.1, 96.91);
    assert_eq!(yellow_lch.2, 102.85);

    let green = Color::new_string("rgb(0, 255, 0)").unwrap();
    let green_lch = green.get_lcha();
    assert_eq!(green_lch.0, 87.73);
    assert_eq!(green_lch.1, 119.77);
    assert_eq!(green_lch.2, 136.01);

    let cyan = Color::new_string("cyan").unwrap();
    let cyan_lch = cyan.get_lcha();
    assert_eq!(cyan_lch.0, 91.11);
    assert_eq!(cyan_lch.1, 50.12);
    assert_eq!(cyan_lch.2, 196.37);

    let blue = Color::new_string("blue").unwrap();
    let blue_lch = blue.get_lcha();
    assert_eq!(blue_lch.0, 32.3);
    assert_eq!(blue_lch.1, 133.81);
    assert_eq!(blue_lch.2, 306.29);

    let magenta = Color::new_string("magenta").unwrap();
    let magenta_lch = magenta.get_lcha();
    assert_eq!(magenta_lch.0, 60.32);
    assert_eq!(magenta_lch.1, 115.53);
    assert_eq!(magenta_lch.2, 328.24);
}

#[test]
fn color_lab_2_rgb() {
    let black_lab = Color::new_laba(0.0, 0.0, 0.0, 1.0);
    assert_eq!(black_lab.red, 0);
    assert_eq!(black_lab.green, 0);
    assert_eq!(black_lab.blue, 0);

    let white_lab = Color::new_laba(100.0, 0.0, 0.0, 1.0);
    assert_eq!(white_lab.red, 255);
    assert_eq!(white_lab.green, 255);
    assert_eq!(white_lab.blue, 255);

    let gray_lab = Color::new_laba(53.59, 0.0, 0.0, 1.0);
    assert_eq!(gray_lab.red, 128);
    assert_eq!(gray_lab.green, 128);
    assert_eq!(gray_lab.blue, 128);

    let red_lab = Color::new_laba(53.24, 80.09, 67.2, 1.0);
    assert_eq!(red_lab.red, 255);
    assert_eq!(red_lab.green, 0);
    assert_eq!(red_lab.blue, 0);

    let yellow_lab = Color::new_laba(97.14, -21.55, 94.48, 1.0);
    assert_eq!(yellow_lab.red, 255);
    assert_eq!(yellow_lab.green, 255);
    assert_eq!(yellow_lab.blue, 0);

    let green_lab = Color::new_laba(87.73, -86.18, 83.18, 1.0);
    assert_eq!(green_lab.red, 0);
    assert_eq!(green_lab.green, 255);
    assert_eq!(green_lab.blue, 0);

    let cyan_lab = Color::new_laba(91.11, -48.09, -14.13, 1.0);
    assert_eq!(cyan_lab.red, 0);
    assert_eq!(cyan_lab.green, 255);
    assert_eq!(cyan_lab.blue, 255);

    let blue_lab = Color::new_laba(32.3, 79.19, -107.86, 1.0);
    assert_eq!(blue_lab.red, 0);
    assert_eq!(blue_lab.green, 0);
    assert_eq!(blue_lab.blue, 255);

    let magenta_lab = Color::new_laba(60.32, 98.23, -60.82, 1.0);
    assert_eq!(magenta_lab.red, 255);
    assert_eq!(magenta_lab.green, 0);
    assert_eq!(magenta_lab.blue, 255);
}

#[test]
fn color_lch_2_lab() {
    let black_lch = Color::new_lcha(0.0, 0.0, std::f64::NAN, 1.0);
    let black_lab = black_lch.get_laba();
    assert_eq!(black_lab.0, 0.0);
    assert_eq!(black_lab.1, 0.0);
    assert_eq!(black_lab.2, 0.0);

    let white_lch = Color::new_lcha(100.0, 0.0, std::f64::NAN, 1.0);
    let white_lab = white_lch.get_laba();
    assert_eq!(white_lab.0, 100.0);
    assert_eq!(white_lab.1, 0.0);
    assert_eq!(white_lab.2, 0.0);

    let gray_lch = Color::new_lcha(53.59, 0.0, std::f64::NAN, 1.0);
    let gray_lab = gray_lch.get_laba();
    assert_eq!(gray_lab.0, 53.59);
    assert_eq!(gray_lab.1, 0.0);
    assert_eq!(gray_lab.2, 0.0);

    let red_lch = Color::new_lcha(53.24, 104.55, 40.0, 1.0);
    let red_lab = red_lch.get_laba();
    assert_eq!(red_lab.0, 53.24);
    assert_eq!(red_lab.1, 80.09);
    assert_eq!(red_lab.2, 67.2);

    let yellow_lch = Color::new_lcha(97.14, 96.91, 102.85, 1.0);
    let yellow_lab = yellow_lch.get_laba();
    assert_eq!(yellow_lab.0, 97.14);
    assert_eq!(yellow_lab.1, -21.55);
    assert_eq!(yellow_lab.2, 94.48);

    let green_lch = Color::new_lcha(87.73, 119.77, 136.01, 1.0);
    let green_lab = green_lch.get_laba();
    assert_eq!(green_lab.0, 87.73);
    assert_eq!(green_lab.1, -86.18);
    assert_eq!(green_lab.2, 83.18);

    let cyan_lch = Color::new_lcha(91.11, 50.12, 196.37, 1.0);
    let cyan_lab = cyan_lch.get_laba();
    assert_eq!(cyan_lab.0, 91.11);
    assert_eq!(cyan_lab.1, -48.09);
    assert_eq!(cyan_lab.2, -14.13);

    let blue_lch = Color::new_lcha(32.3, 133.81, 306.29, 1.0);
    let blue_lab = blue_lch.get_laba();
    assert_eq!(blue_lab.0, 32.3);
    assert_eq!(blue_lab.1, 79.19);
    assert_eq!(blue_lab.2, -107.86);

    let magenta_lch = Color::new_lcha(60.32, 115.53, 328.24, 1.0);
    let magenta_lab = magenta_lch.get_laba();
    assert_eq!(magenta_lab.0, 60.32);
    assert_eq!(magenta_lab.1, 98.23);
    assert_eq!(magenta_lab.2, -60.82);
}

#[test]
fn color_lch_2_rgb() {
    let black_lch = Color::new_lcha(0.0, 0.0, std::f64::NAN, 1.0);
    assert_eq!(black_lch.to_rgb_string(), "rgb(0, 0, 0)");

    let white_lch = Color::new_lcha(100.0, 0.0, std::f64::NAN, 1.0);
    assert_eq!(white_lch.to_rgb_string(), "rgb(255, 255, 255)");

    let gray_lch = Color::new_lcha(53.59, 0.0, std::f64::NAN, 1.0);
    assert_eq!(gray_lch.to_rgb_string(), "rgb(128, 128, 128)");

    let red_lch = Color::new_lcha(53.24, 104.55, 40.0, 1.0);
    assert_eq!(red_lch.to_rgb_string(), "rgb(255, 0, 0)");

    let yellow_lch = Color::new_lcha(97.14, 96.91, 102.85, 1.0);
    assert_eq!(yellow_lch.to_rgb_string(), "rgb(255, 255, 0)");

    let green_lch = Color::new_lcha(87.73, 119.78, 136.02, 1.0);
    assert_eq!(green_lch.to_rgb_string(), "rgb(0, 255, 0)");

    let cyan_lch = Color::new_lcha(91.11, 50.12, 196.38, 1.0);
    assert_eq!(cyan_lch.to_rgb_string(), "rgb(0, 255, 255)");

    let blue_lch = Color::new_lcha(32.3, 133.81, 306.28, 1.0);
    assert_eq!(blue_lch.to_rgb_string(), "rgb(0, 0, 255)");

    let magenta_lch = Color::new_lcha(60.32, 115.54, 328.23, 1.0);
    assert_eq!(magenta_lch.to_rgb_string(), "rgb(255, 0, 255)");
}

#[test]
fn color_interpolate_lch() {
    let red = Color::new_string("rgb(255, 0, 0)").unwrap();
    let green = Color::new_string("rgb(0, 255, 0)").unwrap();

    let interpolate_0 = red.interpolate_lch(green, 0.0);
    let interpolate_0_1 = red.interpolate_lch(green, 0.1);
    let interpolate_0_5 = red.interpolate_lch(green, 0.5);
    let interpolate_1 = red.interpolate_lch(green, 1.0);

    assert_eq!(interpolate_0.to_hex_string(), "#FF0000");
    assert_eq!(interpolate_0_1.to_hex_string(), "#FE4000");
    assert_eq!(interpolate_0_5.to_hex_string(), "#D7A600");
    assert_eq!(interpolate_1.to_hex_string(), "#00FF00");
}

#[test]
fn color_darken() {
    let color = Color::new_string("#ff0000").unwrap();
    let color_darkened_1 = color.darken(1.0);
    let color_darkened_2 = color.darken(2.0);
    let color_darkened_10 = color.darken(10.0);

    assert_eq!(color_darkened_1.to_hex_string(), "#C20000");
    assert_eq!(color_darkened_2.to_hex_string(), "#890000");
    assert_eq!(color_darkened_10.to_hex_string(), "#000000");
}

#[test]
fn color_brighten() {
    let color = Color::new_string("#ff0000").unwrap();
    let color_brightened_1 = color.brighten(1.0);
    let color_brightened_10 = color.brighten(10.0);

    assert_eq!(color_brightened_1.to_hex_string(), "#FF5A36");
    assert_eq!(color_brightened_10.to_hex_string(), "#FFFFFF");
}

#[test]
fn color_to_number() {
    let black = Color::new_string("#000000").unwrap();
    let blue = Color::new_string("#0000ff").unwrap();
    let green = Color::new_string("#00ff00").unwrap();
    let red = Color::new_string("#ff0000").unwrap();
    let half_transparent_black = Color::new_string("#00000080").unwrap();

    assert_eq!(black.to_number(), -16_777_216);
    assert_eq!(black.to_number(), 0xff000000_u32 as i32);
    assert_eq!(blue.to_number(), -16_776_961);
    assert_eq!(blue.to_number(), 0xff0000ff_u32 as i32);
    assert_eq!(green.to_number(), -16_711_936);
    assert_eq!(green.to_number(), 0xff00ff00_u32 as i32);
    assert_eq!(red.to_number(), -65_536);
    assert_eq!(red.to_number(), 0xffff0000_u32 as i32);
    assert_eq!(half_transparent_black.to_number(), -2_147_483_648);
    assert_eq!(half_transparent_black.to_number(), 0x80000000_u32 as i32);
}

#[test]
fn color_from_number() {
    let black_value = -16_777_216;
    let black_value_hex = 0xff000000_u32 as i32;
    let blue_value = -16_776_961;
    let blue_value_hex = 0xff0000ff_u32 as i32;
    let green_value = -16_711_936;
    let green_value_hex = 0xff00ff00_u32 as i32;
    let red_value = -65_536;
    let red_value_hex = 0xffff0000_u32 as i32;
    let half_transparent_black_value = -2_147_483_648;
    let half_transparent_black_value_hex = 0x80000000_u32 as i32;
    let black = Color::from(black_value);
    let black_from_hex = Color::from(black_value_hex);
    let blue = Color::from(blue_value);
    let blue_from_hex = Color::from(blue_value_hex);
    let green: Color = green_value.into();
    let green_from_hex: Color = green_value_hex.into();
    let red: Color = red_value.into();
    let red_from_hex: Color = red_value_hex.into();
    let half_transparent_black = Color::from(half_transparent_black_value);
    let half_transparent_black_from_hex = Color::from(half_transparent_black_value_hex);

    assert_eq!(black_from_hex.to_hex_string(), "#000000");
    assert_eq!(black.to_hex_string(), "#000000");
    assert_eq!(blue_from_hex.to_hex_string(), "#0000FF");
    assert_eq!(blue.to_hex_string(), "#0000FF");
    assert_eq!(green_from_hex.to_hex_string(), "#00FF00");
    assert_eq!(green.to_hex_string(), "#00FF00");
    assert_eq!(red_from_hex.to_hex_string(), "#FF0000");
    assert_eq!(red.to_hex_string(), "#FF0000");
    assert_eq!(half_transparent_black_from_hex.to_hex_string(), "#00000080");
    assert_eq!(half_transparent_black.to_hex_string(), "#00000080");
}

#[test]
fn color_get_luminance() {
    let white = Color::new_string("white").unwrap();
    let aquamarine = Color::new_string("aquamarine").unwrap();
    let hotpink = Color::new_string("hotpink").unwrap();
    let darkslateblue = Color::new_string("darkslateblue").unwrap();
    let black = Color::new_string("black").unwrap();

    assert_eq!(white.get_luminance(), 1.0);
    assert_eq!(aquamarine.get_luminance(), 0.8078549208338043);
    assert_eq!(hotpink.get_luminance(), 0.3465843816971475);
    assert_eq!(darkslateblue.get_luminance(), 0.06579284622798763);
    assert_eq!(black.get_luminance(), 0.0);
}

#[test]
fn color_get_contrast() {
    let pink = Color::new_string("pink").unwrap();
    let hotpink = Color::new_string("hotpink").unwrap();
    let purple = Color::new_string("purple").unwrap();

    assert_eq!(pink.get_contrast(hotpink), 1.7214765344592284);
    assert_eq!(pink.get_contrast(purple), 6.124225406859997);
}
