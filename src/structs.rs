struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn main() {
    let mut bright_red = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    // Adjustment
    bright_red.green = 5;
    bright_red.blue = 5;

    println!(
        "Our shade of red contains: {}, {}, {}",
        bright_red.red, bright_red.green, bright_red.blue
    );
}
