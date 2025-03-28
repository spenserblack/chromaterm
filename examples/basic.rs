// Importing * from prelude magically adds the coloring methods to strings.
use chromaterm::Colors;
use chromaterm::prelude::*;
use rand::prelude::*;

fn main() {
    // Setup the configuration for how you want coloring to behave.
    chromaterm::config::use_default_color_support();
    chromaterm::config::convert_to_supported(true);

    // Note that this example can behave differently on different devices or terminals.
    println!(
        "Hello, {}",
        "simple colors".black().on_bright_white().italic()
    );
    println!(
        "Hello, {}",
        "eight-bit lookup table colors"
            .eight_bit(232)
            .on_eight_bit(231)
            .italic()
    );
    println!(
        "Hello, {}",
        "true colors"
            .rgb(0xFF, 0x44, 0xAA)
            .on_rgb(0x33, 0x33, 0x33)
            .italic()
    );
    println!(
        "It works on all types that implement `Deref<Target=str>`, like {}",
        String::from("String")
            .underline()
            .bright_yellow()
            .on_black()
    );

    println!(
        "----------------------------------------------------------------------------------------"
    );

    println!("Color might not be known at compile-time? No worries! Use one of the enums");
    println!("that act as unions of colors.");

    enum LogLevel {
        Info,
        Warn,
        Error,
    }
    // Randomly picking a log level.
    let mut rng = rand::rng();
    let log_level = [LogLevel::Info, LogLevel::Warn, LogLevel::Error]
        .choose(&mut rng)
        .unwrap();

    let (status, color) = match log_level {
        LogLevel::Info => ("INFO", Colors::new_bright_green()),
        LogLevel::Warn => ("WARN", Colors::new_yellow()),
        LogLevel::Error => ("ERROR", Colors::new_bright_red()),
    };

    println!("[{}]: It works!", status.color(color));
}
