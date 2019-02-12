# Purpose

This rust library is intended to do some processing of color values.  
It can parse strings in different formats (known color names, hex, rgb, cmyk, hsl, ...) and output color values in different formats. It can also do some basic modifications, like grayscale and colorization.

This library cannot modify images.

## Usage

To use `color_processing`, first add this to your `Cargo.toml`:

```toml
[dependencies]
color_processing = "0.2"
```

Next, add this to your crate:

```rust
extern crate color_processing;

use color_processing::Color;

fn main() {
    // ...
}
```

# Documentation

For the latest documentation and examples, please go to [https://docs.rs/color_processing](https://docs.rs/color_processing).

# Miscellaneous

If you have suggestions or found an error, feel free to open an [issue](https://github.com/ringostarr80/rust-color-processing/issues) or create a [pull request](https://github.com/ringostarr80/rust-color-processing/pulls) on github.
