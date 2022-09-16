/* # Primitive types:
    - Integers: u8, i8, i16, i16, u32, i32, u64, i64, u128, i128
        - u = unsigned, i = signed
        - (unsigned does not have a negative number column,
            but it doubles the maximum representable value)
        - number is the amount of bits in memory
        - i32 is most common
    - Floats: f32, f64
    - Boolean: bool
    - Character: char
    - Tuple: ()
    - Array: []
        - Are fixed length, unlike C++
        - If you need something you can allocate dynamically, you use vectors

    Note: Rust is a statically typed language, which means that it must know all types of all
        variables at compile time. However, the compile can usually infer what type we want to use,
        based on the value and how we use it.

        It actually really reminds me of my imagination when I switched from C++ to Python and back.
        Explicit typing seems needlessly verbose, but dynamic typing leaves so many holes for runtime errors.
        With implicit static typing, we get the best of both worlds!
 */

pub fn main() {
    // Note: underscores prevent compiler from complaining about unused variables.

    // Defaults to i32, as Idea with the Rust plugin shows us
    let x = 1;

    // Defaults to f64
    let y = 2.5;

    // Example of overriding type
    // It's just the same as Python's typehinting syntax :3
    let pi: i64 = 3141;

    // Example of querying maximum size for a given type
    println!("Max i128: {size}", size=i128::MAX);

    // Bool example
    let state = true;

    // bool expression
    let is_greater = 10 < 5;

    // Char example
    let a1 = 'a';

    // Note: char supports unicode
    let middle_finger = '🖕';

    // And can even be encoded in basic ASCII as a codepoint, like so:

    let lol = '\u{1F602}';

    println!("{:?}", (x, y, pi, state, is_greater, a1, middle_finger, lol));

}