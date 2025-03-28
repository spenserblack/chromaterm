use chromaterm::{ColorSupport, Colorize, DisplayWithExact, DisplayWithFallback, Stylize};

fn main() {
    const SUPPORT: ColorSupport = ColorSupport::True;
    let text = "World".white().on_blue();
    println!("Hello, {}!", text.display_exact(SUPPORT));

    println!("Here's a gradient using true color:");
    (0..=255).step_by(8).for_each(|c| {
        let r = 255;
        let g = c / 2;
        let b = 255 - c;
        let text = " ".on_rgb(r, g, b);
        print!("{}", text.display_exact(SUPPORT));
    });
    println!();
    println!("Here's the gradient again if we fall back to lower support levels:");
    (0..=255).step_by(8).for_each(|c| {
        let r = 255;
        let g = c / 2;
        let b = 255 - c;
        let text = " ".on_rgb(r, g, b);
        print!("{}", text.display_fallback(ColorSupport::Simple));
    });
    println!();

    println!("If we have *any* color support, then we have style support");
    [
        ("true color", ColorSupport::True),
        ("simple colors", ColorSupport::Simple),
        ("no color support", ColorSupport::None),
    ]
    .into_iter()
    .for_each(|(text, support)| {
        println!(
            "Style with {}",
            text.italic()
                .black()
                .underline()
                .on_white()
                .display_exact(support)
        )
    });
}
