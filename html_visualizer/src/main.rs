extern crate color_processing;

use std::fs;
use std::fs::File;
use std::io::prelude::Write;
use color_processing::{Color, KnownColors};

fn main() {
    let output_directory_ok = match fs::create_dir("output") {
        Ok(_) => true,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::AlreadyExists => true,
                _ => false
            }
        }
    };
    if !output_directory_ok {
        return;
    }
    build_index_html().is_ok();
}

fn build_index_html() -> std::io::Result<()> {
    let mut index_html_content = String::new();
    index_html_content.push_str("<!DOCTYPE html>\n");
    index_html_content.push_str("<html>\n");
    index_html_content.push_str("   <head>\n");
    index_html_content.push_str("       <title>color_processing visualizer</title>\n");
    index_html_content.push_str("       <link rel=\"stylesheet\" href=\"index.css\">\n");
    index_html_content.push_str("   </head>\n");
    index_html_content.push_str("   <body>\n");
    index_html_content.push_str("       <h1>color_processing visualizer</h1>\n");
    index_html_content.push_str("       <ul>");
    index_html_content.push_str("           <li>");
    index_html_content.push_str("               <a href=\"enum-string-comparison.html\">enum-string-comparison</a><br />");
    index_html_content.push_str("           </li>");
    index_html_content.push_str("           <li>");
    index_html_content.push_str("               <a href=\"cmyk-examples.html\">cmyk examples</a>");
    index_html_content.push_str("           </li>");
    index_html_content.push_str("           <li>");
    index_html_content.push_str("               <a href=\"gray-examples.html\">gray examples</a>");
    index_html_content.push_str("           </li>");
    index_html_content.push_str("           <li>");
    index_html_content.push_str("               <a href=\"hsl-examples.html\">hsl examples</a>");
    index_html_content.push_str("           </li>");
    index_html_content.push_str("           <li>");
    index_html_content.push_str("               <a href=\"hsv-examples.html\">hsv examples</a>");
    index_html_content.push_str("           </li>");
    index_html_content.push_str("           <li>");
    index_html_content.push_str("               <a href=\"hwb-examples.html\">hwb examples</a>");
    index_html_content.push_str("           </li>");
    index_html_content.push_str("           <li>");
    index_html_content.push_str("               <a href=\"grayscaling.html\">grayscaling</a>");
    index_html_content.push_str("           </li>");
    index_html_content.push_str("           <li>");
    index_html_content.push_str("               <a href=\"interpolation.html\">interpolation</a>");
    index_html_content.push_str("           </li>");
    index_html_content.push_str("       </ul>");
    index_html_content.push_str("   </body>\n");
    index_html_content.push_str("</html>\n");

    let mut index_html_file = File::create("output/index.html")?;
    index_html_file.write_all(index_html_content.as_bytes())?;

    let index_css_content = build_index_css();
    let mut index_css_file = File::create("output/index.css")?;
    index_css_file.write_all(index_css_content.as_bytes())?;

    let enum_string_comparison_html_content = build_enum_string_comparison_html();
    let mut enum_string_comparison_html_file = File::create("output/enum-string-comparison.html")?;
    enum_string_comparison_html_file.write_all(enum_string_comparison_html_content.as_bytes())?;

    let cmyk_examples_html_content = build_cmyk_examples_html();
    let mut cmyk_examples_file = File::create("output/cmyk-examples.html")?;
    cmyk_examples_file.write_all(cmyk_examples_html_content.as_bytes())?;

    let gray_examples_html_content = build_gray_examples_html();
    let mut gray_examples_file = File::create("output/gray-examples.html")?;
    gray_examples_file.write_all(gray_examples_html_content.as_bytes())?;

    let hsl_examples_html_content = build_hsl_examples_html();
    let mut hsl_examples_file = File::create("output/hsl-examples.html")?;
    hsl_examples_file.write_all(hsl_examples_html_content.as_bytes())?;

    let hsv_examples_html_content = build_hsv_examples_html();
    let mut hsv_examples_file = File::create("output/hsv-examples.html")?;
    hsv_examples_file.write_all(hsv_examples_html_content.as_bytes())?;

    let hwb_examples_html_content = build_hwb_examples_html();
    let mut hwb_examples_file = File::create("output/hwb-examples.html")?;
    hwb_examples_file.write_all(hwb_examples_html_content.as_bytes())?;

    let grayscaling_html_content = build_grayscaling_html();
    let mut grayscaling_file = File::create("output/grayscaling.html")?;
    grayscaling_file.write_all(grayscaling_html_content.as_bytes())?;

    let interpolation_html_content = build_interpolation_html();
    let mut interpolation_file = File::create("output/interpolation.html")?;
    interpolation_file.write_all(interpolation_html_content.as_bytes())?;

    Ok(())
}

fn build_index_css() -> String {
    let mut css_content = String::new();
    css_content.push_str(".color-box {\n");
    css_content.push_str("  display: inline-block;\n");
    css_content.push_str("  width: 100px;\n");
    css_content.push_str("  height: 20px;\n");
    css_content.push_str("}\n");
    css_content.push_str(".color-bar {\n");
    css_content.push_str("  display: inline-block;\n");
    css_content.push_str("  width: 1px;\n");
    css_content.push_str("  height: 20px;\n");
    css_content.push_str("}\n");
    css_content.push_str("table {\n");
    css_content.push_str("  border-collapse: collapse;\n");
    css_content.push_str("}\n");
    css_content.push_str("table.center {\n");
    css_content.push_str("  margin-left: auto;\n");
    css_content.push_str("  margin-right: auto;\n");
    css_content.push_str("}\n");
    css_content.push_str("table td, table th {\n");
    css_content.push_str("  border: 1px solid black;\n");
    css_content.push_str("}\n");
    css_content.push_str(".center-text {\n");
    css_content.push_str("  text-align: center;\n");
    css_content.push_str("}\n");

    return css_content;
}

fn build_enum_string_comparison_html() -> String {
    let mut html_content = String::new();
    html_content.push_str("<!DOCTYPE html>\n");
    html_content.push_str("<html>\n");
    html_content.push_str(" <head>\n");
    html_content.push_str("     <title>enum-string-comparison</title>\n");
    html_content.push_str("     <link rel=\"stylesheet\" href=\"index.css\">\n");
    html_content.push_str(" </head>\n");
    html_content.push_str(" <body>\n");
    html_content.push_str("     <a href=\"index.html\">&lt; back</a>");
    html_content.push_str("     <table class=\"center\">\n");
    html_content.push_str("         <thead>\n");
    html_content.push_str("             <tr>\n");
    html_content.push_str("                 <th>name</th>\n");
    html_content.push_str("                 <th>by css-property</th>\n");
    html_content.push_str("                 <th>by rust Color::new_string(\"&lt;name&gt;\")</th>\n");
    html_content.push_str("                 <th>by rust Color::new_enum(&lt;KnownColors::Name&gt;)</th>\n");
    html_content.push_str("             </tr>\n");
    html_content.push_str("         </thead>\n");
    html_content.push_str("         <tbody>\n");
    html_content.push_str(build_color_table_row("aliceblue", KnownColors::AliceBlue).as_str());
    html_content.push_str(build_color_table_row("antiquewhite", KnownColors::AntiqueWhite).as_str());
    html_content.push_str(build_color_table_row("aqua", KnownColors::Aqua).as_str());
    html_content.push_str(build_color_table_row("aquamarine", KnownColors::AquaMarine).as_str());
    html_content.push_str(build_color_table_row("azure", KnownColors::Azure).as_str());
    html_content.push_str(build_color_table_row("beige", KnownColors::Beige).as_str());
    html_content.push_str(build_color_table_row("bisque", KnownColors::Bisque).as_str());
    html_content.push_str(build_color_table_row("black", KnownColors::Black).as_str());
    html_content.push_str(build_color_table_row("blanchedalmond", KnownColors::BlanchedAlmond).as_str());
    html_content.push_str(build_color_table_row("blue", KnownColors::Blue).as_str());
    html_content.push_str(build_color_table_row("blueviolet", KnownColors::BlueViolet).as_str());
    html_content.push_str(build_color_table_row("brown", KnownColors::Brown).as_str());
    html_content.push_str(build_color_table_row("burlywood", KnownColors::BurlyWood).as_str());
    html_content.push_str(build_color_table_row("cadetblue", KnownColors::CadetBlue).as_str());
    html_content.push_str(build_color_table_row("chartreuse", KnownColors::Chartreuse).as_str());
    html_content.push_str(build_color_table_row("chocolate", KnownColors::Chocolate).as_str());
    html_content.push_str(build_color_table_row("coral", KnownColors::Coral).as_str());
    html_content.push_str(build_color_table_row("cornflowerblue", KnownColors::CornflowerBlue).as_str());
    html_content.push_str(build_color_table_row("cornsilk", KnownColors::Cornsilk).as_str());
    html_content.push_str(build_color_table_row("crimson", KnownColors::Crimson).as_str());
    html_content.push_str(build_color_table_row("cyan", KnownColors::Cyan).as_str());
    html_content.push_str(build_color_table_row("darkblue", KnownColors::DarkBlue).as_str());
    html_content.push_str(build_color_table_row("darkcyan", KnownColors::DarkCyan).as_str());
    html_content.push_str(build_color_table_row("darkgoldenrod", KnownColors::DarkGoldenrod).as_str());
    html_content.push_str(build_color_table_row("darkgray", KnownColors::DarkGray).as_str());
    html_content.push_str(build_color_table_row("darkgreen", KnownColors::DarkGreen).as_str());
    html_content.push_str(build_color_table_row("darkkhaki", KnownColors::DarkKhaki).as_str());
    html_content.push_str(build_color_table_row("darkmagenta", KnownColors::DarkMagenta).as_str());
    html_content.push_str(build_color_table_row("darkolivegreen", KnownColors::DarkOliveGreen).as_str());
    html_content.push_str(build_color_table_row("darkorange", KnownColors::DarkOrange).as_str());
    html_content.push_str(build_color_table_row("darkorchid", KnownColors::DarkOrchid).as_str());
    html_content.push_str(build_color_table_row("darkred", KnownColors::DarkRed).as_str());
    html_content.push_str(build_color_table_row("darksalmon", KnownColors::DarkSalmon).as_str());
    html_content.push_str(build_color_table_row("darkseagreen", KnownColors::DarkSeaGreen).as_str());
    html_content.push_str(build_color_table_row("darkslateblue", KnownColors::DarkSlateBlue).as_str());
    html_content.push_str(build_color_table_row("darkslategray", KnownColors::DarkSlateGray).as_str());
    html_content.push_str(build_color_table_row("darkturquoise", KnownColors::DarkTurquoise).as_str());
    html_content.push_str(build_color_table_row("darkviolet", KnownColors::DarkViolet).as_str());
    html_content.push_str(build_color_table_row("deeppink", KnownColors::DeepPink).as_str());
    html_content.push_str(build_color_table_row("deepskyblue", KnownColors::DeepSkyBlue).as_str());
    html_content.push_str(build_color_table_row("dimgray", KnownColors::DimGray).as_str());
    html_content.push_str(build_color_table_row("dodgerblue", KnownColors::DodgerBlue).as_str());
    html_content.push_str(build_color_table_row("firebrick", KnownColors::Firebrick).as_str());
    html_content.push_str(build_color_table_row("floralwhite", KnownColors::FloralWhite).as_str());
    html_content.push_str(build_color_table_row("forestgreen", KnownColors::ForestGreen).as_str());
    html_content.push_str(build_color_table_row("fuchsia", KnownColors::Fuchsia).as_str());
    html_content.push_str(build_color_table_row("gainsboro", KnownColors::Gainsboro).as_str());
    html_content.push_str(build_color_table_row("ghostwhite", KnownColors::GhostWhite).as_str());
    html_content.push_str(build_color_table_row("gold", KnownColors::Gold).as_str());
    html_content.push_str(build_color_table_row("goldenrod", KnownColors::Goldenrod).as_str());
    html_content.push_str(build_color_table_row("gray", KnownColors::Gray).as_str());
    html_content.push_str(build_color_table_row("green", KnownColors::Green).as_str());
    html_content.push_str(build_color_table_row("greenyellow", KnownColors::GreenYellow).as_str());
    html_content.push_str(build_color_table_row("honeydew", KnownColors::Honeydew).as_str());
    html_content.push_str(build_color_table_row("hotpink", KnownColors::HotPink).as_str());
    html_content.push_str(build_color_table_row("indianred", KnownColors::IndianRed).as_str());
    html_content.push_str(build_color_table_row("indigo", KnownColors::Indigo).as_str());
    html_content.push_str(build_color_table_row("ivory", KnownColors::Ivory).as_str());
    html_content.push_str(build_color_table_row("khaki", KnownColors::Khaki).as_str());
    html_content.push_str(build_color_table_row("lavender", KnownColors::Lavender).as_str());
    html_content.push_str(build_color_table_row("lavenderblush", KnownColors::LavenderBlush).as_str());
    html_content.push_str(build_color_table_row("lawngreen", KnownColors::LawnGreen).as_str());
    html_content.push_str(build_color_table_row("lemonchiffon", KnownColors::LemonChiffon).as_str());
    html_content.push_str(build_color_table_row("lightblue", KnownColors::LightBlue).as_str());
    html_content.push_str(build_color_table_row("lightcoral", KnownColors::LightCoral).as_str());
    html_content.push_str(build_color_table_row("lightcyan", KnownColors::LightCyan).as_str());
    html_content.push_str(build_color_table_row("lightgoldenrodyellow", KnownColors::LightGoldenrodYellow).as_str());
    html_content.push_str(build_color_table_row("lightgray", KnownColors::LightGray).as_str());
    html_content.push_str(build_color_table_row("lightgreen", KnownColors::LightGreen).as_str());
    html_content.push_str(build_color_table_row("lightpink", KnownColors::LightPink).as_str());
    html_content.push_str(build_color_table_row("lightsalmon", KnownColors::LightSalmon).as_str());
    html_content.push_str(build_color_table_row("lightseagreen", KnownColors::LightSeaGreen).as_str());
    html_content.push_str(build_color_table_row("lightskyblue", KnownColors::LightSkyBlue).as_str());
    html_content.push_str(build_color_table_row("lightslategray", KnownColors::LightSlateGray).as_str());
    html_content.push_str(build_color_table_row("lightsteelblue", KnownColors::LightSteelBlue).as_str());
    html_content.push_str(build_color_table_row("lightyellow", KnownColors::LightYellow).as_str());
    html_content.push_str(build_color_table_row("lime", KnownColors::Lime).as_str());
    html_content.push_str(build_color_table_row("limegreen", KnownColors::LimeGreen).as_str());
    html_content.push_str(build_color_table_row("linen", KnownColors::Linen).as_str());
    html_content.push_str(build_color_table_row("magenta", KnownColors::Magenta).as_str());
    html_content.push_str(build_color_table_row("maroon", KnownColors::Maroon).as_str());
    html_content.push_str(build_color_table_row("mediumaquamarine", KnownColors::MediumAquaMarine).as_str());
    html_content.push_str(build_color_table_row("mediumblue", KnownColors::MediumBlue).as_str());
    html_content.push_str(build_color_table_row("mediumorchid", KnownColors::MediumOrchid).as_str());
    html_content.push_str(build_color_table_row("mediumpurple", KnownColors::MediumPurple).as_str());
    html_content.push_str(build_color_table_row("mediumseagreen", KnownColors::MediumSeaGreen).as_str());
    html_content.push_str(build_color_table_row("mediumslateblue", KnownColors::MediumSlateBlue).as_str());
    html_content.push_str(build_color_table_row("mediumspringgreen", KnownColors::MediumSpringGreen).as_str());
    html_content.push_str(build_color_table_row("mediumturquoise", KnownColors::MediumTurquoise).as_str());
    html_content.push_str(build_color_table_row("mediumvioletred", KnownColors::MediumVioletRed).as_str());
    html_content.push_str(build_color_table_row("midnightblue", KnownColors::MidnightBlue).as_str());
    html_content.push_str(build_color_table_row("mintcream", KnownColors::MintCream).as_str());
    html_content.push_str(build_color_table_row("mistyrose", KnownColors::MistyRose).as_str());
    html_content.push_str(build_color_table_row("moccasin", KnownColors::Moccasin).as_str());
    html_content.push_str(build_color_table_row("navajowhite", KnownColors::NavajoWhite).as_str());
    html_content.push_str(build_color_table_row("navy", KnownColors::Navy).as_str());
    html_content.push_str(build_color_table_row("oldlace", KnownColors::OldLace).as_str());
    html_content.push_str(build_color_table_row("olive", KnownColors::Olive).as_str());
    html_content.push_str(build_color_table_row("olivedrab", KnownColors::OliveDrab).as_str());
    html_content.push_str(build_color_table_row("orange", KnownColors::Orange).as_str());
    html_content.push_str(build_color_table_row("orangered", KnownColors::OrangeRed).as_str());
    html_content.push_str(build_color_table_row("orchid", KnownColors::Orchid).as_str());
    html_content.push_str(build_color_table_row("palegoldenrod", KnownColors::PaleGoldenrod).as_str());
    html_content.push_str(build_color_table_row("palegreen", KnownColors::PaleGreen).as_str());
    html_content.push_str(build_color_table_row("paleturquoise", KnownColors::PaleTurquoise).as_str());
    html_content.push_str(build_color_table_row("palevioletred", KnownColors::PaleVioletRed).as_str());
    html_content.push_str(build_color_table_row("papayawhip", KnownColors::PapayaWhip).as_str());
    html_content.push_str(build_color_table_row("peachpuff", KnownColors::PeachPuff).as_str());
    html_content.push_str(build_color_table_row("peru", KnownColors::Peru).as_str());
    html_content.push_str(build_color_table_row("pink", KnownColors::Pink).as_str());
    html_content.push_str(build_color_table_row("plum", KnownColors::Plum).as_str());
    html_content.push_str(build_color_table_row("powderblue", KnownColors::PowderBlue).as_str());
    html_content.push_str(build_color_table_row("purple", KnownColors::Purple).as_str());
    html_content.push_str(build_color_table_row("red", KnownColors::Red).as_str());
    html_content.push_str(build_color_table_row("rosybrown", KnownColors::RosyBrown).as_str());
    html_content.push_str(build_color_table_row("royalblue", KnownColors::RoyalBlue).as_str());
    html_content.push_str(build_color_table_row("saddlebrown", KnownColors::SaddleBrown).as_str());
    html_content.push_str(build_color_table_row("salmon", KnownColors::Salmon).as_str());
    html_content.push_str(build_color_table_row("sandybrown", KnownColors::SandyBrown).as_str());
    html_content.push_str(build_color_table_row("seagreen", KnownColors::SeaGreen).as_str());
    html_content.push_str(build_color_table_row("seashell", KnownColors::SeaShell).as_str());
    html_content.push_str(build_color_table_row("sienna", KnownColors::Sienna).as_str());
    html_content.push_str(build_color_table_row("silver", KnownColors::Silver).as_str());
    html_content.push_str(build_color_table_row("skyblue", KnownColors::SkyBlue).as_str());
    html_content.push_str(build_color_table_row("slateblue", KnownColors::SlateBlue).as_str());
    html_content.push_str(build_color_table_row("slategray", KnownColors::SlateGray).as_str());
    html_content.push_str(build_color_table_row("snow", KnownColors::Snow).as_str());
    html_content.push_str(build_color_table_row("springgreen", KnownColors::SpringGreen).as_str());
    html_content.push_str(build_color_table_row("steelblue", KnownColors::SteelBlue).as_str());
    html_content.push_str(build_color_table_row("tan", KnownColors::Tan).as_str());
    html_content.push_str(build_color_table_row("teal", KnownColors::Teal).as_str());
    html_content.push_str(build_color_table_row("thistle", KnownColors::Thistle).as_str());
    html_content.push_str(build_color_table_row("tomato", KnownColors::Tomato).as_str());
    html_content.push_str(build_color_table_row("transparent", KnownColors::Transparent).as_str());
    html_content.push_str(build_color_table_row("turquoise", KnownColors::Turquoise).as_str());
    html_content.push_str(build_color_table_row("violet", KnownColors::Violet).as_str());
    html_content.push_str(build_color_table_row("wheat", KnownColors::Wheat).as_str());
    html_content.push_str(build_color_table_row("white", KnownColors::White).as_str());
    html_content.push_str(build_color_table_row("whitesmoke", KnownColors::WhiteSmoke).as_str());
    html_content.push_str(build_color_table_row("yellow", KnownColors::Yellow).as_str());
    html_content.push_str(build_color_table_row("yellowgreen", KnownColors::YellowGreen).as_str());
    html_content.push_str("         </tbody>\n");
    html_content.push_str("     </table>\n");
    html_content.push_str(" </body>\n");
    html_content.push_str("</html>\n");

    return html_content;
}

fn build_cmyk_examples_html() -> String {
    let mut html_content = String::new();
    html_content.push_str("<!DOCTYPE html>\n");
    html_content.push_str("<html>\n");
    html_content.push_str(" <head>\n");
    html_content.push_str("     <title>cmyk-examples</title>\n");
    html_content.push_str("     <link rel=\"stylesheet\" href=\"index.css\">\n");
    html_content.push_str(" </head>\n");
    html_content.push_str(" <body>\n");
    html_content.push_str("     <a href=\"index.html\">&lt; back</a>");
    html_content.push_str("     <table class=\"center\">\n");
    html_content.push_str("         <thead>\n");
    html_content.push_str("             <tr>\n");
    html_content.push_str("                 <th>values</th>\n");
    html_content.push_str("                 <th>by rust Color::new_cmyk(cyan, magenta, yellow, key)</th>\n");
    html_content.push_str("                 <th>by rust Color::new_string(\"cmyk(&lt;c&gt;%, &lt;m&gt;%, &lt;y&gt;%, &lt;k&gt;%)\")</th>\n");
    html_content.push_str("             </tr>\n");
    html_content.push_str("         </thead>\n");
    html_content.push_str("         <tbody>\n");
    html_content.push_str(build_cmyk_color_table_row(0.0, 0.0, 0.0, 0.0).as_str());
    html_content.push_str(build_cmyk_color_table_row(0.0, 0.0, 0.0, 0.25).as_str());
    html_content.push_str(build_cmyk_color_table_row(0.0, 0.0, 0.0, 0.5).as_str());
    html_content.push_str(build_cmyk_color_table_row(0.0, 0.0, 0.0, 0.75).as_str());
    html_content.push_str(build_cmyk_color_table_row(0.0, 0.0, 0.0, 1.0).as_str());
    html_content.push_str(build_cmyk_color_table_row(1.0, 0.0, 0.0, 0.0).as_str());
    html_content.push_str(build_cmyk_color_table_row(0.0, 1.0, 0.0, 0.0).as_str());
    html_content.push_str(build_cmyk_color_table_row(0.0, 0.0, 1.0, 0.0).as_str());
    html_content.push_str(build_cmyk_color_table_row(0.0, 1.0, 1.0, 0.0).as_str());
    html_content.push_str(build_cmyk_color_table_row(1.0, 0.0, 1.0, 0.0).as_str());
    html_content.push_str(build_cmyk_color_table_row(1.0, 1.0, 0.0, 0.0).as_str());
    html_content.push_str(build_cmyk_color_table_row(1.0, 0.0, 0.0, 0.5).as_str());
    html_content.push_str(build_cmyk_color_table_row(0.0, 1.0, 0.0, 0.5).as_str());
    html_content.push_str(build_cmyk_color_table_row(0.0, 0.0, 1.0, 0.5).as_str());
    html_content.push_str(build_cmyk_color_table_row(0.0, 1.0, 1.0, 0.5).as_str());
    html_content.push_str(build_cmyk_color_table_row(1.0, 0.0, 1.0, 0.5).as_str());
    html_content.push_str(build_cmyk_color_table_row(1.0, 1.0, 0.0, 0.5).as_str());
    html_content.push_str("         </tbody>\n");
    html_content.push_str("     </table>\n");
    html_content.push_str(" </body>\n");
    html_content.push_str("</html>\n");

    return html_content;
}

fn build_cmyk_color_table_row(cyan: f64, magenta: f64, yellow: f64, key: f64) -> String {
    let mut cmyk_string = String::from("cmyk(");
    cmyk_string.push_str(format!("{}%, {}%, {}%, {}%", cyan * 100.0, magenta * 100.0, yellow * 100.0, key * 100.0).as_str());
    cmyk_string.push_str(")");
    let cmyk_by_string = Color::new_string(cmyk_string.as_str()).unwrap();

    let cmyk = Color::new_cmyk(cyan, magenta, yellow, key);
    let cmyk_string = cmyk.to_cmyk_string();
    let cmyk_str = cmyk_string.as_str();
    let hex_string = cmyk.to_hex_string();
    let hex_str = hex_string.as_str();

    let mut row_content = String::new();
    row_content.push_str("              <tr>\n");
    row_content.push_str("                  <td>");
    row_content.push_str(cmyk_str);
    row_content.push_str("</td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(hex_str);
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(cmyk_by_string.to_hex_string().as_str());
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("              </tr>\n");

    return row_content;
}

fn build_color_table_row(color_name: &str, known_color: KnownColors) -> String {
    let mut row_content = String::new();
    row_content.push_str("             <tr>\n");
    row_content.push_str("                 <td class=\"center-text\">");
    row_content.push_str(color_name);
    row_content.push_str("</td>\n");
    row_content.push_str("                 <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(color_name);
    row_content.push_str(";\"></div></td>\n");
    let color_by_string = Color::new_string(color_name).unwrap();
    let color_by_enum = Color::new_enum(known_color);
    row_content.push_str("                 <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(color_by_string.to_hex_string().as_str());
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("                 <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(color_by_enum.to_hex_string().as_str());
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("             </tr>\n");

    return row_content;
}

fn build_gray_color_table_row(gray: u8) -> String {
    let gray_string = format!("gray({})", gray);
    let gray_by_string = Color::new_string(gray_string.as_str()).unwrap();

    let gray = Color::new_gray(gray);
    let gray_string = gray.to_gray_string();
    let gray_str = gray_string.as_str();
    let hex_string = gray.to_hex_string();
    let hex_str = hex_string.as_str();

    let mut row_content = String::new();
    row_content.push_str("              <tr>\n");
    row_content.push_str("                  <td>");
    row_content.push_str(gray_str);
    row_content.push_str("</td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(hex_str);
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(gray_by_string.to_hex_string().as_str());
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("              </tr>\n");

    return row_content;
}

fn build_hsl_color_table_row(h: f64, s: f64, l: f64) -> String {
    let hsl_string = format!("hsl({}, {}%, {}%)", h, s * 100.0, l * 100.0);
    let hsl_by_string = Color::new_string(hsl_string.as_str()).unwrap();

    let hsl = Color::new_hsl(h, s, l);
    let hsl_string = hsl.to_hsl_string();
    let hsl_str = hsl_string.as_str();
    let hex_string = hsl.to_hex_string();
    let hex_str = hex_string.as_str();

    let mut row_content = String::new();
    row_content.push_str("              <tr>\n");
    row_content.push_str("                  <td>");
    row_content.push_str(hsl_str);
    row_content.push_str("</td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(hex_str);
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(hsl_by_string.to_hex_string().as_str());
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("              </tr>\n");

    return row_content;
}

fn build_hsv_color_table_row(h: f64, s: f64, v: f64) -> String {
    let hsv_string = format!("hsv({}, {}%, {}%)", h, s * 100.0, v * 100.0);
    let hsv_by_string = Color::new_string(hsv_string.as_str()).unwrap();

    let hsv = Color::new_hsv(h, s, v);
    let hsv_string = hsv.to_hsv_string();
    let hsv_str = hsv_string.as_str();
    let hex_string = hsv.to_hex_string();
    let hex_str = hex_string.as_str();

    let mut row_content = String::new();
    row_content.push_str("              <tr>\n");
    row_content.push_str("                  <td>");
    row_content.push_str(hsv_str);
    row_content.push_str("</td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(hex_str);
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(hsv_by_string.to_hex_string().as_str());
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("              </tr>\n");

    return row_content;
}

fn build_hwb_color_table_row(h: f64, w: f64, b: f64) -> String {
    let hwb_string = format!("hwb({}, {}%, {}%)", h, w * 100.0, b * 100.0);
    let hwb_by_string = Color::new_string(hwb_string.as_str()).unwrap();

    let hwb = Color::new_hwb(h, w, b);
    let hwb_string = hwb.to_hwb_string();
    let hwb_str = hwb_string.as_str();
    let hex_string = hwb.to_hex_string();
    let hex_str = hex_string.as_str();

    let mut row_content = String::new();
    row_content.push_str("              <tr>\n");
    row_content.push_str("                  <td>");
    row_content.push_str(hwb_str);
    row_content.push_str("</td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(hex_str);
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(hwb_by_string.to_hex_string().as_str());
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("              </tr>\n");

    return row_content;
}

fn build_grayscaling_table_row(h: f64, w: f64, b: f64) -> String {
    let hwb = Color::new_hwb(h, w, b);
    let hex_string = hwb.to_hex_string();
    let hex_str = hex_string.as_str();

    let grayscaled = hwb.grayscale();
    let grayscaled_string = grayscaled.to_hex_string();
    let grayscaled_str = grayscaled_string.as_str();

    let grayscaled_hdtv = hwb.grayscale_hdtv();
    let grayscaled_hdtv_string = grayscaled_hdtv.to_hex_string();
    let grayscaled_hdtv_str = grayscaled_hdtv_string.as_str();

    let grayscaled_hdr = hwb.grayscale_hdr();
    let grayscaled_hdr_string = grayscaled_hdr.to_hex_string();
    let grayscaled_hdr_str = grayscaled_hdr_string.as_str();

    let mut row_content = String::new();
    row_content.push_str("              <tr>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(hex_str);
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(grayscaled_str);
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(grayscaled_hdtv_str);
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("                  <td class=\"center-text\"><div class=\"color-box\" style=\"background-color: ");
    row_content.push_str(grayscaled_hdr_str);
    row_content.push_str(";\"></div></td>\n");
    row_content.push_str("              </tr>\n");

    return row_content;
}

fn build_interpolation_table_row(start_color_str: &str, end_color_str: &str) -> String {
    let start_color = Color::new_string(start_color_str).unwrap();
    let end_color = Color::new_string(end_color_str).unwrap();
    let color_bar_width = 256;

    let build_row = |method: &str| -> String {
        let mut row_content = String::new();
        row_content.push_str("              <tr>\n");
        row_content.push_str("                  <td class=\"center-text\">");
        row_content.push_str(method);
        row_content.push_str("</td>\n");
        row_content.push_str("                  <td class=\"center-text\">");
        row_content.push_str(start_color_str);
        row_content.push_str("</td>\n");
        row_content.push_str("                  <td class=\"center-text\">");
        match method {
            "css linear-gradient" => {
                row_content.push_str("<div class=\"color-box\" style=\"background-image: linear-gradient(to right, ");
                row_content.push_str(start_color_str);
                row_content.push_str(", ");
                row_content.push_str(end_color_str);
                row_content.push_str("); width: ");
                row_content.push_str(color_bar_width.to_string().as_str());
                row_content.push_str("px;\"></div>\n");
            },
            "rust color.interpolate(color, interpolation)" => {
                row_content.push_str("<div>");
                for i in 0..color_bar_width {
                    let interpolation = i as f64 / color_bar_width as f64;
                    let interpolated_color = start_color.interpolate(end_color, interpolation);
                    row_content.push_str("<div class=\"color-bar\" style=\"background-color: ");
                    row_content.push_str(interpolated_color.to_hex_string().as_str());
                    row_content.push_str(";\"></div>");
                }
                row_content.push_str("</div>");
            },
            "rust color.interpolate_hsv(color, interpolation)" => {
                row_content.push_str("<div>");
                for i in 0..color_bar_width {
                    let interpolation = i as f64 / color_bar_width as f64;
                    let interpolated_color = start_color.interpolate_hsv(end_color, interpolation);
                    row_content.push_str("<div class=\"color-bar\" style=\"background-color: ");
                    row_content.push_str(interpolated_color.to_hex_string().as_str());
                    row_content.push_str(";\"></div>");
                }
                row_content.push_str("</div>");
            },
            "rust color.interpolate_hsl(color, interpolation)" => {
                row_content.push_str("<div>");
                for i in 0..color_bar_width {
                    let interpolation = i as f64 / color_bar_width as f64;
                    let interpolated_color = start_color.interpolate_hsl(end_color, interpolation);
                    row_content.push_str("<div class=\"color-bar\" style=\"background-color: ");
                    row_content.push_str(interpolated_color.to_hex_string().as_str());
                    row_content.push_str(";\"></div>");
                }
                row_content.push_str("</div>");
            },
            "rust color.interpolate_hwb(color, interpolation)" => {
                row_content.push_str("<div>");
                for i in 0..color_bar_width {
                    let interpolation = i as f64 / color_bar_width as f64;
                    let interpolated_color = start_color.interpolate_hwb(end_color, interpolation);
                    row_content.push_str("<div class=\"color-bar\" style=\"background-color: ");
                    row_content.push_str(interpolated_color.to_hex_string().as_str());
                    row_content.push_str(";\"></div>");
                }
                row_content.push_str("</div>");
            },
            "rust color.interpolate_lch(color, interpolation)" => {
                row_content.push_str("<div>");
                for i in 0..color_bar_width {
                    let interpolation = i as f64 / color_bar_width as f64;
                    let interpolated_color = start_color.interpolate_lch(end_color, interpolation);
                    row_content.push_str("<div class=\"color-bar\" style=\"background-color: ");
                    row_content.push_str(interpolated_color.to_hex_string().as_str());
                    row_content.push_str(";\"></div>");
                }
                row_content.push_str("</div>");
            },
            _ => {}
        }
        row_content.push_str("</td>");
        row_content.push_str("                  <td class=\"center-text\">");
        row_content.push_str(end_color_str);
        row_content.push_str("</td>\n");
        row_content.push_str("              </tr>\n");

        row_content
    };

    let mut row_content = String::new();
    let row1 = build_row("css linear-gradient");
    let row2 = build_row("rust color.interpolate(color, interpolation)");
    let row3 = build_row("rust color.interpolate_hsv(color, interpolation)");
    let row4 = build_row("rust color.interpolate_hsl(color, interpolation)");
    let row5 = build_row("rust color.interpolate_hwb(color, interpolation)");
    let row6 = build_row("rust color.interpolate_lch(color, interpolation)");
    row_content.push_str(row1.as_str());
    row_content.push_str(row2.as_str());
    row_content.push_str(row3.as_str());
    row_content.push_str(row4.as_str());
    row_content.push_str(row5.as_str());
    row_content.push_str(row6.as_str());

    row_content
}

fn build_gray_examples_html() -> String {
    let mut html_content = String::new();
    html_content.push_str("<!DOCTYPE html>\n");
    html_content.push_str("<html>\n");
    html_content.push_str(" <head>\n");
    html_content.push_str("     <title>gray-examples</title>\n");
    html_content.push_str("     <link rel=\"stylesheet\" href=\"index.css\">\n");
    html_content.push_str(" </head>\n");
    html_content.push_str(" <body>\n");
    html_content.push_str("     <a href=\"index.html\">&lt; back</a>");
    html_content.push_str("     <table class=\"center\">\n");
    html_content.push_str("         <thead>\n");
    html_content.push_str("             <tr>\n");
    html_content.push_str("                 <th>value</th>\n");
    html_content.push_str("                 <th>by rust Color::new_gray(gray)</th>\n");
    html_content.push_str("                 <th>by rust Color::new_string(\"gray(&lt;gray&gt;)\")</th>\n");
    html_content.push_str("             </tr>\n");
    html_content.push_str("         </thead>\n");
    html_content.push_str("         <tbody>\n");
    html_content.push_str(build_gray_color_table_row(0).as_str());
    html_content.push_str(build_gray_color_table_row(32).as_str());
    html_content.push_str(build_gray_color_table_row(64).as_str());
    html_content.push_str(build_gray_color_table_row(92).as_str());
    html_content.push_str(build_gray_color_table_row(128).as_str());
    html_content.push_str(build_gray_color_table_row(160).as_str());
    html_content.push_str(build_gray_color_table_row(192).as_str());
    html_content.push_str(build_gray_color_table_row(224).as_str());
    html_content.push_str(build_gray_color_table_row(255).as_str());
    html_content.push_str("         </tbody>\n");
    html_content.push_str("     </table>\n");
    html_content.push_str(" </body>\n");
    html_content.push_str("</html>\n");

    return html_content;
}

fn build_hsl_examples_html() -> String {
    let mut html_content = String::new();
    html_content.push_str("<!DOCTYPE html>\n");
    html_content.push_str("<html>\n");
    html_content.push_str(" <head>\n");
    html_content.push_str("     <title>hsl-examples</title>\n");
    html_content.push_str("     <link rel=\"stylesheet\" href=\"index.css\">\n");
    html_content.push_str(" </head>\n");
    html_content.push_str(" <body>\n");
    html_content.push_str("     <a href=\"index.html\">&lt; back</a>");
    html_content.push_str("     <table class=\"center\">\n");
    html_content.push_str("         <thead>\n");
    html_content.push_str("             <tr>\n");
    html_content.push_str("                 <th>value</th>\n");
    html_content.push_str("                 <th>by rust Color::new_hsl(h, s, l)</th>\n");
    html_content.push_str("                 <th>by rust Color::new_string(\"hsl(&lt;h&gt;, &lt;s&gt;, &lt;l&gt;)\")</th>\n");
    html_content.push_str("             </tr>\n");
    html_content.push_str("         </thead>\n");
    html_content.push_str("         <tbody>\n");
    for h in 0..36 {
        html_content.push_str(build_hsl_color_table_row(h as f64 * 10.0, 1.0, 0.5).as_str());
    }
    for h in 0..6 {
        html_content.push_str(build_hsl_color_table_row(h as f64 * 60.0, 0.5, 0.5).as_str());
    }
    html_content.push_str(build_hsl_color_table_row(0.0, 0.0, 0.5).as_str());
    html_content.push_str(build_hsl_color_table_row(0.0, 1.0, 0.0).as_str());
    for h in 0..6 {
        html_content.push_str(build_hsl_color_table_row(h as f64 * 60.0, 1.0, 0.25).as_str());
    }
    for h in 0..6 {
        html_content.push_str(build_hsl_color_table_row(h as f64 * 60.0, 1.0, 0.5).as_str());
    }
    for h in 0..6 {
        html_content.push_str(build_hsl_color_table_row(h as f64 * 60.0, 1.0, 0.75).as_str());
    }
    html_content.push_str(build_hsl_color_table_row(0.0, 1.0, 1.0).as_str());
    html_content.push_str("         </tbody>\n");
    html_content.push_str("     </table>\n");
    html_content.push_str(" </body>\n");
    html_content.push_str("</html>\n");

    return html_content;
}

fn build_hsv_examples_html() -> String {
    let mut html_content = String::new();
    html_content.push_str("<!DOCTYPE html>\n");
    html_content.push_str("<html>\n");
    html_content.push_str(" <head>\n");
    html_content.push_str("     <title>hsv-examples</title>\n");
    html_content.push_str("     <link rel=\"stylesheet\" href=\"index.css\">\n");
    html_content.push_str(" </head>\n");
    html_content.push_str(" <body>\n");
    html_content.push_str("     <a href=\"index.html\">&lt; back</a>");
    html_content.push_str("     <table class=\"center\">\n");
    html_content.push_str("         <thead>\n");
    html_content.push_str("             <tr>\n");
    html_content.push_str("                 <th>value</th>\n");
    html_content.push_str("                 <th>by rust Color::new_hsv(h, s, v)</th>\n");
    html_content.push_str("                 <th>by rust Color::new_string(\"hsl(&lt;h&gt;, &lt;s&gt;, &lt;v&gt;)\")</th>\n");
    html_content.push_str("             </tr>\n");
    html_content.push_str("         </thead>\n");
    html_content.push_str("         <tbody>\n");
    for h in 0..36 {
        html_content.push_str(build_hsv_color_table_row(h as f64 * 10.0, 1.0, 1.0).as_str());
    }
    for h in 0..6 {
        html_content.push_str(build_hsv_color_table_row(h as f64 * 60.0, 0.5, 1.0).as_str());
    }
    html_content.push_str(build_hsv_color_table_row(0.0, 0.0, 0.5).as_str());
    html_content.push_str(build_hsv_color_table_row(0.0, 1.0, 0.0).as_str());
    for h in 0..6 {
        html_content.push_str(build_hsv_color_table_row(h as f64 * 60.0, 1.0, 0.25).as_str());
    }
    for h in 0..6 {
        html_content.push_str(build_hsv_color_table_row(h as f64 * 60.0, 1.0, 0.5).as_str());
    }
    for h in 0..6 {
        html_content.push_str(build_hsv_color_table_row(h as f64 * 60.0, 1.0, 0.75).as_str());
    }
    html_content.push_str(build_hsv_color_table_row(0.0, 1.0, 1.0).as_str());
    html_content.push_str("         </tbody>\n");
    html_content.push_str("     </table>\n");
    html_content.push_str(" </body>\n");
    html_content.push_str("</html>\n");

    return html_content;
}

fn build_hwb_examples_html() -> String {
    let mut html_content = String::new();
    html_content.push_str("<!DOCTYPE html>\n");
    html_content.push_str("<html>\n");
    html_content.push_str(" <head>\n");
    html_content.push_str("     <title>hwb-examples</title>\n");
    html_content.push_str("     <link rel=\"stylesheet\" href=\"index.css\">\n");
    html_content.push_str(" </head>\n");
    html_content.push_str(" <body>\n");
    html_content.push_str("     <a href=\"index.html\">&lt; back</a>");
    html_content.push_str("     <table class=\"center\">\n");
    html_content.push_str("         <thead>\n");
    html_content.push_str("             <tr>\n");
    html_content.push_str("                 <th>value</th>\n");
    html_content.push_str("                 <th>by rust Color::new_hwb(h, w, b)</th>\n");
    html_content.push_str("                 <th>by rust Color::new_string(\"hwb(&lt;h&gt;, &lt;w&gt;, &lt;b&gt;)\")</th>\n");
    html_content.push_str("             </tr>\n");
    html_content.push_str("         </thead>\n");
    html_content.push_str("         <tbody>\n");
    for h in 0..36 {
        html_content.push_str(build_hwb_color_table_row(h as f64 * 10.0, 0.0, 0.0).as_str());
    }
    html_content.push_str(build_hwb_color_table_row(0.0, 0.5, 0.5).as_str());
    html_content.push_str(build_hwb_color_table_row(0.0, 0.0, 1.0).as_str());
    html_content.push_str(build_hwb_color_table_row(0.0, 1.0, 0.0).as_str());
    for h in 0..6 {
        html_content.push_str(build_hwb_color_table_row(h as f64 * 60.0, 0.5, 0.0).as_str());
    }
    for h in 0..6 {
        html_content.push_str(build_hwb_color_table_row(h as f64 * 60.0, 0.0, 0.5).as_str());
    }
    html_content.push_str("         </tbody>\n");
    html_content.push_str("     </table>\n");
    html_content.push_str(" </body>\n");
    html_content.push_str("</html>\n");

    return html_content;
}

fn build_grayscaling_html() -> String {
    let mut html_content = String::new();
    html_content.push_str("<!DOCTYPE html>\n");
    html_content.push_str("<html>\n");
    html_content.push_str(" <head>\n");
    html_content.push_str("     <title>grayscaling</title>\n");
    html_content.push_str("     <link rel=\"stylesheet\" href=\"index.css\">\n");
    html_content.push_str(" </head>\n");
    html_content.push_str(" <body>\n");
    html_content.push_str("     <a href=\"index.html\">&lt; back</a>");
    html_content.push_str("     <table class=\"center\">\n");
    html_content.push_str("         <thead>\n");
    html_content.push_str("             <tr>\n");
    html_content.push_str("                 <th>color</th>\n");
    html_content.push_str("                 <th>by rust color.grayscale()</th>\n");
    html_content.push_str("                 <th>by rust color.grayscale_hdtv()</th>\n");
    html_content.push_str("                 <th>by rust color.grayscale_hdr()</th>\n");
    html_content.push_str("             </tr>\n");
    html_content.push_str("         </thead>\n");
    html_content.push_str("         <tbody>\n");
    for h in 0..36 {
        html_content.push_str(build_grayscaling_table_row(h as f64 * 10.0, 0.0, 0.0).as_str());
    }
    html_content.push_str("         </tbody>\n");
    html_content.push_str("     </table>\n");
    html_content.push_str(" </body>\n");
    html_content.push_str("</html>\n");

    return html_content;
}

fn build_interpolation_html() -> String {
    let mut html_content = String::new();
    html_content.push_str("<!DOCTYPE html>\n");
    html_content.push_str("<html>\n");
    html_content.push_str(" <head>\n");
    html_content.push_str("     <title>interpolation</title>\n");
    html_content.push_str("     <link rel=\"stylesheet\" href=\"index.css\">\n");
    html_content.push_str(" </head>\n");
    html_content.push_str(" <body>\n");
    html_content.push_str("     <a href=\"index.html\">&lt; back</a>");
    html_content.push_str("     <table class=\"center\">\n");
    html_content.push_str("         <thead>\n");
    html_content.push_str("             <tr>\n");
    html_content.push_str("                 <th>method</th>\n");
    html_content.push_str("                 <th>start-color</th>\n");
    html_content.push_str("                 <th>interpolation</th>\n");
    html_content.push_str("                 <th>end-color</th>\n");
    html_content.push_str("             </tr>\n");
    html_content.push_str("         </thead>\n");
    html_content.push_str("         <tbody>\n");
    html_content.push_str(build_interpolation_table_row("white", "black").as_str());
    html_content.push_str(build_interpolation_table_row("red", "green").as_str());
    html_content.push_str(build_interpolation_table_row("red", "blue").as_str());
    html_content.push_str(build_interpolation_table_row("red", "cyan").as_str());
    html_content.push_str(build_interpolation_table_row("red", "magenta").as_str());
    html_content.push_str(build_interpolation_table_row("red", "yellow").as_str());
    html_content.push_str(build_interpolation_table_row("green", "blue").as_str());
    html_content.push_str(build_interpolation_table_row("green", "cyan").as_str());
    html_content.push_str(build_interpolation_table_row("green", "magenta").as_str());
    html_content.push_str(build_interpolation_table_row("green", "yellow").as_str());
    html_content.push_str(build_interpolation_table_row("blue", "cyan").as_str());
    html_content.push_str(build_interpolation_table_row("blue", "magenta").as_str());
    html_content.push_str(build_interpolation_table_row("blue", "yellow").as_str());
    html_content.push_str(build_interpolation_table_row("cyan", "magenta").as_str());
    html_content.push_str(build_interpolation_table_row("cyan", "yellow").as_str());
    html_content.push_str(build_interpolation_table_row("magenta", "yellow").as_str());
    html_content.push_str("         </tbody>\n");
    html_content.push_str("     </table>\n");
    html_content.push_str(" </body>\n");
    html_content.push_str("</html>\n");

    return html_content;
}
