pub fn main() {
    greetings("Hi", "Alex");
    let num1 = 5;
    let num2 = 16;
    let result = add(num1, num2);
    println!("Sum: {}", result);

    // In-place function definition called a 'closure'
    // Sort of like a lambda
    let n3 = 3;
    let divide_plus_3 = |l: i32, r: i32| l / r + n3; // Note that local variables can be accessed from this scope
    let divide_result = divide_plus_3(100, 4);
    println!("{}", divide_result);
    // Output: 28
}

fn greetings(greet: &str, name: &str) {
    println!("{}, {}! How are you?", greet, name);
}

// When you don't use a semicolon, it tells Rust that is this a return.
// I think it's intended to make lambda-style coding easier and more unified.

fn add(l: i32, r: i32) -> i32 {
    l + r
}

// This also works, and I think is a little more readable for conventional usage
// fn sub(l: i32, r: i32) -> i32 {
//     return l + r;
// }
