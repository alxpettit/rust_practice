use std::io;
use std::io::Write;
use text_io::read;

pub fn main() {
    //////////////////////////////////////////////////////////////////

    // Conditionals work exactly like you'd expect, with `&&` for `and`, and `||` for `or`...

    print!("Enter your body temperature: ");
    io::stdout().flush().ok();
    let temp: f64 = read!("{}\n");

    println!("Your temperature is: {}", temp);
    if temp >= 36.1 && temp <= 36.6 {
        println!("Your body temperature is normal.");
    } else if temp >= 37.0 && temp <= 37.2 {
        println!("Your body temperature is probably normal, although a little surprising.");
    } else if temp > 39.4 {
        println!("That sounds like a fever. You should seek medical attention.")
    } else {
        println!("I don't know what to say about that...")
    }

    // Note: no ternary operator in Rust that I know of
    // Instead, you use similar syntax to Python, except with curly brackets
    let final_statement: &str = if temp <= 30.0 {
        "Wow... that seems really cold..."
    } else {
        ""
    };

    println!("{}", final_statement);

    //////////////////////////////////////////////////////////////////
}
