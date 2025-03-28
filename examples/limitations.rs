use chromaterm::ColorSupport;
use chromaterm::prelude::*;

fn main() {
    let red_green = ".red().green()".red().green();
    println!(
        "{} will be red",
        red_green.display_exact(ColorSupport::True)
    );
}
