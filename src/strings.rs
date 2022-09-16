/* Rust has multiple types of string, which was really confusing to me at first.
   - `str`: primitive string type -- the objects themselves are immutable and fixed-length,
       but mutable references (&str) that point at them will still be happy to be reassigned.
       `str` lives in the memory stack, and functions as a hardcoded binary data asset
   - `String`: Growable data structure stored in the heap, allocated on the fly.
*/

pub fn main() {
    // TIME TO PLAY WITH: `str`
    // Here, you can see an immutable string reference. If you were to try to do anything to this, you'd get an error
    let hello_world = "Hello world!";
    // Note that this is technically an immutable reference to an immutable string. We can make the reference mutable, but not the str itself...
    println!("{}", hello_world);

    // A mutable string.
    let mut test = "I'm not mutable. You can't edit me! I live in your stack. I am immortal :D";
    println!("{}", test);
    test = "Nooo! My reference has been overwritten! D: I am defeated Y~Y";
    println!("{}", test);

    // Since string literals give us a string reference (&str)
    // assigning a dereferenced string literal gives us a `str`
    // let x = *"test";
    // This gives us a "the size for values of type `str` cannot be known at compilation time"
    // Probably trying to create it in the heap rather than the stack. Odd. This is less about strings,
    // And more about how ref and deref work in Rust. Will explore more later...

    ///////////////////////////////////////////////
    // TIME TO PLAY WITH: `String`

    let mut greeting_string = String::from("I'm mutable! Hewwo");

    // Getting capacity of string
    println!("Capacity in bytes: {}", greeting_string.capacity());
    // Example of accessing length
    println!(
        "My greeting's length is: {length}.",
        length = greeting_string.len()
    );
    // Capacity and length are both 18.
    // As expected, String allocates just enough memory to fit our starting `str`

    // Let's mess with the string, making runtime edits...
    // allows us to append single char
    greeting_string.push(' ');
    // allows us to append full string
    greeting_string.push_str("world");

    // Let's check if capacity has changed...
    println!("Capacity in bytes: {}", greeting_string.capacity());
    println!(
        "My greeting's length is: {length}.",
        length = greeting_string.len()
    );
    // Note that, on my machine, this returns 36 even though the string is only 24 in length.
    // My guess is that, because memory allocation ops are costly,
    // `String` is trying to minimize the number of times it has to do it.
    // We are increasing capacity by a certain chunk size each time.

    // Note: I haven't looked any of this up -- I'm just playing around and learning as I go, here.

    println!("{}", greeting_string);

    /* Output:
        I'm mutable! Hewwo world
    */

    ///////////////////////////////////////////////

    // Declaring string with specific capacity from the getgo
    let mut s = String::with_capacity(10);
    s.push_str("uwu");
    println!("{}", s.capacity());
    // Output: 10
    // Nothing more is allocated because we have enough memory already

    ///////////////////////////////////////////////

    // example of checking for substring
    println!(
        "Does `greeting_string` contain `world` substr?: {}",
        greeting_string.contains("world")
    );
    /* Output:
       Does `greeting_string` contain `world` substr?: true
    */

    ///////////////////////////////////////////////

    // Note that this method does not operate in-place
    greeting_string = greeting_string.replace("Hewwo", "UwU");
    println!("{}", greeting_string);
    /* Output:
       I'm mutable! UwU world
    */

    ///////////////////////////////////////////////

    // Example of checking if string is empty
    println!("{}", greeting_string.is_empty());

    ///////////////////////////////////////////////

    // Let's look at splitting...
    // The for loops in Rust are so Python-like that I accidentally reverted to colon-based indents while typing this...
    for word in greeting_string.split_whitespace() {
        println!("Word: {}", word);
    }
    /* Output:
       Word: I'm
       Word: mutable!
       Word: UwU
       Word: world
    */
}
