// Tuple struct. Kinda weird. I don't really get the point
struct Color(u8, u8, u8);

pub fn main() {
    let mut bright_red = Color(255, 0, 0);
    // Tuples can be easily mutated, of course
    bright_red.1 = 10;
    bright_red.2 = 10;
    println!(
        "Our shade of red contains: {}, {}, {}",
        bright_red.0, bright_red.1, bright_red.2
    );
}
