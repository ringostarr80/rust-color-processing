extern crate color_processing;

use color_processing::Color;

#[test]
fn test_new() {
    let black = Color::new();

    assert_eq!(0, black.red);
    assert_eq!(0, black.green);
    assert_eq!(0, black.blue);
    assert_eq!(255, black.alpha);
}
