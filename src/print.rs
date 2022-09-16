pub fn main() {
    println!("Hewwo UwU");
    println!("I am the print.rs file...");
    // Basic formatting
    println!("{} is from {}", "Alexandria", "The Internet");
    // Positional args
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Alexandria", "The Internet", "code"
    );
    // Named args
    println!(
        "{name} likes to {activity}",
        name = "Alexandria",
        activity = "program"
    );

    // Placeholder traits
    println!("Binary: {:b}\nHex: {:x}\nOctal {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (3.14159, false, "gibberish string"));
}
